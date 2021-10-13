use std::io::Error;

pub enum RuntimeException {
    NoHomeFolderDetected,
    UnableToReadFile(Error),
    UnableToWriteFile(Error),
}
