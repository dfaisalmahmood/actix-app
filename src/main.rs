mod logger;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use logger::configure_log;
use slog::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let log = configure_log();
    let host = "127.0.0.1";
    let port = 8080;

    info!(log, "Starting the server at http://{host}:{port}");

    HttpServer::new(move || {
        App::new()
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::Ok().body("Ok") }),
            )
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind((host, port))?
    .run()
    .await
}
