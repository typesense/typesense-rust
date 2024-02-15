use typesense::Typesense;
use serde::{Serialize, Deserialize};

#[derive(Typesense, Serialize, Deserialize)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facets)]
    country: String,
    keywords: Option<Vec<String>>,
}

fn main() {}
