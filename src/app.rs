use actix_web::{web, HttpResponse};

use crate::dtos::responses::ErrorResponse;
use crate::endpoints::*;
use crate::errors::MuuzikaError;

pub fn config(cfg: &mut web::ServiceConfig) {
    let json_config = web::JsonConfig::default().error_handler(|err, _| {
        let error = MuuzikaError::InvalidRequestBody {
            message: err.to_string()
        };
        let response = HttpResponse::BadRequest().json(ErrorResponse::from_error(&error));
        actix_web::error::InternalError::from_response(error, response).into()
    });

    cfg.app_data(json_config)
        .service(create_room_endpoint)
        .service(join_room_endpoint)
        .service(destroy_room_endpoint)
        .service(websocket_endpoint);
}