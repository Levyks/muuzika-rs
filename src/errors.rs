use std::backtrace::Backtrace;
use std::sync::{MutexGuard, PoisonError};
use actix::{MailboxError, ResponseFuture};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::dtos::responses::ErrorResponse;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UserFacingError {
    #[error("Unknown error")]
    InternalError,
    
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
}

impl ResponseError for UserFacingError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserFacingError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            UserFacingError::InvalidRequestBody { .. } => StatusCode::BAD_REQUEST,
            UserFacingError::RoomNotFound { .. } => StatusCode::NOT_FOUND,
            UserFacingError::UsernameTaken { .. } => StatusCode::CONFLICT,
            UserFacingError::OutOfAvailableCodes => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(ErrorResponse::from_error(self))
    }
}

impl From<MuuzikaError> for UserFacingError
{
    fn from(error: MuuzikaError) -> Self {
        match error {
            MuuzikaError::RoomNotFound { code } => UserFacingError::RoomNotFound { code },
            MuuzikaError::OutOfAvailableCodes => UserFacingError::OutOfAvailableCodes,
            MuuzikaError::UsernameTaken { room_code, username } => UserFacingError::UsernameTaken { room_code, username },
            _ => {
                let backtrace = Backtrace::capture();
                error!("Internal error: {}", error);
                error!("{:?}", backtrace);
                UserFacingError::InternalError
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum MuuzikaError {
    
    #[error("Token has expired")]
    ExpiredToken,

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
    
    // <Wrapped errors>
    #[error("PoisonError: {0}")]
    PoisonError(String),
    
    #[error("MailboxError: {0}")]
    MailboxError(#[from] MailboxError),
    
    #[error("SystemTimeError: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    // </Wrapped errors>
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for MuuzikaError
{
    fn from(error: PoisonError<MutexGuard<'_, T>>) -> Self {
        MuuzikaError::PoisonError(error.to_string())
    }
}

pub type MuuzikaResult<T> = Result<T, MuuzikaError>;
pub type MuuzikaFutureResult<T> = ResponseFuture<MuuzikaResult<T>>;