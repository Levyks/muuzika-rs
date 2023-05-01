use actix::prelude::*;
use crate::dtos::common::RoomDto;
use crate::errors::{MuuzikaError};

#[derive(Message)]
#[rtype(result = "Result<RoomDto, MuuzikaError>")]
pub struct DumpRoom;

#[derive(Message)]
#[rtype(result = "Result<String, MuuzikaError>")]
pub struct CreateToken {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "Result<String, MuuzikaError>")]
pub struct JoinRoom {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "Result<(), MuuzikaError>")]
pub struct DestroyRoom;

#[derive(Message)]
#[rtype(result = "Result<(), MuuzikaError>")]
pub struct PreConnect {
    pub username: String
}

pub struct Delay(pub u64);

impl Message for Delay {
    type Result = Result<(), MuuzikaError>;
}