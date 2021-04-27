use typesense::Document;
use serde::{Serialize, Deserialize};

#[derive(Document, Serialize, Deserialize)]
enum SomeEnum {
    First,
    Second,
    Third,
    Other(String),
}

fn main() {}
