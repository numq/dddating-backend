use std::{error::Error, fmt::{Debug, Display, Formatter, Result}};

pub struct InternalError {
    name: String,
    message: String,
    line: u32,
}

pub trait MakeError: Sized {
    fn make_error(name: &str, message: &str, line: u32) -> Box<dyn Error + Send + Sync> {
        Box::new(InternalError { name: name.to_string(), message: message.to_string(), line })
    }
}

impl MakeError for InternalError {}

impl Debug for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: {} - {}", self.name, self.line, self.message)
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} - {}", self.name, self.message)
    }
}

impl Error for InternalError {}

///
/// # Arguments
///
/// * `message` - A string that represents the reason for the error
///
/// * `name` - A string that represents the name for the error
///
#[macro_export]
macro_rules! make_error {
    ($message:expr)=>{
        {
            let file = file!();
            let line = line!();
            <$crate::InternalError as $crate::MakeError>::make_error(file, $message, line)
        }
    };
    ($name:expr, $message:expr)=>{
        {
            let name = $name;
            let line = line!();
            let display_name = if !name.is_empty() {
                name
            } else {
                file!()
            };
            <$crate::InternalError as $crate::MakeError>::make_error(display_name, $message, line)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unnamed_error() {
        let default_name = "src\\lib.rs";
        let message = "something get wrong";
        let error = make_error!(message);
        assert_eq!(error.to_string(), format!("{} - {}", default_name, message));
    }

    #[test]
    fn test_named_error() {
        let name = "test module";
        let message = "something get wrong";
        let error = make_error!(name, message);
        assert_eq!(error.to_string(), format!("{} - {}", name, message));
    }
}
