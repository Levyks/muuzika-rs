use actix::prelude::*;
use actix_web_actors::ws;
use serde_json::json;
use crate::actors::{
    player::structs::Player,
    room::messages::Connect
};
use crate::actors::player::messages::MuuzikaEvent;
use crate::actors::room::messages::Disconnect;

impl Actor for Player {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_hb_interval(ctx);
        
        let addr = ctx.address();
        
        self.room
            .send(Connect {
                username: self.username.clone(),
                addr,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(Ok(room)) => ctx.text(json!(MuuzikaEvent::SyncRoom { room }).to_string()),
                    _ => ctx.stop(),
                }
            
                fut::ready(())
            }).wait(ctx);
    }
    
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.room.do_send(Disconnect {
            username: self.username.clone(),
        });
        
        Running::Stop
    }
}