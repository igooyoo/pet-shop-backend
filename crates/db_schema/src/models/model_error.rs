use serde::Serialize;
#[derive(Debug)]
pub enum ModelError {
    UniqueViolation(Option<String>),
    ForeignKeyViolation(Option<String>),
    InternalServerError(Option<String>),
    SerializeFailure(Option<String>),
    ColumnNotExists(Option<String>),
    QuerySyntax(Option<String>),
    NotFound,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ApiError {
    pub status_code: ApiErrorEnum,
    pub error_body: ErrorBody,
}

#[derive(Serialize, Debug)]
pub enum ApiErrorEnum {
    InvalidRequest,
    InvalidToken,
    InvalidClient,
    UnsufficientScope,
    AccountNotAllowed,
    ObjectLimitReached,
    NotFound,
    MethodNotAllowed,
    Conflict,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    TemporarilyUnavailable,
}
