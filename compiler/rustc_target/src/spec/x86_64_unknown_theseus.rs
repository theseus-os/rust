use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let options = TargetOptions {
        os: "theseus".into(),
        features: "-mmx,-sse,+soft-float".into(),
        has_thread_local: true,
        disable_redzone: true,
        ..Default::default()
    };

    Target {
        llvm_target: "x86_64-unknown-theseus".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options,
    }
}
