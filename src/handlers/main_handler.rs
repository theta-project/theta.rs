use actix_web::{web, HttpResponseBuilder};

pub fn login(body: &web::BytesMut, res: &mut HttpResponseBuilder) {
    let login_data: Vec<&str> = std::str::from_utf8(body).unwrap().lines().collect();
    let username = login_data.get(0).expect("why was no username parsed");

    println!("username: {}", username);
    println!("{:?}", login_data);


    res.insert_header(("cho-token", "test"));
}
/*
pub fn handle_packet(req: HttpRequest, res: HttpResponseBuilder) {

}*/
