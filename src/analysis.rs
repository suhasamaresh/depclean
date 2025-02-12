use crate::graph::DependancyGraph;
use crate::parser::package;

use semver::{Version, VersionReq};
use std::collections::HashMap;

pub struct DuplicateSet{
    pub name: String, 
    pub versions: vec<String>
}

pub fn find_duplicate(graph: &DependancyGraph) -> Vec<DuplicateSet>{
    let mut packages : HashMap<&str, Vec<&package>> = HashMap::new();
    for node in graph.graph.node_indices(){
        let pkg = &graph.graph[node];
        let entry = packages.entry(&pkg.name).or_default().push(pkg);
    }

    packages.into_iter()
        .filter(|(_, versions)| versions.len() > 1)
        .map(|(name, versions)| DuplicateSet{
            name: name.to_string(),
            versions: versions.iter().map(|pkg| pkg.version.clone()).collect()
        })
        .collect()
}

pub fn fine_optimal(versions: &[String]) -> Option<String> {
    let mut parsed_versions = versions.iter()
        .filtermap(|v| Version::parse(v).ok())
        .collect();
    if parsed_versions.is_empty(){
        return None;
    }
    parsed_versions.sort();

    let latest = parsed_versions.last()?;

    let req_str = format!("^{}", latest);

    let req = VersionReq::parse(&req_str).ok()?;
    if parsed_versions.iter().all(|v| req.matches(v)){
        Some(latest.to_string())
    } else {
        None
    }
    None
}