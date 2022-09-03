use actix_web::{HttpRequest, HttpResponseBuilder};

pub fn login(req: &HttpRequest, mut res: &HttpResponseBuilder) -> HttpResponseBuilder {
    res.insert_header(("cho-token", "test"));
    *res
}

pub fn handle_packet(req: HttpRequest, res: HttpResponseBuilder) -> HttpResponseBuilder {


    res
}