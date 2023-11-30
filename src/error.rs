// In your AppError.rs or similar file

use bincode;
use rusqlite;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    Sqlite(rusqlite::Error),
    Bincode(bincode::Error),
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::Sqlite(ref err) => write!(f, "SQLite Error: {}", err),
            AppError::Bincode(ref err) => write!(f, "Bincode Error: {}", err),
            AppError::Io(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> AppError {
        AppError::Sqlite(err)
    }
}

impl From<bincode::Error> for AppError {
    fn from(err: bincode::Error) -> AppError {
        AppError::Bincode(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}
