use rustube::Error;
use thiserror::Error as terror;

// wrap the rustube errors and errors writing to files, so we can return and send to the gui feedback box
#[derive(Debug, terror)]
pub enum Errors {
    #[error("Rustube error: {0}")]
    RustubeError(#[from] Error),
    #[error("OS Error:")]
    OSError // we need some error type here ..? like when changing a filename fails
}