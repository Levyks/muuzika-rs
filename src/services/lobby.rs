use std::sync::{Arc, Mutex};
use crate::models::{Room, Player};
use crate::errors::MuuzikaError;
use crate::state::AppState;

pub fn create_room(leader_username: String, state: &AppState) -> Result<Arc<Mutex<Room>>, MuuzikaError> {
    let code = state.pop_available_code();

    let code = match code {
        Some(code) => code,
        None => return Err(MuuzikaError::OutOfAvailableCodes)
    };

    let room = Room::new(code.clone(), leader_username.clone());
    let room_mutex = Arc::new(Mutex::new(room));
    state.put_room(code.clone(), room_mutex.clone());

    Ok(room_mutex)
}

pub fn join_room(code: String, username: String, state: &AppState) -> Result<Arc<Mutex<Room>>, MuuzikaError> {
    let room_mutex = state.get_room(&code)?;
    let cloned_room_mutex = room_mutex.clone();
    let mut room = room_mutex.lock()?;

    if room.players.contains_key(&username) {
        return Err(MuuzikaError::UsernameTaken);
    }

    let player = Player::new(username.clone());
    room.players.insert(username, player);

    Ok(cloned_room_mutex)
}



