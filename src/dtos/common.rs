use serde::{Serialize, Deserialize};

use crate::room::{Room, PlayerEntry};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDto {
    pub username: String,
    pub score: u16,
    pub is_connected: bool,
}

impl PlayerDto {
    pub fn from_player(player: &PlayerEntry) -> PlayerDto {
        PlayerDto {
            username: player.username.clone(),
            score: player.score,
            is_connected: false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomDto {
    pub code: String,
    pub players: Vec<PlayerDto>,
    pub leader_username: String,
}

impl RoomDto {
    pub fn from_room(room: &Room) -> RoomDto {
        RoomDto {
            code: room.code.clone(),
            players: room.players.iter().map(|(_, player)| PlayerDto::from_player(player)).collect(),
            leader_username: room.leader_username.clone()
        }
    }
}