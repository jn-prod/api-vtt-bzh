extern crate bson;
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub city: String,
    pub departement: i32,
    pub date: String,
    pub organisateur: String,
    pub hour: String,
    pub website: String,
    pub price: String,
    pub contact: String,
    pub description: String,
    pub canceled: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstertableEvent {
    pub name: String,
    pub city: String,
    pub departement: i32,
    pub date: String,
    pub organisateur: String,
    pub hour: String,
    pub website: String,
    pub price: String,
    pub contact: String,
    pub description: String,
    pub canceled: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventsResponse {
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableEventsResponse {
    pub events: Vec<InstertableEvent>,
}

#[derive(Debug)]
pub struct EventsUrls {
    pub urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventsReport {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub report: EventsResponse,
    pub date: String,
    pub kind: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableEventsReport {
    pub report: InsertableEventsResponse,
    pub date: String,
    pub kind: String
}