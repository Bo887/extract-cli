use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoSuchFile,
    UnrecognizedExtension,
    UnableToExtract,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoSuchFile => write!(f, "No such file found! Does the specified file exist?"),
            Error::UnrecognizedExtension => write!(f, "Unrecognized file extension! Is the extension supported?"),
            Error::UnableToExtract => write!(f, "Unable to extract the archive. The child process either failed or return a non-zero exit code."),
        }
    }
}
