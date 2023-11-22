
use crate::models::model_error::{ApiError, ApiErrorEnum, ErrorBody, ModelError};
use actix_web::ResponseError;
use actix_web::{http::StatusCode, HttpResponse};

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(_err: serde_json::error::Error) -> ApiError {
        ApiError {
            status_code: ApiErrorEnum::InvalidRequest,
            error_body: ErrorBody {
                code: "invalid_request".to_string(),
                message: "Invalid request. See https://developer.aircampi.com/api/".to_string(),
            },
        }
    }
}

impl From<std::num::ParseIntError> for ApiError {
    fn from(_err: std::num::ParseIntError) -> ApiError {
        ApiError {
            status_code: ApiErrorEnum::InvalidRequest,
            error_body: ErrorBody {
                code: "invalid_request".to_string(),
                message: "Invalid request. See https://developer.aircampi.com/api/".to_string(),
            },
        }
    }
}

impl From<actix_web::error::Error> for ApiError {
    fn from(_err: actix_web::error::Error) -> ApiError {
        ApiError {
            status_code: ApiErrorEnum::InvalidRequest,
            error_body: ErrorBody {
                code: "invalid_request".to_string(),
                message: "Invalid request. See https://developer.aircampi.com/api/".to_string(),
            },
        }
    }
}

impl From<actix_web::error::QueryPayloadError> for ApiError {
    fn from(_err: actix_web::error::QueryPayloadError) -> ApiError {
        ApiError {
            status_code: ApiErrorEnum::InvalidRequest,
            error_body: ErrorBody {
                code: "invalid_request".to_string(),
                message: "Invalid request. See https://developer.aircampi.com/api/".to_string(),
            },
        }
    }
}

impl From<ModelError> for ApiError {
    fn from(err: ModelError) -> ApiError {
        match err {
            ModelError::UniqueViolation(ref _message) => ApiError {
                status_code: ApiErrorEnum::Conflict,
                error_body: ErrorBody {
                    code: "conflict".to_string(),
                    message: _message.clone().unwrap() + ". See https://developer.aircampi.com/api/",
                },
            },

            ModelError::ForeignKeyViolation(ref _message) => ApiError {
                status_code: ApiErrorEnum::InvalidRequest,
                error_body: ErrorBody {
                    code: "invalid_request".to_string(),
                    message: _message.clone().unwrap() + ". See https://developer.aircampi.com/api/",
                },
            },

            ModelError::InternalServerError(ref _message) => ApiError {
                status_code: ApiErrorEnum::InvalidRequest,
                error_body: ErrorBody {
                    code: "internal_server_error".to_string(),
                    message: _message.clone().unwrap() + ". See https://developer.aircampi.com/api/",
                },
            },

            ModelError::SerializeFailure(ref _message) => ApiError {
                status_code: ApiErrorEnum::InvalidRequest,
                error_body: ErrorBody {
                    code: "invalid_request".to_string(),
                    message: _message.clone().unwrap() + ". See https://developer.aircampi.com/api/",
                },
            },

            ModelError::ColumnNotExists(ref _message) => ApiError {
                status_code: ApiErrorEnum::InvalidRequest,
                error_body: ErrorBody {
                    code: "invalid_request".to_string(),
                    message: _message.to_owned().unwrap()
                        + ". See https://developer.aircampi.com/api/",
                },
            },

            ModelError::QuerySyntax(ref _message) => ApiError {
                status_code: ApiErrorEnum::InvalidRequest,
                error_body: ErrorBody {
                    code: "invalid_request".to_string(),
                    message: _message.clone().unwrap() + ". See https://developer.aircampi.com/api/",
                },
            },

            ModelError::NotFound => ApiError {
                status_code: ApiErrorEnum::NotFound,
                error_body: ErrorBody {
                    code: "not_found".to_string(),
                    message: "Resource not found. See https://developer.aircampi.com/api/"
                        .to_string(),
                },
            },
        }
    }
}

impl ApiError {
    pub fn new(_type: ApiErrorEnum, _code: String, _message: String) -> ApiError {
        ApiError {
            status_code: _type,
            error_body: ErrorBody {
                code: _code,
                message: _message,
            },
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self.status_code {
            ApiErrorEnum::InvalidRequest => StatusCode::BAD_REQUEST,
            ApiErrorEnum::InvalidToken => StatusCode::UNAUTHORIZED,
            ApiErrorEnum::InvalidClient => StatusCode::UNAUTHORIZED,
            ApiErrorEnum::UnsufficientScope => StatusCode::FORBIDDEN,
            ApiErrorEnum::AccountNotAllowed => StatusCode::FORBIDDEN,
            ApiErrorEnum::ObjectLimitReached => StatusCode::FORBIDDEN,
            ApiErrorEnum::NotFound => StatusCode::NOT_FOUND,
            ApiErrorEnum::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiErrorEnum::Conflict => StatusCode::CONFLICT,
            ApiErrorEnum::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            ApiErrorEnum::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorEnum::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ApiErrorEnum::TemporarilyUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.error_body.clone())
    }
}



