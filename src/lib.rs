pub mod service;
pub mod response;
pub mod config;
pub mod hosts;
pub mod site;
pub mod api;
pub mod database;
pub mod schema;
pub mod collector;


/// Generic Error to satisfy Box<dyn Error>
use std::error::Error;
use std::fmt;

// Generic Error to satisfy Box<dyn Error>
#[derive(Debug)]
struct GenericError(String);

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GenericError {}

