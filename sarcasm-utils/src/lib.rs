//! SaRcAsM tExT creation and validation library.
//!
//! This library provides utilities for creating and validating the capitalization style known as
//! SaRcAsM tExT. It is the library behind the command line utility `sarcasm` which provides a
//! convenient interface for day-to-day use, including automatic copying to the clipboard.
//!
//! All steps have been taken to make the library production ready. It has full logging via the
//! `log` crate and code should be panic free unless otherwise noted. If a panic is found in a
//! non-labeled function, this is a bug.
//!
//! Feature requests, pull requests, and issues are all welcome.
//!
//! # Example
//!
//! For more details, see [`encode_sarcasm`] and [`is_sarcasm`].
//!
//! ```edition2018
//! use sarcasm_utils::{StartingCase, is_sarcasm, IsSarcasm};
//! use sarcasm_utils::encode_sarcasm;
//!
//! // Encoding SaRcAsM
//! let encoded: String = encode_sarcasm("Hello!", StartingCase::Uppercase);
//! assert_eq!(encoded, "HeLlO!");
//!
//! // Decoding SaRcAsM
//! let decoded: IsSarcasm = is_sarcasm(&encoded);
//! assert_eq!(decoded, IsSarcasm::Yes(StartingCase::Uppercase));
//!
//! // Matching on the type of SaRcAsM
//! match decoded {
//!     IsSarcasm::Yes(StartingCase::Uppercase) => println!("UpPeRcAsE sArCaSm!"),
//!     IsSarcasm::Yes(StartingCase::Lowercase) => println!("lOwErCaSe SaRcAsM!"),
//!     IsSarcasm::No => println!("No sarcasm here!"),
//!     IsSarcasm::TooShort => println!("Can't tell if there's sarcasm!"),
//! }
//! ```
//!
//! # Why?
//!
//! Because.
//!
//! # Is this a joke?
//!
//! No.
//!
//! # Really?
//!
//! Okay, yes the purpose is silly, but this was an exersize in writing a library in rust
//! that is designed to be robust yet easy to use. Also, it does provide some (minimal) use for
//! someone who wants to do this kind of thing. I will do my best to maintain and attend to this
//! library, as well as expand it with new "useful" features in the future.
//!
//! In short: maybe?

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

/// The case of the first letter in SaRcAsM tExT.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StartingCase {
    /// First letter is lOwErCaSe
    Lowercase,
    /// First letter is UpPeRcAsE
    Uppercase,
}

#[doc(hidden)]
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
    fn encode_nothing() {
        assert_eq!(crate::encode_sarcasm("", StartingCase::Uppercase), "");
        assert_eq!(crate::encode_sarcasm("", StartingCase::Lowercase), "");
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
