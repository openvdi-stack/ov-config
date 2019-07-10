extern crate failure;

mod error;

pub use error::OVConfigError;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_error() {
        let section = "section";
        let key = "key";
        let value = "bad_value";
        let suggest = vec!["correct1".to_string(), "correct2".to_string()];
        let reason = "some_reason";
        assert_eq!(
            format!(
                "OVConfigError: Bad [{}]::{}. Found: {} -- Expected: {:?}.",
                section, key, value, suggest
            ),
            OVConfigError::BadValueSuggest {
                section: section.to_string(),
                key: key.to_string(),
                value: value.to_string(),
                suggest
            }
            .to_string()
        );
        assert_eq!(
            format!(
                "OVConfigError: Bad [{}]::{}. Found: {} -- Reason: {}.",
                section, key, value, reason
            ),
            OVConfigError::BadValueReason {
                section: section.to_string(),
                key: key.to_string(),
                value: value.to_string(),
                reason: reason.to_string()
            }
            .to_string()
        );
        assert_eq!(
            format!(
                "OVConfigError: Bad [{}]::{}. Found: {}",
                section, key, value
            ),
            OVConfigError::BadValue {
                section: section.to_string(),
                key: key.to_string(),
                value: value.to_string(),
            }
            .to_string()
        );
    }
}
