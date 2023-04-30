use crate::state::{AppState};
use crate::dtos::requests::CreateOrJoinRoomRequest;
use crate::dtos::responses::CreateOrJoinRoomResponse;

use actix_web::{post, web, Result, HttpResponse, Responder};
use crate::errors::MuuzikaError;
use crate::services::lobby::{create_room, join_room};


#[post("/rooms")]
pub async fn create_room_endpoint(body: web::Json<CreateOrJoinRoomRequest>, data: web::Data<AppState>) -> Result<impl Responder, MuuzikaError> {
    let room_mutex = create_room(body.username.clone(), &data)?;
    Ok(HttpResponse::Ok().json(CreateOrJoinRoomResponse::from_room_mutex(room_mutex)))
}

#[post("/rooms/{code}")]
pub async fn join_room_endpoint(path: web::Path<String>, body: web::Json<CreateOrJoinRoomRequest>, data: web::Data<AppState>) -> Result<impl Responder, MuuzikaError> {
    let code = path.into_inner();
    let room_mutex = join_room(code, body.username.clone(), &data)?;
    Ok(HttpResponse::Ok().json(CreateOrJoinRoomResponse::from_room_mutex(room_mutex)))
}