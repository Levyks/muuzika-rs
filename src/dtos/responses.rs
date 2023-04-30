use std::sync::{Arc, Mutex};
use serde::Serialize;

use crate::dtos::common::RoomDto;
use crate::errors::MuuzikaError;
use crate::models::{Room};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrJoinRoomResponse {
    pub token: String,
    pub room: RoomDto
}

impl CreateOrJoinRoomResponse {
    pub fn from_room(room: &Room) -> CreateOrJoinRoomResponse {
        CreateOrJoinRoomResponse {
            token: "foo".into(),
            room: RoomDto::from_room(room)
        }
    }

    pub fn from_room_mutex(room_mutex: Arc<Mutex<Room>>) -> Result<CreateOrJoinRoomResponse, MuuzikaError> {
        let room = room_mutex.lock()?;
        Ok(CreateOrJoinRoomResponse::from_room(&room))
    }
}

#[derive(Serialize)]
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

