use std::time::Instant;
use actix::prelude::*;
use actix_web_actors::ws;
use serde_json::json;
use crate::actors::player::messages::Send;
use crate::actors::player::Player;
use crate::errors::MuuzikaResult;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg)
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
    }
}

impl Handler<Send> for Player {
    type Result = MuuzikaResult<()>;

    fn handle(&mut self, msg: Send, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(json!(msg.0).to_string());
        Ok(())
    }
}