use crate::spec::{Target, TargetOptions};

use super::SanitizerSet;

pub fn target() -> Target {
    // LLVM 15 doesn't support OpenHarmony yet, use a linux target instead.
    let mut base = super::linux_musl_base::opts();
    base.force_emulated_tls = true;
    base.has_thread_local = false;
    base.env =  "ohos".into();
    Target {
        llvm_target: "loongarch64-unknown-linux-musl".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n64-S128".into(),
        arch: "loongarch64".into(),
        options: TargetOptions {
            cpu: "generic".into(),
            features: "+f,+d".into(),
            llvm_abiname: "lp64d".into(),
            max_atomic_width: Some(64),
            crt_static_default: false,
            supported_sanitizers: SanitizerSet::ADDRESS
                | SanitizerSet::LEAK
                | SanitizerSet::MEMORY
                | SanitizerSet::MEMTAG
                | SanitizerSet::THREAD
                | SanitizerSet::HWADDRESS,
            ..base
        },
    }
}
