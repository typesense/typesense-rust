use typesense::Typesense;
use serde::{Serialize, Deserialize};

#[derive(Typesense, Serialize, Deserialize)]
#[typesense(default_sorting_field = "wrong_field")]
struct Company {
    company_name: String,
    num_employees: i32,
    country: String,
}

fn main() {}
