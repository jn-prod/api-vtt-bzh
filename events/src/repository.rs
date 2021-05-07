extern crate mongodb;
extern crate utils;
extern crate futures;

use mongodb::{Cursor, error::Error, options::FindOptions};
use bson::{Bson, Document};
use utils::db::{connect};
use futures::stream::StreamExt;
use crate::contract::{InsertableEventsReport, EventsReport};

const DB_COLLECTION: &str = "events-report";

pub async fn insert_events_report(report: &InsertableEventsReport) -> Result<Bson, String>{
    let db = connect().await.unwrap();
    let collection = db.collection(&DB_COLLECTION);
    
    match bson::to_bson(&report.clone()) {
        Ok(bson_entity) => match bson::to_document(&bson_entity) {
            Ok(doc) => match collection.insert_one(doc, None).await {
                Ok(res) => Ok(res.inserted_id),
                Err(err) => Err(String::from(format!("Cannot read inserted - err : {}", err))),
            },
            Err(_) => Err(String::from("Failed to create Document"))
        },
        Err(_) => Err(String::from("Failed to create BSON"))
    }
}

pub async fn find_all_events_report() -> Result<Vec<InsertableEventsReport>, Error> {
    let db = connect().await?;
    let collection = db.collection(&DB_COLLECTION);
    let find_options = FindOptions::builder().sort(doc! { "date": -1 }).build();
    let mut cursor: Cursor = collection.find(doc! { }, find_options).await?;

    let mut elements = Vec::new(); 
    while let Some(doc) = cursor.next().await {
        let event: InsertableEventsReport = bson::from_bson(Bson::Document(doc?))?;
        elements.push(event);
    }

    return Ok(elements);
}

pub async fn find_last_events_report() -> Result<Document, Error> {
    let db = connect().await?;
    let collection = db.collection(&DB_COLLECTION);
    // let find_options = FindOptions::builder().sort(doc! { "date": -1 }).build();

    let cursor = collection.find_one(doc! { }, None).await?;

    match cursor {
        Some(doc) => return Ok(doc),
        None => return Ok(doc!{})
    }
}