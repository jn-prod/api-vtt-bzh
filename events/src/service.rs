extern crate chrono;

use chrono::prelude::*;
use bson::{Bson, Document};

use crate::contract::{InsertableEventsReport};
use crate::jobs::{get_updated_events};
use crate::repository::{find_last_events_report, insert_events_report};

pub async fn create_events_report() -> Result<Bson, String> {
    let report = get_updated_events().await;
    let date: DateTime<Utc> = Utc::now();
    let events_report = InsertableEventsReport {
        report,
        date: date.to_string(),
        kind: String::from("nafix-report")
    };
    
    let inserted = insert_events_report(&events_report).await;
    match inserted {
        Ok(doc) => return Ok(bson::to_bson(&doc).unwrap()),
        Err(e) => Err(e)
    }
}

pub async fn get_events() -> Result<Document, String> {
    let results = find_last_events_report().await;

    match results {
        Ok(documents) => return Ok(documents),
        Err(e) => return Err(String::from(format!("Cannot get entities in db. {}", e))),
    }
}