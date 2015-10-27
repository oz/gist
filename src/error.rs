//! Error and Result module

extern crate hyper;

use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;

use self::hyper::error::Error as HyperError;
use self::Error::{Io, Http, ApiError};

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Http(HyperError),
    ApiError,
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::Http(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Io(ref e) => e.description(),
            Http(ref e) => e.description(),
            ApiError => "API error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Io(ref e) => Some(e),
            Http(ref e) => Some(e),
            ApiError => None,
        }
    }
}
