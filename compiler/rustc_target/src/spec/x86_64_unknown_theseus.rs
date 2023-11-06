use crate::spec::{Target, TargetOptions, MergeFunctions, RelocModel, TlsModel, CodeModel};

pub fn target() -> Target {
    let options = TargetOptions {
        code_model: Some(CodeModel::Large),
        disable_redzone: true,
        executables: false,
        features: "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float".into(),
        has_thread_local: true,
        merge_functions: MergeFunctions::Disabled,
        os: "theseus".into(),
        relocation_model: RelocModel::Static,
        // TODO: We don't need to set relro-level right?
        tls_model: TlsModel::LocalExec,
        ..Default::default()
    };

    Target {
        arch: "x86_64".into(),
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".into(),
        // TODO
        llvm_target: "x86_64-unknown-theseus".into(),
        pointer_width: 64,
        options,
    }
}