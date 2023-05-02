use actix::prelude::*;
use crate::dtos::common::RoomDto;
use crate::errors::{UserFacingError};

#[derive(Message)]
#[rtype(result = "Result<RoomDto, UserFacingError>")]
pub struct DumpRoom;

#[derive(Message)]
#[rtype(result = "Result<String, UserFacingError>")]
pub struct CreateToken {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "Result<String, UserFacingError>")]
pub struct JoinRoom {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "Result<(), UserFacingError>")]
pub struct DestroyRoom;

#[derive(Message)]
#[rtype(result = "Result<(), UserFacingError>")]
pub struct PreConnect {
    pub username: String
}

pub struct Delay(pub u64);

impl Message for Delay {
    type Result = Result<(), UserFacingError>;
}