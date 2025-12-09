//! # Typesense generic models
mod document_index_parameters;
mod multi_search;
mod scoped_key_parameters;

pub use document_index_parameters::*;
pub use scoped_key_parameters::*;
pub use typesense_codegen::{
    apis::{analytics_api::GetAnalyticsEventsParams, operations_api::TakeSnapshotParams},
    models::{curation_rule::Match as CurationRuleMatch, *},
};

pub use multi_search::MultiSearchBody;
