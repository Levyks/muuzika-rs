use serde::{Deserialize, Serialize};

use crate::dtos::common::RoomDto;
use crate::errors::MuuzikaError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrJoinRoomResponse {
    pub token: String,
    pub room: RoomDto
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: MuuzikaError,
    pub message: String,
}

impl ErrorResponse {
    pub fn from_error(error: &MuuzikaError) -> ErrorResponse {
        ErrorResponse {
            error: error.clone(),
            message: error.to_string()
        }
    }
}

