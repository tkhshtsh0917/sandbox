use std::process;

#[cfg(feature = "switch")]
use sandbox_macros::trace;

/// Default feature main.
#[cfg(not(feature = "switch"))]
#[forbid(unsafe_code)]
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

/// Switch feature main.
#[cfg(feature = "switch")]
#[forbid(unsafe_code)]
#[trace]
fn main() -> process::ExitCode {
    println!("Feature switch!");
    process::ExitCode::SUCCESS
}
