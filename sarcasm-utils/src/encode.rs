use crate::StartingCase;
use itertools::Itertools;
use log::{debug, info, trace};

#[inline]
fn encode_lower(input: &str) -> String {
    let first = input.chars().step_by(2).flat_map(|c| c.to_lowercase());
    let second = input.chars().skip(1).step_by(2).flat_map(|c| c.to_uppercase());

    first.interleave(second).collect()
}

#[inline]
fn encode_upper(input: &str) -> String {
    let first = input.chars().step_by(2).flat_map(|c| c.to_uppercase());
    let second = input.chars().skip(1).step_by(2).flat_map(|c| c.to_lowercase());

    first.interleave(second).collect()
}

/// Transform normal text into SaRcAsM text.
///
/// # Arguments
///
/// - `input` - Input string to be transformed.
/// - `start` - The case of the first letter in the SaRcAsM text.
///
/// # Returns
///
/// Fully transformed string.
///
/// # Examples
///
/// ```edition2018
/// # use sarcasm_utils::{encode_sarcasm, StartingCase};
/// assert_eq!(encode_sarcasm("Hello World!", StartingCase::Uppercase), "HeLlO WoRlD!");
/// assert_eq!(encode_sarcasm("Hello World!", StartingCase::Lowercase), "hElLo wOrLd!");
/// ```
pub fn encode_sarcasm(input: &str, start: StartingCase) -> String {
    info!("Encoding {} bytes to sarcasm", input.len());
    trace!("Encoding input: {}", input);

    let result = match start {
        StartingCase::Lowercase => {
            debug!("Encoding as lOwErCaSe SaRcAsM tExT");
            encode_lower(input)
        }
        StartingCase::Uppercase => {
            debug!("Encoding as lOwErCaSe SaRcAsM tExT");
            encode_upper(input)
        }
    };

    info!("Encoded {} bytes successfully", result.len());
    trace!("Encoding output: {}", result);

    result
}
