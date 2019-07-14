use super::*;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum OVConfigError {
    #[fail(
        display = "OVConfigError: Bad [{}]::{}. Found: {}",
        section, key, value
    )]
    BadValue {
        section: String,
        key: String,
        value: String,
    },
    #[fail(display = "ParseConfigError: {}", _0)]
    ParseConfigError(ini::ini::ParseError),
    #[fail(display = "ParseValueError: {}", _0)]
    ParseValueError(serde_json::error::Error),
    #[fail(display = "IoError: {}", _0)]
    IoError(std::io::Error),
}

impl From<ini::ini::Error> for OVConfigError {
    fn from(e: ini::ini::Error) -> OVConfigError {
        match e {
            ini::ini::Error::Io(err) => OVConfigError::IoError(err),
            ini::ini::Error::Parse(err) => OVConfigError::ParseConfigError(err),
        }
    }
}

impl From<std::io::Error> for OVConfigError {
    fn from(e: std::io::Error) -> OVConfigError {
        OVConfigError::IoError(e)
    }
}

impl From<serde_json::error::Error> for OVConfigError {
    fn from(e: serde_json::error::Error) -> OVConfigError {
        OVConfigError::ParseValueError(e)
    }
}
