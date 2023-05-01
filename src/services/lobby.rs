use std::sync::Arc;

use crate::dtos::responses::CreateOrJoinRoomResponse;
use crate::room::{
    Room,
    messages::{CreateToken, DestroyRoom, DumpRoom, JoinRoom}
};
use crate::errors::MuuzikaError;
use crate::room::messages::PreConnect;
use crate::state::AppState;
use crate::websocket::MyWs;

async fn create_room(code: String, leader_username: String, state: Arc<AppState>) -> Result<CreateOrJoinRoomResponse, MuuzikaError> {
    
    let room_addr = Room::create_and_start(code.clone(), leader_username.clone(), state.clone());
    state.put_room(code, room_addr.clone());

    let token = room_addr.send(CreateToken { username: leader_username }).await??;
    let room = room_addr.send(DumpRoom).await??;

    Ok(CreateOrJoinRoomResponse { token, room })
}

pub async fn create_room_with_random_code(leader_username: String, state: Arc<AppState>) -> Result<CreateOrJoinRoomResponse, MuuzikaError> {
    let code = state.pop_available_code()?;

    let cloned_state = state.clone();

    create_room(code.clone(), leader_username, state).await
        .map_err(|err| {
            match cloned_state.push_available_code(code) {
                _ => err
            }
        })
}

pub async fn join_room(code: String, username: String, state: Arc<AppState>) -> Result<CreateOrJoinRoomResponse, MuuzikaError> {
    let room_addr = state.get_room_addr(&code)?;

    let token = room_addr.send(JoinRoom { username: username.clone() }).await??;
    let room = room_addr.send(DumpRoom).await??;

    Ok(CreateOrJoinRoomResponse { token, room })
}

pub async fn destroy_room(code: String, state: Arc<AppState>) -> Result<(), MuuzikaError> {
    let room_addr = state.get_room_addr(&code)?;

    room_addr.send(DestroyRoom).await??;

    Ok(())
}

pub async fn connect(code: String, username: String, state: Arc<AppState>) -> Result<MyWs, MuuzikaError> {
    let room_addr = state.get_room_addr(&code)?;

    room_addr.send(PreConnect {
        username: username.clone()
    }).await??;
    
    Ok(MyWs {
        username,
        room_code: code,
        room_addr
    })
}