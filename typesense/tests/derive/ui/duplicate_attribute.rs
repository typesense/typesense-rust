use serde::{Deserialize, Serialize};
use typesense::Typesense;
#[derive(Typesense, Serialize, Deserialize)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet, sort, facet)]
    country_code: String,
}

fn main() {}
