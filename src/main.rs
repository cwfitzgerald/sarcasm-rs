#![deny(nonstandard_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(unused)]

use clipboard::{ClipboardContext, ClipboardProvider};
use log::{error, info, warn};
use sarcasm_utils::{encode_sarcasm, is_sarcasm, IsSarcasm, StartingCase};
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sarcasm")]
/// A tool to generate and validate SaRcAsM TeXt.
///
/// While the program itself is a joke, it is designed to be a fully robust program and library that you can use
/// anywhere in real projects, if you had any actual need.
struct Options {
    /// Output uppercase SaRcAsM TeXt
    #[structopt(short, long, group = "case")]
    uppercase: bool,

    /// Output lowercase sArCaSm tExT
    #[structopt(short, long, group = "case")]
    lowercase: bool,

    /// Check input text for sarcasm type
    #[structopt(short, long, group = "case", alias = "validate")]
    check: bool,

    /// Force clipboard use
    #[structopt(long)]
    clipboard: bool,

    /// Log output level (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Text to process
    text: Vec<String>,
}

fn check(full_text: &str) -> i32 {
    match is_sarcasm(full_text) {
        IsSarcasm::Yes(StartingCase::Uppercase) => {
            println!("Yes, this is UpPeRcAsE sArCaSm!");
            0
        }
        IsSarcasm::Yes(StartingCase::Lowercase) => {
            println!("Yes, this is lOwErCaSe SaRcAsM!");
            0
        }
        IsSarcasm::No => {
            println!("No, this is not sarcasm!");
            1
        }
        IsSarcasm::TooShort => {
            println!("I can't tell if this is sarcasm!");
            2
        }
    }
}

fn create(opt: &Options, full_text: &str) -> i32 {
    if full_text.is_empty() {
        error!("Nothing to sarcasm-ise!");
        return 1;
    }

    let case = if opt.lowercase {
        StartingCase::Lowercase
    } else {
        StartingCase::Uppercase
    };
    let result = encode_sarcasm(&full_text, case);
    println!("{}", result);

    let provider: Result<ClipboardContext, _> = ClipboardProvider::new();
    match provider {
        Ok(mut clipboard) => match clipboard.set_contents(result) {
            Ok(()) => {}
            Err(error) => {
                let message = format!("Error setting clipboard contents. Error: {:#?}", error);
                if opt.clipboard {
                    warn!("{}", message);
                } else {
                    info!("{}", message);
                }
            }
        },
        Err(error) => {
            let message = format!("Cannot create clipboard context. Error: {:#?}", error);
            if opt.clipboard {
                warn!("{}", message);
            } else {
                info!("{}", message);
            }
        }
    }

    0
}

fn main() {
    let opt: Options = Options::from_args();
    let full_text = opt.text.join(" ");
    process::exit(if opt.check {
        check(&full_text)
    } else {
        create(&opt, &full_text)
    })
}
