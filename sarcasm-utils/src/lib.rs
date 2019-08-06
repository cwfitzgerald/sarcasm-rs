#![deny(nonstandard_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(unused)]

mod decode;
mod encode;

pub use decode::*;
pub use encode::*;
use log::{debug, error, trace};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StartingCase {
    Lowercase,
    Uppercase,
}

impl From<char> for StartingCase {
    fn from(c: char) -> Self {
        trace!("Analyzing case of leading char {}", c);
        if c.is_lowercase() {
            debug!("Leading character {} is lowercase", c);
            StartingCase::Lowercase
        } else if c.is_uppercase() {
            debug!("Leading character {} is uppercase", c);
            StartingCase::Uppercase
        } else {
            let msg = format!("Invalid char {} provided to From<char> for StartingCase", c);
            error!("{}", msg);
            StartingCase::Uppercase
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IsSarcasm, StartingCase};

    #[test]
    fn encode() {
        assert_eq!(crate::encode_sarcasm("hello!", StartingCase::Uppercase), "HeLlO!");
        assert_eq!(crate::encode_sarcasm("hello!", StartingCase::Lowercase), "hElLo!");
    }

    #[test]
    fn multi_word_encode() {
        assert_eq!(
            crate::encode_sarcasm("Hello World!", StartingCase::Uppercase),
            "HeLlO WoRlD!"
        );
        assert_eq!(
            crate::encode_sarcasm("Hello World!", StartingCase::Lowercase),
            "hElLo wOrLd!"
        );
    }

    #[test]
    fn decode() {
        assert_eq!(crate::is_sarcasm("HeLlO!"), IsSarcasm::Yes(StartingCase::Uppercase));
        assert_eq!(crate::is_sarcasm("hElLo!"), IsSarcasm::Yes(StartingCase::Lowercase));
    }

    #[test]
    fn multi_word_decode() {
        assert_eq!(
            crate::is_sarcasm("HeLlO! hElLo!"),
            IsSarcasm::Yes(StartingCase::Uppercase)
        );
        assert_eq!(
            crate::is_sarcasm("hElLo! HeLlO!"),
            IsSarcasm::Yes(StartingCase::Lowercase)
        );

        assert_eq!(
            crate::is_sarcasm("HeLlO! HeLlO!"),
            IsSarcasm::Yes(StartingCase::Uppercase)
        );
        assert_eq!(
            crate::is_sarcasm("hElLo! hElLo!"),
            IsSarcasm::Yes(StartingCase::Lowercase)
        );
    }

    #[test]
    fn empty_decode() {
        assert_eq!(crate::is_sarcasm(""), IsSarcasm::TooShort);
    }

    #[test]
    fn negatives_decode() {
        assert_eq!(crate::is_sarcasm("Not Sarcasm"), IsSarcasm::No);
        assert_eq!(crate::is_sarcasm("NoT SaRcASM"), IsSarcasm::No);
    }

    #[test]
    fn first_character_decode() {
        assert_eq!(crate::is_sarcasm("heLlO!"), IsSarcasm::No);
        assert_eq!(crate::is_sarcasm("HElLo!"), IsSarcasm::No);
    }
}
