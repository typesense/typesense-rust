use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::{env, fs, process::Command};
mod add_vendor_attributes;
mod preprocess_openapi;
mod test_clean;
mod vendor_attributes;

use preprocess_openapi::preprocess_openapi_file;
use test_clean::test_clean;

const SPEC_URL: &str =
    "https://raw.githubusercontent.com/typesense/typesense-api-spec/master/openapi.yml";

// Input spec file, expected in the project root.
const INPUT_SPEC_FILE: &str = "openapi.yml";
const OUTPUT_PREPROCESSED_FILE: &str = "./preprocessed_openapi.yml";
const CUSTOM_TEMPLATES_DIR: &str = "openapi-generator-template"; // Directory containing our custom templates

// Output directory for the generated code.
const OUTPUT_DIR: &str = "typesense_codegen";

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A task runner for the typesense-rust project"
)]
struct Cli {
    /// The list of tasks to run in sequence.
    #[arg(required = true, value_enum)]
    tasks: Vec<Task>,

    /// Arguments to forward to cargo test
    #[arg(last(true))]
    test_args: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "kebab-case")] // Allows us to type `code-gen` instead of `CodeGen`
enum Task {
    /// Generates client code from the spec file using the Docker container.
    CodeGen,
    /// Fetches the latest OpenAPI spec from [the Typesense repository](https://github.com/typesense/typesense-api-spec/blob/master/openapi.yml).
    Fetch,
    /// Preprocesses fetched OpenAPI spec file into a new one
    Preprocess,
    /// Clean up test artifacts, e.g., collections
    TestClean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let rt = tokio::runtime::Runtime::new().unwrap();

    for task in cli.tasks {
        println!("▶️  Running task: {:?}", task);
        match task {
            Task::CodeGen => task_codegen()?,
            Task::Fetch => task_fetch_api_spec()?,
            Task::Preprocess => preprocess_openapi_file(INPUT_SPEC_FILE, OUTPUT_PREPROCESSED_FILE)
                .expect("Preprocess failed, aborting!"),
            Task::TestClean => {
                let test_args = cli.test_args.clone();
                rt.block_on(async move {
                    test_clean(test_args).await;
                });
            }
        }
    }
    Ok(())
}

fn task_fetch_api_spec() -> Result<()> {
    println!("▶️  Running fetch task...");

    println!("  - Downloading spec from {}", SPEC_URL);
    let response =
        reqwest::blocking::get(SPEC_URL).context("Failed to download OpenAPI spec file")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download spec: HTTP {}", response.status());
    }

    let spec_content = response.text()?;
    fs::write(INPUT_SPEC_FILE, spec_content)
        .context(format!("Failed to write spec to {}", INPUT_SPEC_FILE))?;
    println!("  - Spec saved to {}", INPUT_SPEC_FILE);

    println!("✅ Fetch API spec task finished successfully.");

    Ok(())
}

/// Task to generate client code from the OpenAPI spec using a Docker container.
fn task_codegen() -> Result<()> {
    println!("▶️  Running codegen task via Docker...");
    // Get the absolute path to the project's root directory.
    // std::env::current_dir() gives us the directory from which `cargo xtask` was run.
    let project_root = env::current_dir().context("Failed to get current directory")?;

    // Check if the input spec file exists before trying to run Docker.
    let input_spec_path = project_root.join(INPUT_SPEC_FILE);
    if !input_spec_path.exists() {
        anyhow::bail!(
            "Input spec '{}' not found in project root. Please add it before running.",
            INPUT_SPEC_FILE
        );
    }

    // Construct the volume mount string for Docker.
    // Docker needs an absolute path for the volume mount source.
    // to_string_lossy() is used to handle potential non-UTF8 paths gracefully.
    let volume_mount = format!("{}:/local", project_root.to_string_lossy());
    println!("  - Using volume mount: {}", volume_mount);

    // Set up and run the Docker command.
    println!("  - Starting Docker container...");
    let status = Command::new("docker")
        .arg("run")
        .arg("--rm") // Remove the container after it exits
        .arg("-v")
        .arg(volume_mount) // Mount the project root to /local in the container
        .arg("openapitools/openapi-generator-cli")
        .arg("generate")
        .arg("-i")
        .arg(format!("/local/{}", OUTPUT_PREPROCESSED_FILE)) // Input path inside the container
        .arg("-g")
        .arg("rust")
        .arg("-o")
        .arg(format!("/local/{}", OUTPUT_DIR)) // Output path inside the container
        .arg("-t") // specify the template directory
        .arg(format!("/local/{}", CUSTOM_TEMPLATES_DIR))
        .arg("--additional-properties")
        .arg("library=reqwest")
        .arg("--additional-properties")
        .arg("supportMiddleware=true")
        .arg("--additional-properties")
        .arg("useSingleRequestParameter=true")
        .status()
        .context("Failed to execute Docker command. Is Docker installed and running?")?;

    // Check if the command was successful.
    if !status.success() {
        anyhow::bail!("Docker command failed with status: {}", status);
    }

    println!("✅ Codegen task finished successfully.");
    println!("   Generated code is available in '{}'", OUTPUT_DIR);

    // Run cargo fmt after codegen
    println!("▶️  Running cargo fmt...");
    let fmt_status = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .status()
        .context("Failed to run cargo fmt")?;

    if !fmt_status.success() {
        eprintln!("⚠️  cargo fmt failed (check your Rust installation).");
    } else {
        println!("✅ Successfully formatted the code.");
    }

    Ok(())
}
