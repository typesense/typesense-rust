use std::time::Duration;
use typesense::{Client, ExponentialBackoff, models::GetCollectionsParameters};

async fn clean_test_artifacts() {
    let client = Client::builder()
        .nodes(vec!["http://localhost:8108"])
        .api_key("xyz")
        .healthcheck_interval(Duration::from_secs(5))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(3))
        .connection_timeout(Duration::from_secs(3))
        .build()
        .expect("Failed to create Typesense client");

    let collections = client
        .collections()
        .retrieve(GetCollectionsParameters::new())
        .await
        .expect("Get all collections failed!");

    println!("Cleaning up {} test collections...", collections.len());

    for collection in collections.iter() {
        if let Err(err) = client
            .collection_schemaless(&collection.name)
            .delete()
            .await
        {
            eprintln!("Failed to delete {}: {}", collection.name, err);
        } else {
            println!("Deleted {}", collection.name);
        }
    }
    println!("✅ Cleanup complete.");
}

pub async fn test_clean(is_wasm: bool, args: Vec<String>) {
    let status = if is_wasm {
        println!("Running wasm-pack test...");
        std::process::Command::new("wasm-pack")
            .arg("test")
            .arg("--headless")
            .arg("--chrome")
            .args(&args)
            .arg("typesense")
            .status()
            .expect("Failed to run wasm-pack test")
    } else {
        println!("Running cargo test with arguments: {}", args.join(" "));
        std::process::Command::new("cargo")
            .arg("test")
            .args(&args)
            .status()
            .expect("Failed to run cargo test")
    };

    clean_test_artifacts().await;

    // Propagate cargo test exit code
    std::process::exit(status.code().unwrap_or(1));
}
