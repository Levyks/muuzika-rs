use std::time::{Duration, Instant};
use actix::{ActorContext, Addr, AsyncContext};
use actix_web_actors::ws;
use crate::actors::player::messages::MuuzikaEvent;
use crate::actors::room::{
    Room,
    messages::Disconnect
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Player {
    pub username: String,
    pub room: Addr<Room>,
    pub hb: Instant,
}

impl Player {
    fn do_hb_check(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        if Instant::now().duration_since(self.hb) <= CLIENT_TIMEOUT {
            ctx.ping(b"");
            return;
        }

        println!("Websocket Client heartbeat failed, disconnecting!");
        self.room.do_send(Disconnect { username: self.username.clone() });
        ctx.stop();
    }

    pub fn start_hb_interval(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, Player::do_hb_check);
    }
    
    pub fn new(username: String, room: Addr<Room>) -> Player {
        Player {
            username,
            room,
            hb: Instant::now(),
        }
    }
}