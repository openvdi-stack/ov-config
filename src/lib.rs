//! ov-config is a configuration parsing library that provide macros and convenience functions for generating configuration schema, sanity check, flush, refresh, etc. Design for `.toml` and `.ini`.
//!
//! # Usage
//! - Create Configuration Schema
//! ```
//! extern crate ov_config;
//!
//! use ov_config::*;
//!
//! make_config!(
//!     TestConfig,
//!     SECTION1 {
//!         //key: Type: Default Value => Verification closure
//!         a_string: String: "key1".into() => |x: &String| x.len() > 0,
//!         a_vector: Vec<i32>: vec![1, 2, 3] => |x: &Vec<i32>| x.len() < 4
//!     };
//!     // Support for multi section per config
//!     SECTION2 {
//!         a_i32: i32: 15 => |x: &i32| *x < 20,
//!         a_bool: bool: true => |_| true
//!     }
//! );
//!
//! fn main() {
//!     let config = TestConfig{..Default::default()};
//!     assert_eq!(config.SECTION1.a_string, "key1");
//!     assert_eq!(config.SECTION1.a_vector, vec![1, 2, 3]);
//!     assert_eq!(config.SECTION2.a_i32, 15);
//!     assert_eq!(config.SECTION2.a_bool, true);
//! }
//! ```
//!
//! - Get config from file -- will automatcially do sanity check on each value.
//! ```
//! extern crate ov_config;
//! use ov_config::*;
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! make_config!(
//!     TestConfig,
//!     SECTION1 {
//!         //key: Type: Default Value => Verification closure
//!         a_string: String: "key1".into() => |x: &String| x.len() > 0,
//!         a_vector: Vec<i32>: vec![1, 2, 3] => |x: &Vec<i32>| x.len() < 4
//!     };
//!     // Support for multi section per config
//!     SECTION2 {
//!         a_i32: i32: 15 => |x: &i32| *x < 20,
//!         a_bool: bool: true => |_| true
//!     }
//! );
//!
//! fn main() {
//!     let config = r#"
//!         [SECTION1]
//!         a_string: i_am_a_string
//!         a_vector: [1, 2, 3]
//!         [SECTION2]
//!         a_i32: 12
//!         a_bool: true
//!     "#;
//!
//!     let mut file = File::create("PATH_TO_CONFIG.ini").unwrap();
//!     file.write_all(config.as_bytes()).unwrap();
//!     file.sync_all().unwrap();
//!
//!     let config = TestConfig::get_config("PATH_TO_CONFIG.ini").unwrap();
//!
//!     assert_eq!(config.SECTION1.a_string, "i_am_a_string");
//!     assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
//!     assert_eq!(config.SECTION2.a_i32, 12);
//!     assert_eq!(config.SECTION2.a_bool, true);
//!     std::fs::remove_file("PATH_TO_CONFIG.ini").unwrap();
//! }
//! ```
//! # Generated function [doc](../ov_config/struct.ExampleConfig.html).
//! See the [example config](../ov_config/struct.ExampleConfig.html) for generated function docs.

extern crate failure;
extern crate ini;
extern crate serde_json;

mod error;

pub use error::OVConfigError;
pub use ini::Ini;

/// The macro used to generate the configuration schema structure.
///
/// See the [crate level docs](../ov_config/index.html) for examples.
///
/// See the [example config](../ov_config/struct.ExampleConfig.html) for generated function docs.
///
#[macro_export]
macro_rules! make_config {
    (
        $name:ident,
        $(
            $section:ident {
                $($key:ident:$type:ty:$default_value:expr=>$closure:expr),*
            }
        );*
    ) => {
        mod ovconfig {
            use super::*;
            $(
                #[allow(non_camel_case_types)]
                #[derive(Debug, PartialEq)]
                pub struct $section{
                    $(pub $key: $type),*
                }

                impl $section {
                    /// Verification Function
                    pub fn verify(&self) -> Result<(), OVConfigError> {
                        $(
                            if !$closure(&self.$key) {
                                return Err(OVConfigError::BadValue{
                                    section:stringify!($section).into(),
                                    key:stringify!($key).into(),
                                    value: serde_json::to_string(&self.$key).unwrap_or("UNKONWN".into())
                                });
                            }
                        )*
                        Ok(())
                    }

                    pub fn get_config<T: AsRef<str> + ?Sized>(path: &T) -> Result<Self, OVConfigError> {
                        let ini =  Ini::load_from_file(path.as_ref())?;
                        Ok(Self{
                            $(
                                $key: match ini.get_from(Some(stringify!($section)), stringify!($key)) {
                                    None => $default_value,
                                    Some(v) => match stringify!($type) {
                                        "String" | "str" => serde_json::from_str(format!("\"{}\"", v).as_ref())?,
                                        _=> serde_json::from_str(v)?
                                    }
                                }
                            ),*
                        })
                    }

                }

                impl Default for $section {
                    fn default() -> Self {
                        Self {
                            $($key: $default_value),*
                        }
                    }
                }
            )*
        }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[derive(Debug, Default, PartialEq)]
        /// Configuration schema struct.
        ///
        /// Basically is a struct of all sections. User will need to use `Config.Section.Key` to access value.
        pub struct $name {
            pub c_p_a_t_h: String,
            $(pub $section: ovconfig::$section,)*
        }

        impl $name {
            /// Sanity check convenience function
            ///
            /// This function will exec the closure on each field with the input of the field's value.
            /// Change `c_p_a_t_h` will change the path that cached inthe configuration object.
            pub fn verify(&self) -> Result<(), OVConfigError> {
                $(self.$section.verify()?;)*
                Ok(())
            }

            fn get_config_impl<T:AsRef<str> + ?Sized>(path: &T) -> Result<Self, OVConfigError> {
                Ok(Self {
                    c_p_a_t_h: path.as_ref().into(),
                    $($section: ovconfig::$section::get_config(&path)?,)*
                })
            }

            /// Get configuration without auto verification.
            ///
            /// Will use default value if specific field is not found in the configuration file.
            ///
            /// # Argument:
            /// - path: Path to the configuration. This path will be cached in the object for refresh and flush.
            ///
            /// # Return:
            /// Will return configuration object on success.
            pub fn get_config_no_verify<T:AsRef<str> + ?Sized>(path: &T) -> Result<Self, OVConfigError> {
                Self::get_config_impl(path)
            }

            /// Get configuration with auto verification.
            ///
            /// Will use default value if specific field is not found in the configuration file.
            ///
            /// # Argument:
            /// - path: Path to the configuration. This path will be cached in the object for refresh and flush.
            ///
            /// # Return:
            /// Will return configuration object on success.
            pub fn get_config<T:AsRef<str> + ?Sized>(path: &T) -> Result<Self, OVConfigError> {
                let res = Self::get_config_impl(path)?;
                res.verify()?;
                Ok(res)
            }

            fn refresh_impl(&mut self) -> Result<(), OVConfigError> {
                $(self.$section = ovconfig::$section::get_config(&self.c_p_a_t_h)?;)*
                Ok(())
            }

            /// Read the configuration file and update current object.
            ///
            /// This function will automatically do sanity check on the value.
            pub fn refresh(&mut self) -> Result<(), OVConfigError>{
                self.refresh_impl()?;
                self.verify()?;
                Ok(())
            }

            /// Read the configuration file and update current object.
            ///
            /// This function will NOT automatically do sanity check on the value.
            pub fn refresh_no_verify(&mut self) -> Result<(), OVConfigError>{
                self.refresh_impl()?;
                Ok(())
            }

            fn flush_impl(&self) -> Result<(), OVConfigError> {
                let mut conf = Ini::new();
                $(
                    conf.with_section(Some(stringify!($section).to_string()))
                        $(.set(stringify!($key), serde_json::to_string(&self.$section.$key)?))*
                );*;

                conf.write_to_file(&self.c_p_a_t_h)?;
                Ok(())
            }

            /// Flush whatever in configuration object to file.
            ///
            /// This function will automatically do sanity check on the value.
            pub fn flush(&self) -> Result<(), OVConfigError> {
                self.verify()?;
                self.flush_impl()
            }

            /// Flush whatever in configuration object to file.
            ///
            /// This function will automatically do sanity check on the value.
            pub fn flush_no_verify(&self) -> Result<(), OVConfigError> {
                self.flush_impl()
            }
        }
    }
}

make_config!(ExampleConfig, Section {
    example:String:"example".into()=>|x: &String| x.len() > 0
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn ovc_test_error() {
        let section = "section";
        let key = "key";
        let value = "bad_value";

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

    make_config!(TestConfig, SECTION1 {
        a_string:String:"key1".into()=>|x: &String| x.len() > 0,
        a_vector:Vec<i32>:vec![1, 2, 3]=>|x: &Vec<i32>| x.len() < 4
    }; SECTION2 {
        a_i32:i32:15=>|x: &i32| *x < 20,
        a_bool:bool:true =>|x| vec![true, false].contains(x)
    });

    #[test]
    fn ovc_test_default() {
        let d = TestConfig {
            ..Default::default()
        };

        assert_eq!(d.SECTION1.a_string, "key1");
        assert_eq!(d.SECTION1.a_vector, vec![1, 2, 3]);
        assert_eq!(d.SECTION2.a_i32, 15);
        assert_eq!(d.SECTION2.a_bool, true);
        d.verify().unwrap();
    }

    #[test]
    fn ovc_test_verify() {
        let mut d = TestConfig {
            ..Default::default()
        };
        d.SECTION2.a_i32 = 50;
        match d.verify() {
            Ok(_) => panic!("Should not be OK"),
            Err(e) => assert_eq!(
                "OVConfigError: Bad [SECTION2]::a_i32. Found: 50",
                e.to_string()
            ),
        }
    }

    #[test]
    fn ovc_test_get_config() {
        let config = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 12
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_get_config.ini").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        file.sync_all().unwrap();

        let config = match TestConfig::get_config("ovc_test_get_config.ini") {
            Ok(c) => {
                std::fs::remove_file("ovc_test_get_config.ini").unwrap();
                c
            }
            Err(e) => {
                std::fs::remove_file("ovc_test_get_config.ini").unwrap();
                panic!(e);
            }
        };

        assert_eq!(config.SECTION1.a_string, "i_am_a_string");
        assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
        assert_eq!(config.SECTION2.a_i32, 12);
        assert_eq!(config.SECTION2.a_bool, true);
    }

    #[test]
    fn ovc_test_get_config_verify_failed() {
        let config = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 128
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_get_config_verify_failed.ini").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        file.sync_all().unwrap();

        match TestConfig::get_config("ovc_test_get_config_verify_failed.ini") {
            Ok(_) => {
                std::fs::remove_file("ovc_test_get_config_verify_failed.ini").unwrap();
                panic!("Should not be OK");
            }
            Err(e) => {
                std::fs::remove_file("ovc_test_get_config_verify_failed.ini").unwrap();
                assert_eq!(
                    "OVConfigError: Bad [SECTION2]::a_i32. Found: 128",
                    e.to_string()
                )
            }
        };
    }

    #[test]
    fn ovc_test_get_config_no_verify() {
        let config = r#"
        [SECTION1]
        a_string=i_am_a_string
        a_vector=[1, 2, 3]

        [SECTION2]
        a_i32=128
        a_bool=true
        "#;

        let mut file = File::create("ovc_test_get_config_no_verify.ini").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        file.sync_all().unwrap();

        let config = match TestConfig::get_config_no_verify("ovc_test_get_config_no_verify.ini") {
            Ok(c) => {
                std::fs::remove_file("ovc_test_get_config_no_verify.ini").unwrap();
                c
            }
            Err(e) => {
                std::fs::remove_file("ovc_test_get_config_no_verify.ini").unwrap();
                panic!(e.to_string());
            }
        };

        assert_eq!(config.SECTION1.a_string, "i_am_a_string");
        assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
        assert_eq!(config.SECTION2.a_i32, 128);
        assert_eq!(config.SECTION2.a_bool, true);
    }

    #[test]
    fn ovc_test_refresh() {
        let config = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 12
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_refresh.ini").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        file.sync_all().unwrap();

        let mut config = TestConfig::get_config("ovc_test_refresh.ini").unwrap();

        assert_eq!(config.SECTION1.a_string, "i_am_a_string");
        assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
        assert_eq!(config.SECTION2.a_i32, 12);
        assert_eq!(config.SECTION2.a_bool, true);

        let cfg = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 13
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_refresh.ini").unwrap();
        file.write_all(cfg.as_bytes()).unwrap();
        file.sync_all().unwrap();

        match config.refresh() {
            Ok(_) => std::fs::remove_file("ovc_test_refresh.ini").unwrap(),
            Err(e) => {
                std::fs::remove_file("ovc_test_refresh.ini").unwrap();
                panic!(e.to_string());
            }
        };
        assert_eq!(config.SECTION2.a_i32, 13);
    }

    #[test]
    fn ovc_test_refresh_verify_error() {
        let config = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 12
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_refresh_verify_error.ini").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        file.sync_all().unwrap();

        let mut config = TestConfig::get_config("ovc_test_refresh_verify_error.ini").unwrap();

        assert_eq!(config.SECTION1.a_string, "i_am_a_string");
        assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
        assert_eq!(config.SECTION2.a_i32, 12);
        assert_eq!(config.SECTION2.a_bool, true);

        let cfg = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 139
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_refresh_verify_error.ini").unwrap();
        file.write_all(cfg.as_bytes()).unwrap();
        file.sync_all().unwrap();

        match config.refresh() {
            Ok(_) => {
                std::fs::remove_file("ovc_test_refresh_verify_error.ini").unwrap();
                panic!("Should not be OK.");
            }
            Err(e) => {
                std::fs::remove_file("ovc_test_refresh_verify_error.ini").unwrap();
                assert_eq!(
                    "OVConfigError: Bad [SECTION2]::a_i32. Found: 139",
                    e.to_string()
                );
            }
        };
    }

    #[test]
    fn ovc_test_refresh_no_verify() {
        let config = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 12
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_refresh_no_verify.ini").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        file.sync_all().unwrap();

        let mut config = TestConfig::get_config("ovc_test_refresh_no_verify.ini").unwrap();

        assert_eq!(config.SECTION1.a_string, "i_am_a_string");
        assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
        assert_eq!(config.SECTION2.a_i32, 12);
        assert_eq!(config.SECTION2.a_bool, true);

        let cfg = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 130
        a_bool: true
        "#;

        let mut file = File::create("ovc_test_refresh_no_verify.ini").unwrap();
        file.write_all(cfg.as_bytes()).unwrap();
        file.sync_all().unwrap();

        match config.refresh_no_verify() {
            Ok(_) => std::fs::remove_file("ovc_test_refresh_no_verify.ini").unwrap(),
            Err(e) => {
                std::fs::remove_file("ovc_test_refresh_no_verify.ini").unwrap();
                panic!(e.to_string());
            }
        };
        assert_eq!(config.SECTION2.a_i32, 130);
    }

    #[test]
    fn ovc_test_flush() {
        let mut d = TestConfig {
            ..Default::default()
        };

        d.c_p_a_t_h = "ovc_test_flush.ini".into();
        d.flush().unwrap();
        assert!(std::path::Path::new("ovc_test_flush.ini").exists());
        let config = TestConfig::get_config("ovc_test_flush.ini").unwrap();
        assert_eq!(d, config);
        std::fs::remove_file("ovc_test_flush.ini").unwrap();
    }

    #[test]
    fn ovc_test_flush_failed() {
        let mut d = TestConfig {
            ..Default::default()
        };
        d.SECTION2.a_i32 = 50;
        d.c_p_a_t_h = "ovc_test_flush_failed.ini".into();
        match d.flush() {
            Ok(_) => panic!("Should not be OK"),
            Err(e) => assert_eq!(
                "OVConfigError: Bad [SECTION2]::a_i32. Found: 50",
                e.to_string()
            ),
        };
        assert!(!std::path::Path::new("ovc_test_flush_failed.ini").exists());
    }

    #[test]
    fn ovc_test_flush_no_verfiy() {
        let mut d = TestConfig {
            ..Default::default()
        };
        d.SECTION2.a_i32 = 50;
        d.c_p_a_t_h = "ovc_test_flush_no_verfiy.ini".into();
        d.flush_no_verify().unwrap();
        assert!(std::path::Path::new("ovc_test_flush_no_verfiy.ini").exists());
        let config = TestConfig::get_config_no_verify("ovc_test_flush_no_verfiy.ini").unwrap();
        assert_eq!(d, config);
        std::fs::remove_file("ovc_test_flush_no_verfiy.ini").unwrap();
    }
}
