use std::{io, string::FromUtf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DoomguyError {
    /// An error occuring with doomguy that spawned from an IO error.
    #[error("An IO error occured: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to parse UTF-8LE byte")]
    UTF8ParseError(#[from] FromUtf8Error),
}
