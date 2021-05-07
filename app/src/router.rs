extern crate events;

use actix_web::{get,Responder,HttpResponse,web};
use events::service::{create_events_report, get_events};

#[get("/nafix")]
pub async fn job_nafix() -> impl Responder {
    let events = create_events_report().await;
    HttpResponse::Ok().json(events)
    
}

#[get("/all")]
pub async fn get_all() -> impl Responder {
    let events = get_events().await;
    HttpResponse::Ok().json(events)
}