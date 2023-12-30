use std::error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

// Implement the Display trait for the custom error type
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement the std::error::Error trait for the custom error type
// to use the default implementation
impl error::Error for CustomError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_error_display_format() {
        let error = CustomError {
            message: String::from("This is a custom error."),
        };

        assert_eq!(format!("{}", error), "This is a custom error.");
    }

    #[test]
    fn custom_error_debug_format() {
        let error = CustomError {
            message: String::from("Another custom error."),
        };

        assert_eq!(
            format!("{:?}", error),
            "CustomError { message: \"Another custom error.\" }"
        );
    }

    #[test]
    fn custom_error_into_string() {
        let error = CustomError {
            message: String::from("Yet another custom error."),
        };

        let error_string: String = error.to_string();
        assert_eq!(error_string, "Yet another custom error.");
    }
}
