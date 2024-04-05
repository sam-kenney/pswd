//! Module for the pswd CLI struct.
use crate::utils;
use clap::Parser;
use rand::Rng;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

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

    /// Return a Vec of all usable characters as specified.
    pub fn get_charset(&self) -> Vec<char> {
        let mut chars: Vec<char> = Vec::new();

        if self.all || self.upper {
            chars.extend(utils::to_vec_char(ASCII_UPPER));
        }

        if self.all || self.lower {
            chars.extend(utils::to_vec_char(ASCII_LOWER));
        }

        if self.all || self.numbers {
            chars.extend(utils::to_vec_char(ASCII_NUMBERS));
        }

        if self.all || self.symbols {
            chars.extend(utils::to_vec_char(ASCII_SYMBOLS));
        }

        chars
    }

    /// Generate a password.
    fn generate(&self) -> String {
        let mut password = String::new();
        let mut rng = rand::thread_rng();

        let chars = self.get_charset();

        for _ in 0..self.length {
            let idx = rng.gen_range(0..chars.len());
            password.push(chars[idx]);
        }

        password
    }

    /// Generate a password and insert it into the user's clipboard.
    ///
    /// Returns the password if the display flag is set, or None if
    /// it has been ommited.
    pub fn generate_in_clipboard(&self) -> Option<String> {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

        let pass = self.generate();
        ctx.set_contents(pass.to_owned()).unwrap();

        match self.display {
            true => Some(pass),
            false => None,
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::utils::to_vec_char;

    use super::{Pswd, ASCII_LOWER, ASCII_NUMBERS, ASCII_UPPER};

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

        let pass = pswd.generate();

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

        let chars = pswd.get_charset();
        let mut expected = to_vec_char(ASCII_UPPER);
        expected.extend(to_vec_char(ASCII_LOWER));
        expected.extend(to_vec_char(ASCII_NUMBERS));

        assert_eq!(chars, expected)
    }
}
