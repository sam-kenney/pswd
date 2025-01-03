//! Module for the pswd CLI struct.
use crate::utils;
use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
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

    /// Use all ASCII characters
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
    print: bool,

    /// Show debug messages including clipboard errors
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

impl Pswd {
    /// Set the all flag to true if no flags have been provided.
    fn validate(&mut self) -> Self {
        if !self.all
            && !self.upper
            && !self.lower
            && !self.numbers
            && !self.symbols
        {
            self.all = true;
        }
        *self
    }

    /// Return a Vec of all usable characters as specified.
    fn get_charset(&self) -> Vec<char> {
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
    /// it has been omitted.
    pub fn generate_in_clipboard(&mut self) -> Option<String> {
        let pass = self.validate().generate();

        if let Err(e) = set_clipboard_contents(&pass) {
            if self.debug {
                eprintln!("{e}");
            }
        }

        match self.print {
            true => Some(pass),
            false => None,
        }
    }
}

/// Set the contents of the user's clipboard.
fn set_clipboard_contents(pass: &String) -> Result<(), String> {
    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().map_err(|_| {
            "Could not initialise clipboard. ".to_string() +
            "Try running using the -d flag and manually copying."
        })?;

    clipboard.set_contents(pass.to_owned()).map_err(|_| {
        "Could not copy password to clipboard. ".to_string() +
        "Try running with the -d flag and manually copying."
    })?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::utils::to_vec_char;

    use super::{Pswd, ASCII_LOWER, ASCII_NUMBERS, ASCII_UPPER};

    /// Test that the generate_pass method creates a password of the correct length.
    #[test]
    fn test_generate_pass_has_correct_length_password() {
        let pswd = Pswd {
            length: 8,
            all: true,
            print: false,
            lower: false,
            upper: false,
            numbers: false,
            symbols: false,
            debug: false,
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
            print: false,
            lower: true,
            upper: true,
            symbols: false,
            numbers: true,
            debug: false,
        };

        let chars = pswd.get_charset();
        let mut expected = to_vec_char(ASCII_UPPER);
        expected.extend(to_vec_char(ASCII_LOWER));
        expected.extend(to_vec_char(ASCII_NUMBERS));

        assert_eq!(chars, expected)
    }
}
