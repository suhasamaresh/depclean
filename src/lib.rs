// src/lib.rs

// Re-export modules for easier external access.
pub mod parser;
pub mod graph;
pub mod analysis;
pub mod impact;
pub mod report;
pub mod cli;
// Optional: Include if you're supporting npm as well.

use clap::Parser;
use std::fs;

use crate::cli::Cli;
use crate::parser::parse_cargo_lock;
use crate::graph::DependencyGraph;
use crate::analysis::{find_duplicates, find_optimal_version};
use crate::impact::estimate_impact;
use crate::report::{print_report, VersionSuggestion};

/// Runs the main functionality of the depclean tool.
///
/// This function:
/// - Parses command-line arguments using Clap.
/// - Reads the specified lockfile (e.g., Cargo.lock).
/// - Parses the lockfile into a structured format.
/// - Builds a dependency graph from the parsed data.
/// - Analyzes the graph for duplicate dependencies.
/// - Generates version suggestions and estimates potential savings.
/// - Prints a formatted report to standard output.
pub fn run() {
    // Create a Tokio runtime to execute asynchronous code.
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        // Parse command-line arguments.
        let args = Cli::parse();

        // Load the lockfile contents.
        let lockfile_contents = fs::read_to_string(&args.lockfile)
            .expect("Failed to read lockfile");

        // Parse the Cargo.lock file.
        let lockfile = parse_cargo_lock(&lockfile_contents)
            .expect("Failed to parse lockfile");

        // Build the dependency graph from the parsed packages.
        let graph = DependencyGraph::from_lockfile(lockfile.package);

        // Analyze duplicates in the dependency graph.
        let duplicates = find_duplicates(&graph);

        // Generate version suggestions for each duplicate dependency.
        let mut suggestions = Vec::new();
        for dup in &duplicates {
            // Use the duplicate's package name as the crate name.
            let recommended = find_optimal_version(&dup.name, &dup.versions).await
                .unwrap_or_else(|| "No optimal version found".to_string());
            suggestions.push(VersionSuggestion {
                recommended_version: recommended,
                estimated_saving: 10 * (dup.versions.len() - 1), // simplistic calculation
            });
        }

        // Optionally, perform a more detailed impact estimation.
        let _impact = estimate_impact(&duplicates);

        // Print the final report.
        print_report(duplicates, suggestions);
    });
}
