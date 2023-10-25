use rustc_middle::mir;
use rustc_target::abi::Size;
use rustc_target::spec::abi::Abi;

use log::trace;

use crate::helpers::check_arg_count;
use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum Dlsym {
    NtWriteFile,
}

impl Dlsym {
    // Returns an error for unsupported symbols, and None if this symbol
    // should become a NULL pointer (pretend it does not exist).
    pub fn from_str<'tcx>(name: &str) -> InterpResult<'tcx, Option<Dlsym>> {
        Ok(match name {
            "GetSystemTimePreciseAsFileTime" => None,
            "SetThreadDescription" => None,
            "NtWriteFile" => Some(Dlsym::NtWriteFile),
            _ => throw_unsup_format!("unsupported Windows dlsym: {}", name),
        })
    }
}

impl<'mir, 'tcx: 'mir> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
pub trait EvalContextExt<'mir, 'tcx: 'mir>: crate::MiriEvalContextExt<'mir, 'tcx> {
    fn call_dlsym(
        &mut self,
        dlsym: Dlsym,
        abi: Abi,
        args: &[OpTy<'tcx, Tag>],
        dest: &PlaceTy<'tcx, Tag>,
        ret: Option<mir::BasicBlock>,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        let ret = ret.expect("we don't support any diverging dlsym");
        assert!(this.tcx.sess.target.os == "windows");

        this.check_abi(abi, Abi::System { unwind: false })?;

        match dlsym {
            Dlsym::NtWriteFile => {
                if !this.frame_in_std() {
                    throw_unsup_format!(
                        "NtWriteFile support is crude and just enough for stdout to work"
                    );
                }

                let [
                    handle,
                    _event,
                    _apc_routine,
                    _apc_context,
                    io_status_block,
                    buf,
                    n,
                    byte_offset,
                    _key,
                ] = check_arg_count(args)?;
                let handle = this.read_scalar(handle)?.to_machine_isize(this)?;
                let buf = this.read_pointer(buf)?;
                let n = this.read_scalar(n)?.to_u32()?;
                let byte_offset = this.read_scalar(byte_offset)?.to_machine_usize(this)?; // is actually a pointer
                let io_status_block = this.deref_operand(io_status_block)?;

                if byte_offset != 0 {
                    throw_unsup_format!(
                        "NtWriteFile ByteOffset paremeter is non-null, which is unsupported"
                    );
                }

                let written = if handle == -11 || handle == -12 {
                    // stdout/stderr
                    use std::io::{self, Write};

                    let buf_cont = this.read_bytes_ptr(buf, Size::from_bytes(u64::from(n)))?;
                    let res = if this.machine.mute_stdout_stderr {
                        Ok(buf_cont.len())
                    } else if handle == -11 {
                        io::stdout().write(buf_cont)
                    } else {
                        io::stderr().write(buf_cont)
                    };
                    res.ok().map(|n| n as u32)
                } else {
                    throw_unsup_format!(
                        "on Windows, writing to anything except stdout/stderr is not supported"
                    )
                };
                // We have to put the result into io_status_block.
                if let Some(n) = written {
                    let io_status_information =
                        this.mplace_field_named(&io_status_block, "Information")?;
                    this.write_scalar(
                        Scalar::from_machine_usize(n.into(), this),
                        &io_status_information.into(),
                    )?;
                }
                // Return whether this was a success. >= 0 is success.
                // For the error code we arbitrarily pick 0xC0000185, STATUS_IO_DEVICE_ERROR.
                this.write_scalar(
                    Scalar::from_i32(if written.is_some() { 0 } else { 0xC0000185u32 as i32 }),
                    dest,
                )?;
            }
        }

        trace!("{:?}", this.dump_place(**dest));
        this.go_to_block(ret);
        Ok(())
    }
}
