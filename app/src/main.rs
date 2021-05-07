extern crate actix_rt;
extern crate actix_web;

mod router;

use actix_web::{App, HttpServer, web};
use router::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/jobs")
                .service(job_nafix),
        )
        .service(
            web::scope("/events")
                .service(get_all),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}