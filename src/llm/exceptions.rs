use log::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GlueError {
    message: String,
}

impl GlueError {
    pub fn new(msg: &str) -> Self {
        error!("\n Error: {}\n", msg);
        Self {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for GlueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GlueError {}

#[derive(Debug)]
pub struct GlueLLMError {
    message: String,
    exception_details: String,
}

impl GlueLLMError {
    pub fn new(msg: &str, exception: impl fmt::Display) -> Self {
        let full_message = format!(
            "LLM exception\nException: {}\nException logs: {}",
            msg, exception
        );
        error!("{}", full_message);
        Self {
            message: msg.to_string(),
            exception_details: exception.to_string(),
        }
    }
}

impl fmt::Display for GlueLLMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LLM exception\nException: {}\nException logs: {}",
            self.message, self.exception_details
        )
    }
}

impl Error for GlueLLMError {}

impl From<GlueLLMError> for GlueError {
    fn from(err: GlueLLMError) -> Self {
        GlueError::new(&err.to_string())
    }
}

#[derive(Debug)]
pub struct GlueValidationError {
    message: String,
    exception_details: String,
}

impl GlueValidationError {
    pub fn new(msg: &str, exception: impl fmt::Display) -> Self {
        let full_message = format!(
            "[Invalid user input detected]\nException: {}\nException logs: {}",
            msg, exception
        );
        error!("{}", full_message);
        Self {
            message: msg.to_string(),
            exception_details: exception.to_string(),
        }
    }
}

impl fmt::Display for GlueValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Invalid user input detected]\nException: {}\nException logs: {}",
            self.message, self.exception_details
        )
    }
}

impl Error for GlueValidationError {}

impl From<GlueValidationError> for GlueError {
    fn from(err: GlueValidationError) -> Self {
        GlueError::new(&err.to_string())
    }
}
