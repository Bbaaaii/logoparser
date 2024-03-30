use std::fmt;
use std::panic::Location;

pub struct LocError {
    pub message: String,
    pub location: Location<'static>,
}

impl LocError {
    pub fn new(message: &str, location: Location<'static>) -> Self {
        LocError {
            message: message.to_string(),
            location,
        }
    }
}

// Impl debug and display for nice printing!

impl fmt::Debug for LocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \nLocated at: {}", self.message, self.location)
    }
}

impl fmt::Display for LocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} \nLocated at: {}", self.message, self.location)
    }
}
