use log::info;
use sarcasm_utils::{is_sarcasm, IsSarcasm, StartingCase};

pub fn check(full_text: &str) -> i32 {
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
