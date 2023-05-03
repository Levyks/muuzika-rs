use actix::Message;
use serde::Serialize;
use crate::dtos::common::RoomDto;
use crate::errors::MuuzikaResult;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum MuuzikaEvent {
    PlayerConnected { username: String },
    PlayerDisconnected { username: String },
    SyncRoom { room: RoomDto },
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<()>")]
pub struct Send(pub MuuzikaEvent);