use std::collections::{LinkedList};
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use actix::Addr;
use dashmap::DashMap;
use crate::actors::room::Room;
use crate::errors::{MuuzikaError, MuuzikaResult};
use crate::spotify::fetcher::SpotifyFetcher;

pub struct AppState {
    spotify_fetcher: Addr<SpotifyFetcher>,
    rooms: DashMap<String, Addr<Room>>,
    available_codes: Arc<Mutex<LinkedList<String>>>,
}

impl AppState {
    pub fn new(spotify_fetcher: Addr<SpotifyFetcher>) -> AppState {
        AppState {
            spotify_fetcher,
            available_codes: Arc::new(Mutex::new(initialize_available_codes(4))),
            rooms: DashMap::new(),
        }
    }

    pub fn get_room_addr(&self, code: &str) -> MuuzikaResult<Addr<Room>> {
        match self.rooms.get(code) {
            Some(room) => Ok(room.value().clone()),
            None => Err(MuuzikaError::RoomNotFound { code: code.to_string() })
        }
    }

    pub fn put_room(&self, code: String, room: Addr<Room>) {
        self.rooms.insert(code, room);
    }

    pub fn remove_room(&self, code: &str) {
        self.rooms.remove(code);
        self.rooms.shrink_to_fit();
    }

    pub fn pop_available_code(&self) -> MuuzikaResult<String> {
        match self.available_codes.lock()?.pop_front() {
            Some(code) => Ok(code),
            None => Err(MuuzikaError::OutOfAvailableCodes)
        }
    }

    pub fn push_available_code(&self, code: String) -> MuuzikaResult<()> {
        self.available_codes.lock()?.push_front(code);
        Ok(())
    }
}

fn initialize_available_codes(code_size: u32) -> LinkedList<String> {
    let mut rng = rand::thread_rng();
    let max_num = 10u32.pow(code_size) - 1;
    let mut numbers: Vec<u32> = (0..=max_num).collect();
    numbers.shuffle(&mut rng);

    numbers
        .into_iter()
        .map(|n| format!("{:04}", n))
        .collect()
}