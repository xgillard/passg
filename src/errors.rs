//! This module defines the error that might occur while using passg.

/// These are the errors that can occur while generating a password
#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
pub enum Error {
    #[error("could not parse")]
    ParseError(String),
}
