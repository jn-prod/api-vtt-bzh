// #[macro_use]
extern crate reqwest;
extern crate select;
extern crate chrono;

use chrono::prelude::*;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use crate::contract::{InstertableEvent, InsertableEventsResponse, EventsUrls};

fn get_client() -> reqwest::Client {
    return reqwest::Client::new();
}

fn is_sortie(urls: &str) -> bool {
    assert_ne!(urls.len(), 0);

    return urls.contains("sortie");
}

fn parse_index_document(response_text: &[u8]) -> Vec<String> {
    let mut events_urls: Vec<String> = Vec::new();

    let document = Document::from_read(response_text).expect("Impossible de parser le document");

    let nodes = document
        .find(Name("tr").descendant(Name("th").descendant(Name("div").descendant(Name("a")))));
    for node in nodes {
        let href = node.attr("href").unwrap();
        let keep_url = is_sortie(href);

        if keep_url {
            events_urls.push(String::from(href))
        }
    }

    return events_urls;
}

fn extract_from_id(document: &Document, id: &str) -> String {
    let mut val: String = "".to_owned();
    for node in document.find(Attr("id", id)) {
        val = format!("{} {}", val, node.text());
    }
    return val.trim_start().trim_end().to_string();
}

fn canceled(document: &Document, id: &str) -> bool {
    let mut val: String = "".to_owned();
    for node in document.find(Attr("id", id)) {
        val = format!("{} {}", val, node.text());
    }

    return val.is_empty() == false;
}

fn parse_event_document(response_text: &[u8], url: &String) -> InstertableEvent {
    let document = Document::from_read(response_text).expect("Impossible de parser le document");

    let event = InstertableEvent {
        name: extract_from_id(&document, "txt_ref_int_nom_2"),
        city: extract_from_id(&document, "txt_ref_int_lieu_2"),
        departement: extract_from_id(&document, "txt_ref_int_dpt_2").parse::<i32>().unwrap(),
        date: extract_from_id(&document, "txt_ref_int_date_2"),
        organisateur: extract_from_id(&document, "txt_ref_int_organisateur_2"),
        hour: extract_from_id(&document, "txt_ref_int_horaires_2"),
        website: extract_from_id(&document, "StyleLien1"),
        price: extract_from_id(&document, "txt_ref_int_prix2"),
        contact: extract_from_id(&document, "txt_ref_int_contacttxt"),
        description: extract_from_id(&document, "txt_ref_int_decription"),
        canceled: canceled(&document, "zone_texte_annule"),
        url: url.to_string(),
    };

    // println!("{:?}", event);

    return event;
}

async fn get_doc_by_year(client: &reqwest::Client, year: &i32) -> String {
    return client
        .get(&format!(
            "https://www.nafix.fr/sorties/vtt/{year}-avenir-56-29-22-35-44-0-0-0-1.html",
            year = year
        ))
        .send()
        .await
        .expect("Could not make the request")
        .text()
        .await
        .expect("Could not read response text");
}

async fn get_event_doc_by_urls(client: &reqwest::Client, url: &str) -> String {
    let clean_url = format!("{}{}", "https://www.nafix.fr", url.replace("../", "/"));

    return client
        .get(clean_url)
        .send()
        .await
        .expect("Could not make the request")
        .text()
        .await
        .expect("Could not read response text");
}

fn get_current_year() -> i32 {
    let now: DateTime<Utc> = Utc::now();
    return now.year();
}

async fn get_events_url(client: &reqwest::Client) -> EventsUrls {
    let current_year = get_current_year();
    let current_year_doc = get_doc_by_year(&client, &current_year).await;
    let current_year_events_urls = parse_index_document(current_year_doc.as_bytes());

    println!("Nous avons trouvé {:?}", current_year_events_urls.len());

    EventsUrls {
        urls: current_year_events_urls,
    }
}

async fn get_event_datas(client: &reqwest::Client, events_urls: &EventsUrls) -> InsertableEventsResponse {
    let mut events: Vec<InstertableEvent> = Vec::new();
    assert!(!events_urls.urls.is_empty());

    for url in &events_urls.urls {
        let event_doc = get_event_doc_by_urls(&client, &url).await;
        let event = parse_event_document(event_doc.as_bytes(), &url);

        events.push(event)
    }

    // let event_doc = get_event_doc_by_urls(&client, &events_urls.urls[0]).await;
    // let event = parse_event_document(event_doc.as_bytes(), &events_urls.urls[0]);
    // events.push(event);

    return InsertableEventsResponse { events };
}

pub async fn get_updated_events() -> InsertableEventsResponse {
    // start reqwest client connection
    let request_client = get_client();

    let events_urls = get_events_url(&request_client).await;

    return get_event_datas(&request_client, &events_urls).await;
}