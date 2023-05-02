use actix::prelude::*;
use tokio::time;
use crate::room::actor::{Room, PlayerEntry};
use crate::room::messages::*;
use crate::dtos::common::RoomDto;
use crate::errors::UserFacingError;

impl Handler<DumpRoom> for Room {
    type Result = Result<RoomDto, UserFacingError>;

    fn handle(&mut self, _msg: DumpRoom, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(RoomDto::from_room(self))
    }
}

impl Handler<CreateToken> for Room {
    type Result = Result<String, UserFacingError>;

    fn handle(&mut self, msg: CreateToken, _ctx: &mut Context<Self>) -> Self::Result {
        self.get_player(&msg.username)?;

        Ok("fake token".to_string())
    }
}


impl Handler<JoinRoom> for Room {
    type Result = Result<String, UserFacingError>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Context<Self>) -> Self::Result {
        if self.players.contains_key(&msg.username) {
            return Err(UserFacingError::UsernameTaken {
                username: msg.username.clone(),
                room_code: self.code.clone()
            });
        }

        let player = PlayerEntry::new(msg.username.clone());
        self.players.insert(msg.username.clone(), player);

        self.handle(CreateToken { username: msg.username }, _ctx)
    }
}



impl Handler<DestroyRoom> for Room {
    type Result = Result<(), UserFacingError>;

    fn handle(&mut self, _msg: DestroyRoom, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
        Ok(())
    }
}

impl Handler<PreConnect> for Room {
    type Result = Result<(), UserFacingError>;

    fn handle(&mut self, msg: PreConnect, ctx: &mut Context<Self>) -> Self::Result {
        self.get_player(&msg.username)?;
        Ok(())
    }
}

impl Handler<Delay> for Room {
    type Result = ResponseFuture<Result<(), UserFacingError>>;
    
    fn handle(&mut self, msg: Delay, ctx: &mut Context<Self>) -> Self::Result {
        
        Box::pin(async move {
            time::sleep(time::Duration::from_millis(msg.0)).await;
            Ok(())
        })
    }
}