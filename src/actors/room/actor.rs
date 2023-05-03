use actix::prelude::*;
use log::{info,error};
use crate::actors::{
    room::structs::Room,
};

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