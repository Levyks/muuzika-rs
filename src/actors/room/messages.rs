use actix::prelude::*;
use crate::dtos::common::RoomDto;
use crate::errors::MuuzikaResult;
use crate::actors::player::Player;

#[derive(Message)]
#[rtype(result = "MuuzikaResult<RoomDto>")]
pub struct DumpRoom;

#[derive(Message)]
#[rtype(result = "MuuzikaResult<String>")]
pub struct CreateToken {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<String>")]
pub struct JoinRoom {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<()>")]
pub struct DestroyRoom;

#[derive(Message)]
#[rtype(result = "MuuzikaResult<RoomDto>")]
pub struct Connect {
    pub username: String,
    pub addr: Addr<Player>
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<()>")]
pub struct Disconnect {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<bool>")]
pub struct PlayerExists {
    pub username: String
}

#[derive(Message)]
#[rtype(result = "MuuzikaResult<()>")]
pub struct Delay(pub u64);