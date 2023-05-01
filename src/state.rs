use crate::room::{Room};

use log::{error};

use std::collections::{LinkedList};
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use actix::Addr;
use dashmap::DashMap;
use crate::errors::MuuzikaError;

pub struct AppState {
    rooms: DashMap<String, Addr<Room>>,
    available_codes: Arc<Mutex<LinkedList<String>>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            available_codes: Arc::new(Mutex::new(initialize_available_codes(4))),
            rooms: DashMap::new(),
        }
    }

    pub fn get_room_addr(&self, code: &str) -> Result<Addr<Room>, MuuzikaError> {
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

    pub fn pop_available_code(&self) -> Result<String, MuuzikaError> {
        match self.available_codes.lock() {
            Ok(mut available_codes) => {
                match available_codes.pop_front() {
                    Some(code) => Ok(code),
                    None => Err(MuuzikaError::OutOfAvailableCodes)
                }
            },
            Err(poison_error) => {
                error!("Failed to lock available codes: {}", poison_error);
                Err(MuuzikaError::Unknown { message: poison_error.to_string() })
            }
        }
    }

    pub fn push_available_code(&self, code: String) -> Result<(), MuuzikaError> {
        match self.available_codes.lock() {
            Ok(mut available_codes) => {
                available_codes.push_front(code);
                Ok(())
            },
            Err(poison_error) => {
                error!("Failed to lock available codes: {}", poison_error);
                Err(MuuzikaError::Unknown { message: poison_error.to_string() })
            }
        }
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