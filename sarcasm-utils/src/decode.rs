use crate::StartingCase;
use itertools::Itertools;
use log::{debug, info, trace};
use std::convert::identity;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IsSarcasm {
    Yes(StartingCase),
    No,
    TooShort,
}

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

pub fn is_sarcasm(input: &str) -> IsSarcasm {
    info!("Checking sarcasm on {} bytes", input.len());
    trace!("Checking sarcasm on input: {}", input);

    let mut iter = input.chars().skip_while(|c| !c.is_alphabetic()).peekable();
    match iter.peek() {
        Some(&c) => {
            trace!("Iterator has at least one alphanumeric character");
            let grouped = iter.group_by(|c| c.is_alphabetic());
            let grouped_filtered = grouped.into_iter().filter(|(key, group)| *key == true).map(|(_, g)| g);
            let sarcasm: bool = grouped_filtered
                .map(|g| g.tuple_windows().map(different_case).all(identity))
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
