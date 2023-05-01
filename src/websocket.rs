use actix::prelude::*;
use actix_web_actors::ws;
use log::{error, info};
use serde_json::json;
use crate::room::messages::{Delay, DumpRoom};
use crate::room::Room;

pub struct MyWs {
    pub username: String,
    pub room_code: String,
    pub room_addr: Addr<Room>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Actor for player {} in room {} is started", self.username, self.room_code);
        ctx.text("Hello world!");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if text == "sync" {
                    self.room_addr.send(DumpRoom)
                        .into_actor(self)
                        .then(|res, _, ctx| {
                            match res {
                                Ok(room) => {
                                    ctx.text(json!(room).to_string());
                                }
                                Err(err) => {
                                    error!("Error: {}", err);
                                    ctx.text("error");
                                }
                            }
                            fut::ready(())
                        })
                        .spawn(ctx);
                }
                else if text == "delay" {
                    self.room_addr.send(Delay(10000))
                        .into_actor(self)
                        .then(|res, _, ctx| {
                            match res {
                                Ok(_) => {
                                    ctx.text("delayed");
                                }
                                Err(err) => {
                                    error!("Error: {}", err);
                                    ctx.text("error");
                                }
                            }
                            fut::ready(())
                        })
                        .spawn(ctx);
                } 
                else {
                    ctx.text(text);
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
