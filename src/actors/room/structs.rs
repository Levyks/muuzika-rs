use std::collections::HashMap;
use std::sync::Arc;
use actix::{Actor, Addr, Arbiter};
use crate::actors::player::messages::{MuuzikaEvent, Send};
use crate::actors::player::Player;
use crate::errors::{MuuzikaError, MuuzikaResult};
use crate::state::AppState;

pub struct Room {
    pub code: String,
    pub players: HashMap<String, PlayerRoomEntry>,
    pub leader_username: String,
    pub app_state: Arc<AppState>,
    pub arbiter: Arbiter,
}

impl Room {
    pub fn new(code: String, leader_username: String, app_state: Arc<AppState>, arbiter: Arbiter) -> Room {
        let leader = PlayerRoomEntry::new(leader_username.clone());

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

    pub fn get_player_mut(&mut self, username: &str) -> MuuzikaResult<&mut PlayerRoomEntry> {
        match self.players.get_mut(username) {
            Some(player) => Ok(player),
            None => Err(MuuzikaError::PlayerNotFound {
                username: username.to_string(),
                room_code: self.code.clone()
            })
        }
    }
    
    fn get_all_players_except(&self, username: &str) -> Vec<&PlayerRoomEntry> {
        self.players.values()
            .filter(|p| p.username != username)
            .collect::<Vec<&PlayerRoomEntry>>()
    }
    
    pub fn broadcast_player_connection(&self, username: &str) {
        for player in self.get_all_players_except(username) {
            player.send_event(MuuzikaEvent::PlayerConnected {
                username: username.to_string(),
            });
        }
    }

    pub fn broadcast_player_disconnection(&self, username: &str) {
        for player in self.get_all_players_except(username) {
            player.send_event(MuuzikaEvent::PlayerDisconnected {
                username: username.to_string(),
            });
        }
    }
}

pub struct PlayerRoomEntry {
    pub username: String,
    pub score: u16,
    pub addr: Option<Addr<Player>>,
}

impl PlayerRoomEntry {
    pub fn new(username: String) -> PlayerRoomEntry {
        PlayerRoomEntry {
            username,
            score: 0,
            addr: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.addr.is_some()
    }
    
    pub fn send_event(&self, event: MuuzikaEvent) {
        if let Some(addr) = &self.addr {
            addr.do_send(Send(event));
        }
    }
}



