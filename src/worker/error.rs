use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FtmResult;

use postgres::error::ConnectError;
use postgres::error::Error as PgError;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    ConnectError { message: String },
    DatabaseError { message: String },
    IoError { message: String },
    ConversionError { message: String },
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FtmResult {
        match *self {
            WorkerError::ConnectError { ref message } => write!(f, "Connection error: {}", message),
            WorkerError::DatabaseError { ref message } => write!(f, "Database error: {}", message),
            WorkerError::IoError { ref message } => write!(f, "IO error: {}", message),
            WorkerError::ConversionError { ref message } => {
                write!(f, "Conversion error: {}", message)
            }
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
        }
    }
}

impl From<ConnectError> for WorkerError {
    fn from(error: ConnectError) -> WorkerError {
        match error {
            ConnectError::ConnectParams(ref err) => WorkerError::ConnectError {
                message: err.description().into(),
            },
            ConnectError::Db(ref err) => WorkerError::ConnectError {
                message: err.description().into(),
            },
            ConnectError::Tls(ref err) => WorkerError::ConnectError {
                message: err.description().into(),
            },
            ConnectError::Io(ref err) => WorkerError::ConnectError {
                message: err.description().into(),
            },
        }
    }
}

impl From<PgError> for WorkerError {
    fn from(error: PgError) -> WorkerError {
        match error {
            PgError::Db(ref err) => WorkerError::DatabaseError {
                message: err.description().into(),
            },
            PgError::Io(ref err) => WorkerError::IoError {
                message: err.description().into(),
            },
            PgError::Conversion(ref err) => WorkerError::ConversionError {
                message: err.description().into(),
            },
        }
    }
}
