//! This file contains the main functions that use StemSplitterCore to split a track
//! into its 4 main stems.
//!
//! https://github.com/gentij/stem-splitter-core

use std::fmt;
use stem_splitter_core::{split_file, SplitOptions, SplitResult};
use std::path::PathBuf;
use bevy::log::*;

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


pub struct AudioStems {
    pub vocals: PathBuf,
    pub bass: PathBuf,
    pub drums: PathBuf,
    pub other: PathBuf
}


pub fn split_audio_into_stems(audio_file: &PathBuf, output_dir: PathBuf) -> Result<AudioStems, AudioProcessingError> {
    let output_dir_str_owned = match output_dir.to_str() {
        None => {
            warn!("Cannot split file audio because output directory is not a valid UTF-8 path");
            return Err(AudioProcessingError::InvalidFile("Cannot split audio. Output directory is not a valid UTF-8 path".to_owned()))
        }
        Some(path) => {path.to_string()}
    };

    let audio_file_owned = match audio_file.to_str() {
        None => {
            warn!("Cannot split file audio because input file is not a valid UTF-8 path");
            return Err(AudioProcessingError::InvalidFile("Cannot split audio. Input file is not a valid UTF-8 path".to_owned()))}
        Some(path) => {path.to_string()}
    };

    info!("Splitting audio file {audio_file_owned} into its stems...");

    let options = SplitOptions {
        output_dir: output_dir_str_owned,
        model_name: "htdemucs_ort_v1".to_string(),
        manifest_url_override: None,
    };

    debug!("Using SplitOptions {:?}", options);

    let result = match split_file(&audio_file_owned, options) {
        Ok(result) => result,
        Err(error) => return Err(AudioProcessingError::CannotProcessAudio(error.to_string()))
    };

    info!("Processing successful!");
    Ok(AudioStems{
        vocals: PathBuf::from(result.vocals_path),
        bass: PathBuf::from(result.bass_path),
        drums: PathBuf::from(result.drums_path),
        other: PathBuf::from(result.other_path),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn integration_test() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test/example_audio/example_free_audio.wav");

        let mut output = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        output.push("output/");

        // Split the audio file
        let result = match split_audio_into_stems(&path, output) {
            Ok(result) => result,
            Err(error) => panic!("{}", error)
        };

        // Access the separated stems
        println!("Vocals: {}", result.vocals.to_str().unwrap());
        println!("Drums: {}", result.drums.to_str().unwrap());
        println!("Bass: {}", result.bass.to_str().unwrap());
        println!("Other: {}", result.other.to_str().unwrap());
    }
}