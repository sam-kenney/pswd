//! Entry point for pswd.
mod cli;
mod utils;
use crate::cli::Pswd;
use clap::Parser;

/// Generate a random password of a specified length using the
/// requested character set, and optionally print to the terminal.
fn main() {
    let password = Pswd::parse().validate().generate_in_clipboard();

    if let Some(pass) = password {
        println!("{pass}")
    }
}
