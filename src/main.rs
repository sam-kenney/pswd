//! Entry point for pswd.
mod cli;
mod utils;
use crate::cli::Pswd;
use clap::Parser;

/// Generate a random password of a specified length using the
/// requested character set, and optionally print to the terminal.
fn main() {
    let pswd = Pswd::parse().validate();
    let pass = cli::generate_pass(&pswd);
    utils::to_clipboard(&pass);

    if pswd.display {
        println!("{pass}")
    }
}
