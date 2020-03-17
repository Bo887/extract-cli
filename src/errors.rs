#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Error {
    NoSuchFile,
    UnrecognizedExtension,
    UnableToExtract
}
