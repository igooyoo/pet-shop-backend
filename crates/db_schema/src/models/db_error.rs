use crate::models::model_error::ModelError;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;
pub trait ModelErrorMessage {
    fn uniqueviolation_message(_message: Option<&str>) -> Option<String>;
    fn foreignkeyviolation_message(_message: Option<&str>) -> Option<String>;
    fn diesel_error(_error: Error) -> ModelError {
        match _error {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, ref _message) => {
                ModelError::UniqueViolation(Self::uniqueviolation_message(
                    _message.constraint_name(),
                ))
            }
            Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, ref _message) => {
                ModelError::ForeignKeyViolation(Self::foreignkeyviolation_message(
                    _message.constraint_name(),
                ))
            }
            Error::DatabaseError(DatabaseErrorKind::SerializationFailure, ref _message) => {
                ModelError::SerializeFailure(Some(_message.message().to_string()))
            }
            Error::DatabaseError(DatabaseErrorKind::UnableToSendCommand, ref _message) => {
                ModelError::InternalServerError(Some(_message.message().to_string()))
            }
            Error::DatabaseError(DatabaseErrorKind::__Unknown, ref _message) => {
                if _message.message().contains("syntax error") {
                    ModelError::QuerySyntax(Some(_message.message().to_string()))
                } else if _message.message().contains("column") {
                    ModelError::ColumnNotExists(Some(_message.message().to_string()))
                } else if _message.message().contains("WHERE") {
                    ModelError::QuerySyntax(Some("Q Must be type boolean, not type character varying".to_string()))
                } else {
                    ModelError::QuerySyntax(Some(_message.message().to_string()))
                }
            }
            Error::NotFound => ModelError::NotFound,
            Error::QueryBuilderError(ref _message) => {
                ModelError::QuerySyntax(Some(_message.to_string()))
            }
            Error::DeserializationError(ref _message) => {
                ModelError::QuerySyntax(Some(_message.to_string()))
            }
            Error::SerializationError(ref _message) => {
                ModelError::QuerySyntax(Some(_message.to_string()))
            }
            Error::RollbackTransaction => {
                ModelError::QuerySyntax(Some("RollbackTransaction".to_string()))
            }
            Error::AlreadyInTransaction => {
                ModelError::QuerySyntax(Some("AlreadyInTransaction".to_string()))
            }
            or => ModelError::InternalServerError(Some(or.to_string())),
        }
    }
}