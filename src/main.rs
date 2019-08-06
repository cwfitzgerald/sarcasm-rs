#![deny(nonstandard_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(unused)]

use clipboard::{ClipboardContext, ClipboardProvider};
use log::{debug, error, info, trace, warn, Level, LevelFilter};
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

fn setup_logger(level: u8) -> Result<(), fern::InitError> {
    let level = match level {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let stdout = fern::Dispatch::new()
        .filter(|m| m.level() > Level::Warn)
        .level(LevelFilter::Trace)
        .chain(std::io::stdout());

    let stderr = fern::Dispatch::new().level(LevelFilter::Warn).chain(std::io::stderr());

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.6f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(stderr)
        .chain(stdout)
        .apply()?;

    Ok(())
}

fn check(full_text: &str) -> i32 {
    info!("Validating input text");

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
    info!("Creating SaRcAsM tExT from normal text");

    if full_text.is_empty() {
        error!("Nothing to sarcasm-ise!");
        return 1;
    }

    let case = if opt.lowercase {
        debug!("Creating lOwErCaSe SaRcAsM text");
        StartingCase::Lowercase
    } else {
        debug!("Creating UpPeRcAsE sArCaSm text");
        StartingCase::Uppercase
    };

    let result = encode_sarcasm(&full_text, case);
    let result_len = result.len();
    trace!("Output byte length is {}", result_len);
    trace!(
        "Output size is {:0<.3}x input",
        full_text.len() as f32 / result_len as f32
    );
    println!("{}", result);

    debug!("Attempting to create clipboard context");
    let provider: Result<ClipboardContext, _> = ClipboardProvider::new();
    match provider {
        Ok(mut clipboard) => {
            debug!("Created clipboard context successfully");
            match clipboard.set_contents(result) {
                Ok(()) => {
                    info!("Copied {} bytes to clipboard", result_len);
                }
                Err(error) => {
                    let message = format!("Error setting clipboard contents. Error: {:#?}", error);
                    if opt.clipboard {
                        warn!("{}", message);
                    } else {
                        info!("{}", message);
                    }
                }
            }
        }
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
    match setup_logger(opt.verbose) {
        Err(err) => {
            eprintln!("Error initializing logger. {:?}", err);
            process::exit(1);
        }
        _ => {}
    }

    info!("Verbosity level {}", opt.verbose);

    let full_text = opt.text.join(" ");

    debug!("Input arg count: {}", opt.text.len());
    debug!("Input byte length: {}", full_text.len());
    trace!("Inputs: {:#?}", opt.text);

    process::exit(if opt.check {
        check(&full_text)
    } else {
        create(&opt, &full_text)
    })
}
