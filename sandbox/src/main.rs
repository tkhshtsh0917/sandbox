use std::process;

#[cfg(feature = "switch")]
use sandbox_macros::trace;

#[cfg(not(feature = "switch"))]
fn main() -> process::ExitCode {
    if cfg!(feature = "env-flag") {
        use std::env;
        const ENV_FLAG: &str = "ENV_FLAG";

        match env::var(ENV_FLAG) {
            Ok(value) => println!("{}", value),
            Err(error) => {
                eprintln!("{}: {}", error, ENV_FLAG);
                return process::ExitCode::FAILURE;
            }
        }
    } else {
        println!("Hello, world!");
    }

    process::ExitCode::SUCCESS
}

#[cfg(feature = "switch")]
#[trace]
fn main() -> process::ExitCode {
    println!("Feature switch!");
    process::ExitCode::SUCCESS
}
