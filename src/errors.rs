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
pub enum MuuzikaError {
    #[error("Unknown error")]
    Unknown {
        #[serde(skip_serializing)]
        message: String
    },

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

impl ResponseError for MuuzikaError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MuuzikaError::Unknown { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MuuzikaError::InvalidRequestBody { .. } => StatusCode::BAD_REQUEST,
            MuuzikaError::RoomNotFound { .. } => StatusCode::NOT_FOUND,
            MuuzikaError::UsernameTaken { .. } => StatusCode::CONFLICT,
            MuuzikaError::OutOfAvailableCodes => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST,
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

impl From<MailboxError> for MuuzikaError
{
    fn from(error: MailboxError) -> Self {
        MuuzikaError::Unknown {
            message: format!("MailboxError: {:?}", error)
        }
    }
}

impl From<Canceled> for MuuzikaError
{
    fn from(error: Canceled) -> Self {
        MuuzikaError::Unknown {
            message: format!("Canceled: {:?}", error)
        }
    }
}