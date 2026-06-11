use std::fmt;

#[derive(Debug)]
pub enum AudioProcessingError {
    InvalidFile(String),
    CannotProcessAudio(String),
}

impl fmt::Display for AudioProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioProcessingError::InvalidFile(reason) => {
                write!(f, "Invalid file error: {reason}")
            }
            AudioProcessingError::CannotProcessAudio(reason) => {
                write!(f, "Audio processing failed: {reason}")
            }
        }
    }
}