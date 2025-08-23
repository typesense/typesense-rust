use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::env;
use std::fs;
use std::process::Command;
mod preprocess_openapi;
use preprocess_openapi::preprocess_openapi_file;

const SPEC_URL: &str =
    "https://raw.githubusercontent.com/typesense/typesense-api-spec/master/openapi.yml";

// Input spec file, expected in the project root.
const INPUT_SPEC_FILE: &str = "openapi.yml";
const OUTPUT_PREPROCESSED_FILE: &str = "./preprocessed_openapi.yml";

// Output directory for the generated code.
const OUTPUT_DIR: &str = "typesense_codegen";

// 1. Define the command-line interface using clap's derive macros
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
}

// 2. Define the available tasks as a simple enum
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "kebab-case")] // Allows user to type `code-gen` instead of `CodeGen`
enum Task {
    /// Fetches the latest OpenAPI spec from the Typesense repository.
    Fetch,
    /// Generates client code from the spec file using the Docker container.
    CodeGen,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for task in cli.tasks {
        println!("▶️  Running task: {:?}", task);
        match task {
            Task::Fetch => task_fetch_api_spec()?,
            Task::CodeGen => task_codegen()?,
        }
    }
    Ok(())
}

fn task_fetch_api_spec() -> Result<()> {
    println!("▶️  Running codegen task...");

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

    println!("Preprocessing the Open API spec file...");
    preprocess_openapi_file(INPUT_SPEC_FILE, OUTPUT_PREPROCESSED_FILE)
        .expect("Preprocess failed, aborting!");
    // 1. Get the absolute path to the project's root directory.
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

    // 2. Construct the volume mount string for Docker.
    // Docker needs an absolute path for the volume mount source.
    // to_string_lossy() is used to handle potential non-UTF8 paths gracefully.
    let volume_mount = format!("{}:/local", project_root.to_string_lossy());
    println!("  - Using volume mount: {}", volume_mount);

    // 4. Set up and run the Docker command.
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
        .arg("--additional-properties")
        .arg("library=reqwest")
        .arg("--additional-properties")
        .arg("supportMiddleware=true")
        .arg("--additional-properties")
        .arg("useSingleRequestParameter=true")
        // .arg("--additional-properties")
        // .arg("useBonBuilder=true")
        .status()
        .context("Failed to execute Docker command. Is Docker installed and running?")?;

    // 5. Check if the command was successful.
    if !status.success() {
        anyhow::bail!("Docker command failed with status: {}", status);
    }

    println!("✅ Codegen task finished successfully.");
    println!("   Generated code is available in '{}'", OUTPUT_DIR);
    Ok(())
}
