extern crate failure;

mod error;

pub use error::OVConfigError;

type OVCResult<T> = Result<T, OVConfigError>;

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
                pub struct $section{
                    $(pub $key: $type),*
                }

                impl $section {
                    pub fn verify(&self) -> OVCResult<()> {
                        $(
                            if !$closure(&self.$key) {
                                return Err(OVConfigError::BadValue{
                                    section:stringify!($section).into(),
                                    key:stringify!($key).into(),
                                    value: self.$key.to_string()
                                });
                            }
                        )*
                        Ok(())
                    }

                    pub fn get_config<T: AsRef<str> + ?Sized>(_path: &T) -> OVCResult<Self> {
                        Ok(Self{..Default::default()})
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
        pub struct $name {
            $(pub $section: ovconfig::$section,)*
        }

        impl $name {
            pub fn verify(&self) -> OVCResult<()> {
                $(self.$section.verify()?;)*
                Ok(())
            }

            pub fn get_config<T:AsRef<str> + ?Sized>(path: &T) -> OVCResult<Self> {
                Ok(Self {
                    $($section: ovconfig::$section::get_config(&path)?,)*
                })
            }
        }
    }
}

make_config!(test_config, test_section1 {
    key1:String:"key1".into()=>|_x| true,
    key2:String:"key2".into()=>|_x| true,
    key3:String:"key3".into()=>|_x| true
}; TestSection2 {
    key4:String:"key1".into()=>|_x| true,
    key2:String:"key2".into()=>|_x| true,
    key3:String:"key3".into()=>|_x| true
});

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
