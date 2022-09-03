use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Responder};
use futures_util::StreamExt as _;

use crate::handlers;


#[get("/")]
pub async fn bancho_homepage() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Theta bancho server v{}",
        env!("CARGO_PKG_VERSION")
    ))
}

const MAX_SIZE: usize = 262_144; 

#[post("/")]
pub async fn bancho_handler(req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse, Error>  {
    let headers = req.headers();
    let raw_token = headers.get("osu-token");
    
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let mut res = HttpResponse::Ok(); 

    match headers.get("osu-token") {
        Some(token) => {
            // try to login
            println!("token was found ({:?}) -- should try and find user", token);
        },
        None => {
            // no token
            handlers::main_handler::login(&req, &res);
        }
    };
    

    Ok(res.finish())
}
