//! Module for the pswd CLI struct.
use crate::utils;
use clap::Parser;
use rand::Rng;

const ASCII_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ASCII_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ASCII_NUMBERS: &str = "0123456789";
const ASCII_SYMBOLS: &str = "!@#$%^&*()_+-=";

/// Generate a password and insert it into your clipboard
#[derive(Copy, Clone, Parser)]
#[command(author, version, about)]
pub struct Pswd {
    /// The required length of the password
    #[arg()]
    length: usize,

    /// Use all ASCII charaters
    #[arg(short, long)]
    all: bool,

    /// Use uppercase ASCII letters
    #[arg(short, long, conflicts_with = "all")]
    upper: bool,

    /// Use lowercase ASCII letters
    #[arg(short, long, conflicts_with = "all")]
    lower: bool,

    /// Use ASCII numbers
    #[arg(short, long, conflicts_with = "all")]
    numbers: bool,

    /// Use ASCII symbol characters
    #[arg(short, long, conflicts_with = "all")]
    symbols: bool,

    /// Print the password out once generated
    #[arg(short, long, default_value = "false")]
    pub display: bool,
}

impl Pswd {
    /// Set the all flag to true if no flags have been provided.
    pub fn validate(&mut self) -> Self {
        if !self.all && !self.upper && !self.lower && !self.numbers && !self.symbols {
            self.all = true;
        }
        *self
    }
}

/// Return a Vec of all usable characters as specified.
fn get_charset(pswd: &Pswd) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();

    if pswd.all || pswd.upper {
        chars.extend(utils::to_vec_char(ASCII_UPPER));
    }

    if pswd.all || pswd.lower {
        chars.extend(utils::to_vec_char(ASCII_LOWER));
    }

    if pswd.all || pswd.numbers {
        chars.extend(utils::to_vec_char(ASCII_NUMBERS));
    }

    if pswd.all || pswd.symbols {
        chars.extend(utils::to_vec_char(ASCII_SYMBOLS));
    }

    chars
}

/// Generate a password.
pub fn generate_pass(pswd: &Pswd) -> String {
    let mut password = String::new();
    let mut rng = rand::thread_rng();

    let chars = get_charset(pswd);

    for _ in 0..pswd.length {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx]);
    }

    password
}

#[cfg(test)]
pub mod tests {
    use crate::utils::to_vec_char;

    use super::{generate_pass, get_charset, Pswd, ASCII_LOWER, ASCII_NUMBERS, ASCII_UPPER};

    /// Test that the generate_pass method creates a password of the correct lenght.
    #[test]
    fn test_generate_pass_has_correct_length_password() {
        let pswd = Pswd {
            length: 8,
            all: true,
            display: false,
            lower: false,
            upper: false,
            numbers: false,
            symbols: false,
        };

        let pass = generate_pass(&pswd);

        assert_eq!(pass.len(), pswd.length)
    }

    /// Test that the get_charset method gets the correct characters.
    #[test]
    fn test_get_charset_returns_correct_chars() {
        let pswd = Pswd {
            length: 8,
            all: false,
            display: false,
            lower: true,
            upper: true,
            symbols: false,
            numbers: true,
        };

        let chars = get_charset(&pswd);
        let mut expected = to_vec_char(ASCII_UPPER);
        expected.extend(to_vec_char(ASCII_LOWER));
        expected.extend(to_vec_char(ASCII_NUMBERS));

        assert_eq!(chars, expected)
    }
}
