use diesel;
use std::error;
use std::fmt;
use warp;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    Unknown,
    Database(diesel::result::Error),
    Input(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Unknown => write!(f, "unknown server error occured"),
            ServerError::Database(ref e) => e.fmt(f),
            ServerError::Input(ref message) => write!(f, "input error {}", message),
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Unknown => "unknown error",
            ServerError::Database(ref e) => e.description(),
            ServerError::Input(ref message) => message,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ServerError::Unknown => None,
            ServerError::Database(ref e) => Some(e),
            ServerError::Input(_) => None,
        }
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        ServerError::Database(err)
    }
}

impl Into<warp::Rejection> for ServerError {
    fn into(self) -> warp::Rejection {
        warp::reject::custom(self)
    }
}
