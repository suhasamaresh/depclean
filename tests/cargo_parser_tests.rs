// tests/cargo_parser_tests.rs

use depclean::parser::parse_cargo_lock;
use depclean::graph::DependencyGraph;
use petgraph::visit::EdgeRef;

#[test]
fn test_parse_cargo_lock_and_build_graph() {
    // A simple Cargo.lock sample with two packages and one dependency edge.
    let sample_lockfile = r#"
[[package]]
name = "serde"
version = "1.0.193"

[[package]]
name = "serde_json"
version = "1.0.108"
dependencies = [
 "serde 1.0.193",
]
"#;

    // Parse the lockfile into a Lockfile struct.
    let lockfile = parse_cargo_lock(sample_lockfile)
        .expect("Failed to parse Cargo.lock sample");

    // There should be exactly two packages.
    assert_eq!(lockfile.package.len(), 2);

    // Build the dependency graph from the parsed packages.
    let dep_graph = DependencyGraph::from_lockfile(lockfile.package);
    assert_eq!(dep_graph.graph.node_count(), 2);

    // Verify that the dependency edge from "serde_json" to "serde" exists.
    let serde_key = "serde 1.0.193".to_string();
    let serde_json_key = "serde_json 1.0.108".to_string();

    let serde_index = dep_graph.index_map.get(&serde_key)
        .expect("Expected package 'serde' not found");
    let serde_json_index = dep_graph.index_map.get(&serde_json_key)
        .expect("Expected package 'serde_json' not found");

    let mut found_edge = false;
    for edge in dep_graph.graph.edges(*serde_json_index) {
        if edge.target() == *serde_index {
            found_edge = true;
            break;
        }
    }
    assert!(found_edge, "Dependency edge from 'serde_json' to 'serde' was not found");
}
