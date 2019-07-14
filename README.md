# ov-config
[![Build Status](https://travis-ci.com/openvdi-stack/ov-config.svg?branch=master)](https://travis-ci.com/openvdi-stack/ov-config)
[![Crates.io](https://img.shields.io/crates/v/config_struct.svg)](https://crates.io/crates/ov-config)
[![Docs.rs](https://docs.rs/config_struct/badge.svg)](https://docs.rs/ov-config/0.1.1/ov_config/)

A configuration parsing library that provide macros and convenience functions for generating configuration schema, sanity check, flush, refresh, etc. Design for `.toml` and `.ini`.

# Usage
- Create Configuration Schema
```rust
extern crate ov_config;

use ov_config::*;

make_config!(
	TestConfig,
	SECTION1 {
		//key: Type: Default Value => Verification closure
	    a_string: String: "key1".into() => |x: &String| x.len() > 0,
	    a_vector: Vec<i32>: vec![1, 2, 3] => |x: &Vec<i32>| x.len() < 4
	};
	// Support for multi section per config
	SECTION2 {
	    a_i32: i32: 15 => |x: &i32| *x < 20,
	    a_bool: bool: true => |_| true
	}
);

fn main() {
	let config = TestConfig{..Default::default()};
	assert_eq!(config.SECTION1.a_string, "key1");
	assert_eq!(config.SECTION1.a_vector, vec![1, 2, 3]);
	assert_eq!(config.SECTION2.a_i32, 15);
	assert_eq!(config.SECTION2.a_bool, true);
}
```

- Get config from file -- will automatcially do sanity check on each value.
```rust
extern crate ov_config;

use ov_config::*;
use std::fs::File;
use std::io::prelude::*;

make_config!(
	TestConfig,
	SECTION1 {
		//key: Type: Default Value => Verification closure
	    a_string: String: "key1".into() => |x: &String| x.len() > 0,
	    a_vector: Vec<i32>: vec![1, 2, 3] => |x: &Vec<i32>| x.len() < 4
	};
	// Support for multi section per config
	SECTION2 {
	    a_i32: i32: 15 => |x: &i32| *x < 20,
	    a_bool: bool: true => |_| true
	}
);

fn main() {
	let config = r#"
        [SECTION1]
        a_string: i_am_a_string
        a_vector: [1, 2, 3]
        [SECTION2]
        a_i32: 12
        a_bool: true
    "#;

    let mut file = File::create("PATH_TO_CONFIG.ini").unwrap();
    file.write_all(config.as_bytes()).unwrap();
    file.sync_all().unwrap();

    let config = TestConfig::get_config("PATH_TO_CONFIG.ini").unwrap();

    assert_eq!(config.SECTION1.a_string, "i_am_a_string");
    assert_eq!(config.SECTION1.a_vector, [1, 2, 3]);
    assert_eq!(config.SECTION2.a_i32, 12);
    assert_eq!(config.SECTION2.a_bool, true);
}
```

More details could be found from the documentation.
