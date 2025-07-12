use super::Client; // Use the parent module's Client
use typesense_codegen::apis::{collections_api, Error};
use typesense_codegen::models::{CollectionResponse, CollectionSchema};

// This struct holds a temporary reference to the main client.
// The lifetime parameter `'c` ensures it cannot outlive the Client it borrows from.
pub struct Collections<'c> {
    pub client: &'c Client,
}

// Implement the public methods on the Collections struct.
impl<'c> Collections<'c> {
    /// Retrieve the details of a collection, given its name.
    pub async fn get(&self, collection_name: &str) -> Result<CollectionResponse, Error<collections_api::GetCollectionError>> {
        // It calls back to the generic helper method on the main client.
        let path = format!("/collections/{}", collection_name);
        self.client.get(&path, None).await
    }

    /// When a collection is created, we give it a name and describe the fields.
    pub async fn create(&self, schema: &CollectionSchema) -> Result<CollectionResponse, Error<collections_api::CreateCollectionError>> {
        self.client.post("/collections", schema, None).await
    }

    // ... all other collection-related methods go here ...
}
