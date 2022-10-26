mod flags {
    #![allow(unused)]

    xflags::xflags! {
        src "./examples/hello-generated.rs"

        /// Prints a greeting.
        cmd hello
            /// Whom to greet.
            required name: String
        {
            /// Use non-ascii symbols in the output.
            optional -e, --emoji
        }
    }

    // generated start
    // The following code is generated by `xflags` macro.
    // Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
    #[derive(Debug)]
    pub struct Hello {
        pub name: String,

        pub emoji: bool,
    }

    impl Hello {
        pub const HELP: &'static str = Self::HELP_;

        pub fn from_env() -> xflags::Result<Self> {
            Self::from_env_()
        }

        pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
            Self::from_vec_(args)
        }
    }
    // generated end
}

fn main() {
    match flags::Hello::from_env() {
        Ok(flags) => {
            let bang = if flags.emoji { "❣️" } else { "!" };
            println!("Hello {}{}", flags.name, bang);
        }
        Err(err) => {
            eprintln!("{}\n\n{}", err, flags::Hello::HELP);
            std::process::exit(1)
        }
    }
}
