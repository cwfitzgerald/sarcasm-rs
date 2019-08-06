use crate::StartingCase;
use itertools::Itertools;
use log::{debug, info, trace};

#[inline]
fn different_case((l, r): (char, char)) -> bool {
    let result = if l.is_lowercase() {
        r.is_uppercase()
    } else {
        r.is_lowercase()
    };
    trace!(
        "Characters {} and {} are {}",
        l,
        r,
        if result { "different cases" } else { "the same case" }
    );

    result
}

/// Value returned by [`is_sarcasm`] determining if the input is SaRcAsM.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IsSarcasm {
    /// Input text is SaRcAsM. Contains the case of the first letter of the SaRcAsM text.
    Yes(StartingCase),
    /// Input text is either normal text or malformed SaRcAsM text.
    No,
    /// Input is either empty or contains only non-alphanumeric characters.
    TooShort,
}

/// Determines if input is SaRcAsM and what the case of the first letter is.
///
/// All non-alphanumeric characters will be ignored in the input. As a result,
/// the SaRcAsM pattern only has to hold within groups of alphanumeric characters.
/// This means the starting letter of any "secondary" alphanumeric groups can be any case.
///
/// While the encoder is well defined in the capitalization throughout the whole string,
/// the decoder needs to be able to deal with many possible styles of writing SaRcAsM text.
///
/// # Edge Cases
///
/// - `AbC DeF` and `AbC dEf` -> SaRcAsM.
/// - `Ab-Cd` and `Ab-cD` -> SaRcAsM.
/// - `A` -> SaRcAsM.
/// - `!!` -> Too Short (no alphanumeric characters to judge).
/// - `A!A` and `A!a` -> SaRcAsM.
///
/// # Arguments
///
/// - `input` - String slice to check for SaRcAsM.
///
/// # Return
///
/// [`IsSarcasm`] dictating if the input text was SaRcAsM.
///
/// # Examples
///
/// ```edition2018
/// # use assert_matches::assert_matches;
/// # use sarcasm_utils::{is_sarcasm, IsSarcasm, StartingCase};
/// assert_matches!(is_sarcasm("AbC"), IsSarcasm::Yes(StartingCase::Uppercase));
/// assert_matches!(is_sarcasm("aBc"), IsSarcasm::Yes(StartingCase::Lowercase));
///
/// assert_matches!(is_sarcasm("Abc"), IsSarcasm::No);
/// assert_matches!(is_sarcasm("aBC"), IsSarcasm::No);
///
/// assert_matches!(is_sarcasm(""), IsSarcasm::TooShort);
/// assert_matches!(is_sarcasm("!!"), IsSarcasm::TooShort);
/// ```
pub fn is_sarcasm(input: &str) -> IsSarcasm {
    info!("Checking sarcasm on {} bytes", input.len());
    trace!("Checking sarcasm on input: {}", input);

    let mut iter = input.chars().skip_while(|c| !c.is_alphabetic()).peekable();
    match iter.peek() {
        Some(&c) => {
            trace!("Iterator has at least one alphanumeric character");
            let grouped = iter.group_by(|c| c.is_alphabetic());
            let grouped_filtered = grouped.into_iter().filter(|(key, _group)| *key).map(|(_, group)| group);
            let sarcasm: bool = grouped_filtered
                .map(|g| g.tuple_windows().all(different_case))
                .enumerate()
                .all(|(i, b)| {
                    if b {
                        debug!("Alphanumeric group {} is sarcasm", i);
                    } else {
                        debug!("Alphanumeric group {} is not sarcasm", i);
                    }
                    b
                });

            if sarcasm {
                info!("Input is sarcasm.");
                IsSarcasm::Yes(StartingCase::from(c))
            } else {
                info!("Input is not sarcasm.");
                IsSarcasm::No
            }
        }
        None => {
            if input.is_empty() {
                info!("Cannot determine sarcasm: input string empty");
            } else {
                info!("Cannot determine sarcasm: input only contains non-alphanumeric characters");
            }
            IsSarcasm::TooShort
        }
    }
}
