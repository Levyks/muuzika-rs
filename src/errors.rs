use std::sync::{MutexGuard, PoisonError};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::dtos::responses::ErrorResponse;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MuuzikaError {
    #[error("Unknown error")]
    Unknown {
        #[serde(skip_serializing)]
        message: String
    },

    #[error("Invalid request body")]
    InvalidRequestBody { message: String },

    #[error("Room with code {code} was not found")]
    RoomNotFound { code: String },

    #[error("Out of available codes")]
    OutOfAvailableCodes,

    #[error("Username taken")]
    UsernameTaken,
}

impl ResponseError for MuuzikaError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MuuzikaError::Unknown { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MuuzikaError::InvalidRequestBody { .. } => StatusCode::BAD_REQUEST,
            MuuzikaError::RoomNotFound { .. } => StatusCode::NOT_FOUND,
            MuuzikaError::OutOfAvailableCodes => StatusCode::SERVICE_UNAVAILABLE,
            MuuzikaError::UsernameTaken => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ErrorResponse::from_error(self))
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for MuuzikaError
{
    fn from(error: PoisonError<MutexGuard<'_, T>>) -> Self {
        MuuzikaError::Unknown {
            message: format!("PoisonError: {:?}", error)
        }
    }
}