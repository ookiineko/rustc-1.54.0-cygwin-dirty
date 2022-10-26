use crate::spec::{RelroLevel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "cygwin".to_string(),
        env: "newlib".to_string(),
        vendor: "pc".to_string(),
        dynamic_linking: true,
        executables: true,
        dll_prefix: String::new(),
        dll_suffix: ".dll".to_string(),
        exe_suffix: ".exe".to_string(),
        families: vec!["unix".to_string()],
        has_rpath: true,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        crt_static_respected: true,
        ..Default::default()
    }
}
