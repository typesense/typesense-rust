use typesense::Document;
use serde::{Serialize, Deserialize};

#[derive(Document, Serialize, Deserialize)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facets)]
    country: String,
    keywords: Option<Vec<String>>,
}

fn main() {}
