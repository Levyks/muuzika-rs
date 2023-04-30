use crate::models::{Room};

use std::collections::{LinkedList};
use rand::seq::SliceRandom;
use std::sync::{Mutex, Arc};
use dashmap::DashMap;
use crate::errors::MuuzikaError;

pub struct AppState {
    rooms: DashMap<String, Arc<Mutex<Room>>>,
    available_codes: Mutex<LinkedList<String>>,
}

impl AppState {
    pub fn get_room(&self, code: &str) -> Result<Arc<Mutex<Room>>, MuuzikaError> {
        let room = self.rooms.get(code)
            .ok_or(MuuzikaError::RoomNotFound { code: code.to_string() })?
            .clone();

        Ok(room)
    }

    pub fn new() -> AppState {
        AppState {
            available_codes: Mutex::new(initialize_available_codes(4)),
            rooms: DashMap::new(),
        }
    }

    pub fn put_room(&self, code: String, room_mutex: Arc<Mutex<Room>>) {
        self.rooms.insert(code,  room_mutex);
    }

    pub fn pop_available_code(&self) -> Option<String> {
        self.available_codes.lock().unwrap().pop_front()
    }

    pub fn push_available_code(&self, code: String) {
        self.available_codes.lock().unwrap().push_back(code)
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