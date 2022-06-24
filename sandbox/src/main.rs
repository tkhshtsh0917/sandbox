#[cfg(not(feature = "switch"))]
use std::{env, process};

const ENV_KEY: &str = "ENVIRONMENT_VALUE";

#[cfg(feature = "switch")]
use sandbox_macros::trace;

#[cfg(not(feature = "switch"))]
fn main() -> process::ExitCode {
    let env_value = match env::var(ENV_KEY) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}: {}", error, ENV_KEY);
            return process::ExitCode::FAILURE;
        }
    };

    println!("{}", env_value);
    println!("Hello, world!");

    process::ExitCode::SUCCESS
}

#[cfg(feature = "switch")]
#[trace]
fn main() {
    println!("Feature switch!");
}
