//! Command line wrapper for sarcasm-utils

#![deny(clippy::cargo_common_metadata)]
#![deny(clippy::wildcard_dependencies)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(unused)]

mod check;
mod clipboard;
mod create;

use crate::{check::*, create::*};

use chrono::SecondsFormat;
use log::{debug, info, trace, warn, Level, LevelFilter};
use std::{path::PathBuf, process};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sarcasm")]
/// A tool to generate and validate SaRcAsM TeXt.
///
/// While the program itself is a bit pointless, it is designed to be a fully robust program
/// and library that you can use anywhere in real projects, if you had any actual need.
pub struct Options {
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

    /// Full verbosity log file. Appends to end.
    /// Always sends warnings and errors to stderr.
    #[structopt(long = "log", parse(from_os_str))]
    log_file: Option<PathBuf>,

    /// Output to a file instead of standard out.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Text to process
    text: Vec<String>,
}

fn setup_logger(level: u8, log_file: &Option<PathBuf>) -> Result<(), fern::InitError> {
    let level = match level {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let stderr = fern::Dispatch::new().level(LevelFilter::Warn).chain(std::io::stderr());

    match log_file {
        Some(file) => {
            let file = fern::log_file(file)?;

            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .chain(file)
                .chain(stderr)
                .apply()?;
        }
        None => {
            let stdout = fern::Dispatch::new()
                .filter(|m| m.level() > Level::Warn)
                .level(LevelFilter::Trace)
                .chain(std::io::stdout());

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
        }
    }

    Ok(())
}

fn main() {
    let opt: Options = Options::from_args();
    if let Err(err) = setup_logger(opt.verbose, &opt.log_file) {
        eprintln!("Error initializing logger. {:?}", err);
        process::exit(1);
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
