// tests/analysis_tests.rs

use depclean::graph::DependencyGraph;
use depclean::analysis::{find_duplicates, find_optimal_version};
use depclean::parser::Package;

#[test]
fn test_find_duplicates() {
    // Create a simulated set of packages with duplicates.
    let packages = vec![
        Package {
            name: "serde".to_string(),
            version: "1.0.193".to_string(),
            dependencies: None,
        },
        Package {
            name: "serde".to_string(),
            version: "1.0.190".to_string(),
            dependencies: None,
        },
        Package {
            name: "serde_json".to_string(),
            version: "1.0.108".to_string(),
            dependencies: Some(vec!["serde 1.0.193".to_string()]),
        },
    ];

    let dep_graph = DependencyGraph::from_lockfile(packages);
    let duplicates = find_duplicates(&dep_graph);
    // We expect one duplicate set for "serde".
    assert_eq!(duplicates.len(), 1);
    let dup = &duplicates[0];
    assert_eq!(dup.name, "serde");
    assert_eq!(dup.versions.len(), 2);
}

#[tokio::test]
async fn test_find_optimal_version_success() {
    // All versions are valid semver, so we expect the highest version to be returned.
    let versions = vec![
        "1.2.3".to_string(),
        "1.5.0".to_string(),
        "1.6.0".to_string(),
    ];
    // The first parameter is the crate name; in these tests we use "dummy"
    // so that the API call may fail and simply omit metadata scoring.
    // The base scoring (from the version number itself) should still select "1.6.0".
    let optimal = find_optimal_version("dummy", &versions).await;
    assert!(optimal.is_some());
    assert_eq!(optimal.unwrap(), "1.6.0");
}

#[tokio::test]
async fn test_find_optimal_version_failure() {
    // Provide only invalid version strings so that parsing fails.
    let versions = vec![
        "invalid".to_string(),
        "also-invalid".to_string(),
    ];
    let optimal = find_optimal_version("dummy", &versions).await;
    assert!(optimal.is_none());
}
