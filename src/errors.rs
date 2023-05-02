use std::sync::{MutexGuard, PoisonError};
use actix::MailboxError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use futures::channel::oneshot::Canceled;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::dtos::responses::ErrorResponse;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UserFacingError {
    #[error("Unknown error")]
    InternalError(#[serde(skip_serializing)] InternalError),

    #[error("Invalid request body")]
    InvalidRequestBody { message: String },

    // The compiler literally panics if I put `room_code` instead of `code` here, why???
    #[error("Room with code \"{code}\" was not found")]
    RoomNotFound {  code: String },

    #[error("Out of available codes")]
    OutOfAvailableCodes,

    #[error("There is already a player with username \"{username}\" in room \"{room_code}\"")]
    UsernameTaken { 
        room_code: String,
        username: String 
    },

    #[error("There is no player with username \"{username}\" in room \"{room_code}\"")]
    PlayerNotFound { 
        room_code: String, 
        username: String 
    },
}

impl ResponseError for UserFacingError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserFacingError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            UserFacingError::InvalidRequestBody { .. } => StatusCode::BAD_REQUEST,
            UserFacingError::RoomNotFound { .. } => StatusCode::NOT_FOUND,
            UserFacingError::UsernameTaken { .. } => StatusCode::CONFLICT,
            UserFacingError::OutOfAvailableCodes => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ErrorResponse::from_error(self))
    }
}

impl From<InternalError> for UserFacingError
{
    fn from(error: InternalError) -> Self {
        UserFacingError::InternalError(error)
    }
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum InternalError {
    
    #[error("Token has expired")]
    ExpiredToken,
    
    
    // <Wrapped errors>
    #[error("PoisonError: {0}")]
    PoisonError(String),
    
    #[error("MailboxError: {source}")]
    MailboxError { 
        #[from] source: MailboxError 
    }
    // </Wrapped errors>
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for InternalError
{
    fn from(error: PoisonError<MutexGuard<'_, T>>) -> Self {
        InternalError::PoisonError(error.to_string())
    }
}