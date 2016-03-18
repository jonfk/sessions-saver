
// from http://stackoverflow.com/questions/28911833/error-handling-best-practices
use postgres;
use jwt;

use std::convert;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    PostgresErr(postgres::error::Error),
    JwtErr(jwt::errors::Error),
    RustCryptoErr(&'static str),
    NotFoundErr(String),
    InvalidPasswordErr,
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Error {
        Error::PostgresErr(err)
    }
}

impl From<jwt::errors::Error> for Error {
    fn from(err: jwt::errors::Error) -> Error {
        Error::JwtErr(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PostgresErr(ref err) => err.description(),
            Error::JwtErr(ref err) => err.description(),
            Error::NotFoundErr(ref err) => err,
            Error::InvalidPasswordErr => "Invalid password",
            Error::RustCryptoErr(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(match *self {
            Error::PostgresErr(ref err) => err as &error::Error,
            Error::JwtErr(ref err) => err as &error::Error,
            ref e => e as &error::Error,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PostgresErr(ref err) => fmt::Display::fmt(err, f),
            Error::JwtErr(ref err) => fmt::Display::fmt(err, f),
            Error::NotFoundErr(ref err) => write!(f, "{}", error::Error::description(self)),
            Error::InvalidPasswordErr => write!(f, "{}", error::Error::description(self)),
            Error::RustCryptoErr(_) => write!(f, "{}", error::Error::description(self)),
        }
    }
}
