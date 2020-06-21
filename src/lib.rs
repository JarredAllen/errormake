//! A macro for automatically generating structs which implement the
//! `Error` trait from `std::error`.
//!
//! The `errormake!` macro generates a struct which implements an error
//! and may optionally contain a description and/or a source error. The
//! resulting struct may be either public or private to the module.
//!
//! Here is an example of using some of its functionality:
//! ```
//! use errormake::errormake;
//!
//! errormake!(pub ExampleError);
//!
//! // Create an error with no description or source
//! let error1 = ExampleError::new();
//! // Create an error with a description, but no source
//! let error2 = ExampleError::with_description(String::from("Error description"));
//! // Create an error with a source, but no description
//! let error3 = ExampleError::with_source(Box::new(error1));
//! // Create an error with a source and a description
//! let error4 = ExampleError::with_source_and_description(Box::new(error3), String::from("Error description"));
//! ```
//!
//! If making a public error struct, you can also add custom
//! documentation through the `doc` attribute, as follows:
//! ```
//! use errormake::errormake;
//!
//! // The `DocumentedError` struct now has a documentation, which will
//! // show up if `cargo doc` is run.
//! errormake!(#[doc="Documentation comments"] pub DocumentedError);
//! ```
//!

#[macro_export]
/// The macro used to generate basic Error structs.
///
/// See the [crate docs](../errormake/index.html) for the full
/// documentation.
macro_rules! errormake {
    ($structname:ident) => {
        #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
        struct $structname<T: std::error::Error + 'static> {
            source: Option<Box<T>>,
            description: Option<String>,
        }

        errormake!(impl $structname);
    };
    ($(#[$meta:meta])* pub $structname:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
        pub struct $structname<T: std::error::Error + 'static> {
            source: Option<Box<T>>,
            description: Option<String>,
        }

        errormake!(impl $structname);
    };
    (impl $structname:ident) => {
        #[allow(dead_code)]
        impl $structname<std::convert::Infallible> {
            /// Instantiate with no source or description
            pub fn new() -> $structname<std::convert::Infallible> {
                $structname {
                    source: None,
                    description: None,
                }
            }

            /// Instantiate with the given description and no source
            pub fn with_description(description: String) -> $structname<std::convert::Infallible> {
                $structname {
                    source: None,
                    description: Some(description),
                }
            }
        }

        #[allow(dead_code)]
        impl<T: std::error::Error + 'static> $structname<T> {
            /// Instantiate with the given source and no description
            pub fn with_source(source: T) -> $structname<T> {
                $structname {
                    source: Some(Box::new(source)),
                    description: None,
                }
            }

            /// Instantiate with the given source and description
            pub fn with_source_and_description(source: T, description: String) -> $structname<T> {
                $structname {
                    source: Some(Box::new(source)),
                    description: Some(description),
                }
            }

            /// Instantiate with optional source and description
            /// determined by the arguments
            pub fn with_optional_data(
                source: Option<Box<T>>,
                description: Option<String>,
            ) -> $structname<T> {
                $structname {
                    source,
                    description,
                }
            }
        }

        impl<T: std::error::Error + 'static> std::fmt::Display for $structname<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.source {
                    Some(source) => write!(
                        f,
                        "{}\n\nThe above error caused the following error:\n\n",
                        source
                    )?,
                    None => {}
                }
                write!(
                    f,
                    concat!(stringify!($structname), ": {}"),
                    match self.description.as_ref() {
                        Some(description) => description,
                        None => "No description provided",
                    }
                )?;
                Ok(())
            }
        }

        impl<T: std::error::Error + 'static> std::error::Error for $structname<T> {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.source
                    .as_ref()
                    .map(|err| err.as_ref() as &(dyn std::error::Error + 'static))
            }
        }
    };
}

errormake!(#[doc="An example of an error struct made by `errormake`"] pub ExampleErrorStruct);

#[cfg(test)]
mod tests {
    use super::errormake;
    use std::error::Error;

    errormake!(TestingError);
    errormake!(pub PublicTestingError);

    #[test]
    fn test_stable() {
        let error1 = TestingError::new();
        assert_eq!(
            "TestingError: No description provided",
            format!("{}", error1)
        );
        assert!(error1.source().is_none());
        let error2 = TestingError::with_description(String::from("Custom error message"));
        assert_eq!("TestingError: Custom error message", format!("{}", error2));
        assert!(error2.source().is_none());
        let error3 = TestingError::with_source(Box::new(error2));
        assert_eq!("TestingError: Custom error message\n\nThe above error caused the following error:\n\nTestingError: No description provided", format!("{}", error3));
        assert!(error3.source().is_some());
        let error4 = TestingError::with_source_and_description(
            Box::new(TestingError::with_description(String::from(
                "Custom error message",
            ))),
            String::from("Another message"),
        );
        assert_eq!("TestingError: Custom error message\n\nThe above error caused the following error:\n\nTestingError: Another message", format!("{}", error4));
        assert!(error4.source().is_some());
    }

    #[test]
    fn test_derives() {
        let error1 = TestingError::new();
        assert_eq!(error1, error1.clone());
        let error2 = TestingError::with_source(error1.clone());
        assert_eq!(error2, error2.clone());
        assert_eq!(error2, error2);
        let error3 =
            TestingError::with_source_and_description(error1.clone(), String::from("description"));
        assert_ne!(error3, error2);
        let error4 = TestingError::with_description(String::from("description"));
        assert_ne!(error1, error4);
    }
}
