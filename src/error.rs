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
}
