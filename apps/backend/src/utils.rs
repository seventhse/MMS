use actix_http::h1;
use actix_web::dev;

pub fn bytes_to_payload(buf: actix_web::web::Bytes) -> dev::Payload {
    let (_, mut pl) = h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}
