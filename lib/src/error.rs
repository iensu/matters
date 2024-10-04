#![allow(clippy::module_name_repetitions)]

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ffi", repr(u32))]
pub enum ErrorKind {
    InvalidInput = 1,
    ProblemGeneration,
    ProblemGenerationDivision,
}

pub struct LibError {
    pub kind: ErrorKind,
    pub message: String,
}

impl Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Encountered an error: {{ kind: {:?}, message: {} }}",
            self.kind, self.message
        )
    }
}

impl Debug for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("message", &self.message)
            .field("file", &file!())
            .field("line", &line!())
            .finish()
    }
}
