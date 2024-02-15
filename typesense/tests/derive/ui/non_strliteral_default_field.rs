use typesense::Typesense;
use serde::{Serialize, Deserialize};

#[derive(Typesense, Serialize, Deserialize)]
#[typesense(default_sorting_field = company_name)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}

fn main() {}
