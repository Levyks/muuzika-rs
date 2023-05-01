use std::collections::HashMap;
use std::sync::Arc;
use actix::{Actor, Addr, Arbiter, Context};
use log::{error, info};
use crate::errors::MuuzikaError;
use crate::state::AppState;

pub struct Room {
    pub code: String,
    pub players: HashMap<String, PlayerEntry>,
    pub leader_username: String,
    app_state: Arc<AppState>,
    arbiter: Arbiter,
}

impl Room {
    pub fn new(code: String, leader_username: String, app_state: Arc<AppState>, arbiter: Arbiter) -> Room {
        let leader = PlayerEntry::new(leader_username.clone());

        let mut players = HashMap::new();

        players.insert(leader_username.clone(), leader);

        Room {
            code,
            players,
            leader_username,
            app_state,
            arbiter,
        }
    }
    
    pub fn create_and_start(code: String, leader_username: String, app_state: Arc<AppState>) -> Addr<Room> {
        let arbiter = Arbiter::new();
        let handle = arbiter.handle();
        let room = Room::new(code, leader_username, app_state, arbiter);
        Room::start_in_arbiter(&handle, move |_| room)
    }
    
    pub fn get_player(&self, username: &str) -> Result<&PlayerEntry, MuuzikaError> {
        match self.players.get(username) {
            Some(player) => Ok(player),
            None => Err(MuuzikaError::PlayerNotFound { 
                username: username.to_string(),
                room_code: self.code.clone()
            })
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Actor for room {} is started", self.code);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Actor for room {} is stopped", self.code);
        self.app_state.remove_room(&self.code);
        match self.app_state.push_available_code(self.code.clone()) {
            Ok(_) => info!("Room code {} is available again", self.code),
            Err(error) => error!("Failed to push room code {} back to available codes: {}", self.code, error)
        }
        self.arbiter.stop();
    }
}

pub struct PlayerEntry {
    pub username: String,
    pub score: u16
}

impl PlayerEntry {
    pub fn new(username: String) -> PlayerEntry {
        PlayerEntry {
            username,
            score: 0
        }
    }
}


