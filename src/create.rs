use crate::{clipboard::*, Options};
use log::{debug, error, info, trace};
use sarcasm_utils::{encode_sarcasm, StartingCase};

pub fn create(opt: &Options, full_text: &str) -> i32 {
    info!("Creating SaRcAsM tExT from normal text");

    if full_text.is_empty() {
        error!("Nothing to sarcasm-ise!");
        return 1;
    }

    let case = if opt.lowercase {
        debug!("Creating lOwErCaSe SaRcAsM tExT");
        StartingCase::Lowercase
    } else {
        debug!("Creating UpPeRcAsE sArCaSm tExT");
        StartingCase::Uppercase
    };

    let result = encode_sarcasm(&full_text, case);
    let result_len = result.len();
    trace!("Output byte length is {}", result_len);
    trace!(
        "Output size is {:0<.3}x input",
        full_text.len() as f32 / result_len as f32
    );

    copy(&result, opt.clipboard);

    trace!("Printing {} bytes and bailing", result_len);
    println!("{}", result);

    0
}
