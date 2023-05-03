use actix::prelude::*;
use tokio::time;
use crate::actors::{
    room::{
        structs::{Room, PlayerRoomEntry},
        messages::*
    },
};
use crate::dtos::common::RoomDto;
use crate::errors::{MuuzikaError, MuuzikaFutureResult, MuuzikaResult};

impl Handler<DumpRoom> for Room {
    type Result = MuuzikaResult<RoomDto>;

    fn handle(&mut self, _msg: DumpRoom, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(RoomDto::from_room(self))
    }
}

impl Handler<CreateToken> for Room {
    type Result = MuuzikaResult<String>;

    fn handle(&mut self, msg: CreateToken, _ctx: &mut Context<Self>) -> Self::Result {
        self.get_player_mut(&msg.username)?;

        Ok("fake token".to_string())
    }
}


impl Handler<JoinRoom> for Room {
    type Result = MuuzikaResult<String>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Context<Self>) -> Self::Result {
        if self.players.contains_key(&msg.username) {
            return Err(MuuzikaError::UsernameTaken {
                username: msg.username.clone(),
                room_code: self.code.clone()
            });
        }

        let player = PlayerRoomEntry::new(msg.username.clone());
        self.players.insert(msg.username.clone(), player);

        self.handle(CreateToken { username: msg.username }, _ctx)
    }
}



impl Handler<DestroyRoom> for Room {
    type Result = MuuzikaResult<()>;

    fn handle(&mut self, _msg: DestroyRoom, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
        Ok(())
    }
}

impl Handler<PlayerExists> for Room {
    type Result = MuuzikaResult<bool>;

    fn handle(&mut self, msg: PlayerExists, _ctx: &mut Context<Self>) -> Self::Result {
        match self.get_player_mut(&msg.username) {
            Ok(_) => Ok(true),
            Err(MuuzikaError::PlayerNotFound { .. }) => Ok(false),
            Err(e) => Err(e)
        }
    }
}

impl Handler<Connect> for Room {
    type Result = MuuzikaResult<RoomDto>;

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        let player = self.get_player_mut(&msg.username)?;
        player.addr = Some(msg.addr);
        self.broadcast_player_connection(&msg.username);
        self.handle(DumpRoom, ctx)
    }
}

impl Handler<Disconnect> for Room {
    type Result = MuuzikaResult<()>;

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        let player = self.get_player_mut(&msg.username)?;
        player.addr = None;
        self.broadcast_player_disconnection(&msg.username);
        Ok(())
    }
}

impl Handler<Delay> for Room {
    type Result = MuuzikaFutureResult<()>; 
    
    fn handle(&mut self, msg: Delay, _ctx: &mut Context<Self>) -> Self::Result {
        
        Box::pin(async move {
            time::sleep(time::Duration::from_millis(msg.0)).await;
            Ok(())
        })
    }
}