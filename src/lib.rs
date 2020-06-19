macro_rules! errormake_impls {
    ($structname:ident) => {
        #[allow(dead_code)]
        impl $structname {
            /// Instantiate with no source or description
            pub fn new() -> $structname {
                $structname { source: None, description: None }
            }
            
            /// Instantiate with the given description and no source
            pub fn with_description(description: String) -> $structname {
                $structname { source: None, description: Some(description) }
            }

            /// Instantiate with the given source and no description
            pub fn with_source(source: Box<dyn std::error::Error + 'static>) -> $structname {
                $structname { source: Some(source), description: None }
            }

            /// Instantiate with the given source and description
            pub fn with_source_and_description(source: Box<dyn std::error::Error + 'static>, description: String) -> $structname {
                $structname { source: Some(source), description: Some(description) }
            }

            /// Instantiate with optional source and description
            /// determined by the arguments
            pub fn with_optional_data(source: Option<Box<dyn std::error::Error + 'static>>, description: Option<String>) -> $structname {
                $structname { source, description }
            }
        }

        impl std::fmt::Display for $structname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.source {
                    Some(source) => write!(f, "{}\n\nThe above error caused the following error:\n\n", source)?,
                    None => {},
                }
                write!(f, "{}: {}", stringify!($structname), match self.description.as_ref() {
                    Some(description) => description,
                    None => "No description provided",
                })?;
                Ok(())
            }
        }

        impl std::error::Error for $structname {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.source.as_ref().map(|err| err.as_ref())
            }
        }
    }
}

#[macro_export]
macro_rules! errormake {
    ($structname:ident) => {
        /// An error struct automatically created by `errormake`
        #[derive(Debug)]
        struct $structname {
            source: Option<Box<dyn std::error::Error + 'static>>,
            description: Option<String>,
        }

        errormake_impls!($structname);
    };
    (pub $structname:ident) => {
        /// An error struct automatically created by `errormake`
        #[derive(Debug)]
        pub struct $structname {
            source: Option<Box<dyn std::error::Error + 'static>>,
            description: Option<String>,
        }

        errormake_impls!($structname);
    }
}

errormake!(pub ExampleErrorStruct);

#[cfg(test)]
mod tests {
    use super::errormake;
    use std::error::Error;

    errormake!(TestingError);
    errormake!(pub PublicTestingError);

    #[test]
    fn test_stable() {
        let error1 = TestingError::new();
        assert_eq!("TestingError: No description provided", format!("{}", error1));
        assert!(error1.source().is_none());
        let error2 = TestingError::with_description(String::from("Custom error message"));
        assert_eq!("TestingError: Custom error message", format!("{}", error2));
        assert!(error2.source().is_none());
        let error3 = TestingError::with_source(Box::new(error2));
        assert_eq!("TestingError: Custom error message\n\nThe above error caused the following error:\n\nTestingError: No description provided", format!("{}", error3));
        assert!(error3.source().is_some());
        let error4 = TestingError::with_source_and_description(Box::new(TestingError::with_description(String::from("Custom error message"))), String::from("Another message"));
        assert_eq!("TestingError: Custom error message\n\nThe above error caused the following error:\n\nTestingError: Another message", format!("{}", error4));
        assert!(error4.source().is_some());
    }
}
