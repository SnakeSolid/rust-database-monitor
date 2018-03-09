use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FtmResult;

use postgres::Error as PgError;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    ConnectError { message: String },
    DatabaseError { code: String, message: String },
    IoError { message: String },
    ConversionError { message: String },
    UnknownError,
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FtmResult {
        match *self {
            WorkerError::ConnectError { ref message } => write!(f, "{}", message),
            WorkerError::DatabaseError {
                ref code,
                ref message,
            } => write!(f, "{}: {}", code, message),
            WorkerError::IoError { ref message } => write!(f, "{}", message),
            WorkerError::ConversionError { ref message } => write!(f, "{}", message),
            WorkerError::UnknownError => write!(f, "Unknown error"),
        }
    }
}

impl Error for WorkerError {
    fn description(&self) -> &str {
        match *self {
            WorkerError::ConnectError { .. } => "Connection error",
            WorkerError::DatabaseError { .. } => "Database error",
            WorkerError::IoError { .. } => "IO error",
            WorkerError::ConversionError { .. } => "Conversion error",
            WorkerError::UnknownError { .. } => "Unknown error",
        }
    }
}

impl From<PgError> for WorkerError {
    fn from(error: PgError) -> WorkerError {
        if let Some(err) = error.as_connection() {
            WorkerError::ConnectError {
                message: err.description().into(),
            }
        } else if let Some(err) = error.as_db() {
            WorkerError::DatabaseError {
                code: err.code.code().into(),
                message: err.message.clone(),
            }
        } else if let Some(err) = error.as_conversion() {
            WorkerError::ConversionError {
                message: err.description().into(),
            }
        } else if let Some(err) = error.as_io() {
            WorkerError::IoError {
                message: err.description().into(),
            }
        } else {
            WorkerError::UnknownError
        }
    }
}
