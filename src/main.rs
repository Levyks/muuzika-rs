mod models;
mod endpoints;
mod state;
mod dtos;
mod services;
mod errors;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use env_logger::Env;

use crate::endpoints::{create_room_endpoint, join_room_endpoint};
use crate::dtos::responses::ErrorResponse;
use crate::errors::MuuzikaError;
use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState::new());

    let json_config = web::JsonConfig::default().error_handler(|err, _| {
        let error = MuuzikaError::InvalidRequestBody {
            message: err.to_string()
        };
        let response = HttpResponse::BadRequest().json(ErrorResponse::from_error(&error));
        actix_web::error::InternalError::from_response(error, response).into()
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .app_data(json_config.clone())
            .service(create_room_endpoint)
            .service(join_room_endpoint)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}