use diesel;
use std::error;
use std::fmt;
use warp;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug)]
pub enum ServerError {
    Unknown,
    Database(diesel::result::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Unknown => write!(f, "unkown server error occured"),
            ServerError::Database(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Unknown => "unknown error",
            ServerError::Database(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ServerError::Unknown => None,
            ServerError::Database(ref e) => Some(e),
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
