// tests/impact_tests.rs

use depclean::analysis::DuplicateSet;
use depclean::impact::estimate_impact;

#[test]
fn test_estimate_impact() {
    // Create a duplicate set for testing.
    let duplicates = vec![
        DuplicateSet {
            name: "serde".to_string(),
            versions: vec!["1.0.193".to_string(), "1.0.190".to_string()],
        }
    ];

    let impacts = estimate_impact(&duplicates);
    assert_eq!(impacts.len(), 1);
    let impact = &impacts[0];
    assert_eq!(impact.name, "serde");
    // In our simplistic calculation, potential_savings = (number of versions - 1) * 10.
    assert_eq!(impact.potential_savings, 10);
}
