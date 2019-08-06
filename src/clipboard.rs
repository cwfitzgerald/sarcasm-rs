use clipboard::{ClipboardContext, ClipboardProvider};
use log::{debug, info, warn};

pub fn copy(text: &str, force: bool) {
    debug!("Attempting to create clipboard context");
    let provider: Result<ClipboardContext, _> = ClipboardProvider::new();
    match provider {
        Ok(mut clipboard) => {
            debug!("Created clipboard context successfully");
            match clipboard.set_contents(text.to_owned()) {
                Ok(()) => {
                    info!("Copied {} bytes to clipboard", text.len());
                }
                Err(error) => {
                    let message = format!("Error setting clipboard contents. Error: {:#?}", error);
                    if force {
                        warn!("{}", message);
                    } else {
                        info!("{}", message);
                    }
                }
            }
        }
        Err(error) => {
            let message = format!("Cannot create clipboard context. Error: {:#?}", error);
            if force {
                warn!("{}", message);
            } else {
                info!("{}", message);
            }
        }
    }
}
