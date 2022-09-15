use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
mod bancho;
mod buf;
mod globals;
mod routes;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //globals::setup_globals();
    let globals = web::Data::new(globals::Globals {
        session_list: Mutex::new(vec![]),
        handled_requests: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(globals.clone())
            .service(routes::bancho_homepage)
            .service(routes::bancho_handler)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
