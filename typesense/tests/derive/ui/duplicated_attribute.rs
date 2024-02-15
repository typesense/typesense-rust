use typesense::Typesense;
use serde::{Serialize, Deserialize};
#[derive(Typesense, Serialize, Deserialize)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    #[typesense(facet)]
    country_code: String,
}

fn main() {}
