#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoSuchFile,
    UnrecognizedExtension,
    UnableToExtract,
}
