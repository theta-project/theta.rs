#[macro_use]
extern crate lazy_static;
use actix_web::{HttpServer, App};
mod routes;
mod bancho;
mod buf;
mod session;
#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    HttpServer::new(|| {
        App::new()
            .service(routes::bancho_homepage)
            .service(routes::bancho_handler)
        })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
