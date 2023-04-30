use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use muuzika::app::config;

use muuzika::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState::new());

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .configure(config)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}