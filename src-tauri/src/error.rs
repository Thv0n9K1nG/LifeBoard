use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AppError", 2)?;
        match self {
            AppError::Database(e) => {
                state.serialize_field("error", "DatabaseError")?;
                state.serialize_field("message", &e.to_string())?;
            }
            AppError::NotFound(msg) => {
                state.serialize_field("error", "NotFound")?;
                state.serialize_field("message", msg)?;
            }
            AppError::Validation(msg) => {
                state.serialize_field("error", "ValidationError")?;
                state.serialize_field("message", msg)?;
            }
            AppError::Internal(msg) => {
                state.serialize_field("error", "InternalError")?;
                state.serialize_field("message", msg)?;
            }
        }
        state.end()
    }
}
