use super::*;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum OVConfigError {
    #[fail(
        display = "OVConfigError: Bad [{}]::{}. Found: {} -- Expected: {:?}.",
        section, key, value, suggest
    )]
    BadValueSuggest {
        section: String,
        key: String,
        value: String,
        suggest: Vec<String>,
    },
    #[fail(
        display = "OVConfigError: Bad [{}]::{}. Found: {} -- Reason: {}.",
        section, key, value, reason
    )]
    BadValueReason {
        section: String,
        key: String,
        value: String,
        reason: String,
    },
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
