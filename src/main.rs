use actix_web::{HttpServer, App, web};
use std::sync::Mutex;
mod routes;
mod bancho;
mod buf;
mod session;
mod globals;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    //globals::setup_globals();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(globals::Globals {
                session_list: Mutex::new(vec![]),
                handled_requests: Mutex::new(0)
            }))
            .service(routes::bancho_homepage)
            .service(routes::bancho_handler)
        })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
