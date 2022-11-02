use crate::spec::{RelroLevel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "cygwin".to_string(),
        env: "gnu".to_string(),
        vendor: "pc".to_string(),
        function_sections: false,
        linker: Some("gcc".to_string()),
        dynamic_linking: true,
        executables: true,
        dll_prefix: "cyg".to_string(),
        dll_suffix: ".dll".to_string(),
        exe_suffix: ".exe".to_string(),
        families: vec!["unix".to_string()],
        allows_weak_linkage: false,
        crt_static_respected: true,
        ..Default::default()
    }
}
