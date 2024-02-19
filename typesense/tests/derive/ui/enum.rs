use typesense::Typesense;
use serde::{Serialize, Deserialize};

#[derive(Typesense, Serialize, Deserialize)]
enum SomeEnum {
    First,
    Second,
    Third,
    Other(String),
}

fn main() {}
