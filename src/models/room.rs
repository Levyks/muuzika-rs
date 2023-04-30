use std::collections::HashMap;
use crate::models::player::Player;


#[derive(Clone)]
pub struct Room {
    pub code: String,
    pub players: HashMap<String, Player>,
    pub leader_username: String
}

impl Room {
    pub fn new(code: String, leader_username: String) -> Room {
        let leader = Player::new(leader_username.clone());

        let mut players = HashMap::new();

        players.insert(leader_username.clone(), leader);

        Room {
            code,
            players,
            leader_username
        }
    }
}