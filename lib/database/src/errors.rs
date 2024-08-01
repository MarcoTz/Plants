use std::fmt;
use std::io;

pub enum DBError {
    FilesError(io::Error),
    JSONError(serde_json::Error),
    PathError(String),
}

impl From<io::Error> for DBError {
    fn from(err: io::Error) -> DBError {
        DBError::FilesError(err)
    }
}

impl From<serde_json::Error> for DBError {
    fn from(err: serde_json::Error) -> DBError {
        DBError::JSONError(err)
    }
}

impl fmt::Debug for DBError {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBError::FilesError(err) => fmt::Debug::fmt(err, frmt),
            DBError::JSONError(err) => fmt::Debug::fmt(err, frmt),
            DBError::PathError(err) => frmt.write_str(err),
        }
    }
}
