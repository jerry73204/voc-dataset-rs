use std::path::{Path, PathBuf};
use failure::Error;

#[derive(Debug, Fail)]
#[fail(display = "Cannot parse file {:?}: {}", path, error)]
pub struct ParseAnnotationError {
    path: PathBuf,
    error: Error,
}

impl ParseAnnotationError {
    pub fn new(path: &Path, error: Error) -> ParseAnnotationError {
        ParseAnnotationError {
            path: path.to_owned(),
            error,
        }
    }
}
