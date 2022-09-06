use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Responder};
use bytes::BytesMut;
use futures_util::StreamExt as _;

use crate::bancho;


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
    let mut res_body: BytesMut = BytesMut::default();
    match headers.get("osu-token") {
        Some(token) => {
            // try to login
            println!("token was found ({:?}) -- should try and find user", token);
            bancho::handle_packet(&body);
        },
        None => {
            // no token
            res_body = bancho::login(&body, &mut res);
        }
    };
    

    Ok(res.body(res_body))
}
