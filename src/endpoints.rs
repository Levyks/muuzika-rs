use actix_web::{get, post, delete, web, Result, HttpResponse, Responder, HttpRequest};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::state::{AppState};
use crate::dtos::requests::CreateOrJoinRoomRequest;
use crate::errors::MuuzikaError;
use crate::services::lobby::{create_room_with_random_code, join_room, destroy_room, connect};
use crate::websocket::MyWs;


#[post("/rooms")]
pub async fn create_room_endpoint(body: web::Json<CreateOrJoinRoomRequest>, data: web::Data<AppState>) -> Result<impl Responder, MuuzikaError> {
    let response = create_room_with_random_code(body.username.clone(), data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/rooms/{code}")]
pub async fn join_room_endpoint(path: web::Path<String>, body: web::Json<CreateOrJoinRoomRequest>, data: web::Data<AppState>) -> Result<impl Responder, MuuzikaError> {
    let code = path.into_inner();
    let response = join_room(code, body.username.clone(), data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/rooms/{code}")]
pub async fn destroy_room_endpoint(path: web::Path<String>, data: web::Data<AppState>) -> Result<impl Responder, MuuzikaError> {
    let code = path.into_inner();
    destroy_room(code, data.into_inner()).await?;
    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct Info {
    username: String,
    room_code: String,
}

#[get("/ws")]
pub async fn websocket_endpoint(req: HttpRequest, stream: web::Payload, info: web::Query<Info>, data: web::Data<AppState>) -> Result<HttpResponse, MuuzikaError> {
    
    let ws = connect(info.room_code.clone(), info.username.clone(), data.into_inner()).await?;
    
    ws::start(ws, &req, stream)
        .map_err(|e| MuuzikaError::Unknown { message: e.to_string() })
}