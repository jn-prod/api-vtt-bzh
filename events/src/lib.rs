#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use(doc)]
extern crate bson;

pub mod contract;
pub mod service;
pub mod repository;
mod jobs;