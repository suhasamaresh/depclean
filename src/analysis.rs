// src/analysis.rs

use crate::graph::DependencyGraph;
use crate::parser::Package;
use semver::Version;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use reqwest;

// -----------------------------
// Duplicate Detection
// -----------------------------
pub struct DuplicateSet {
    pub name: String,
    pub versions: Vec<String>,
}

pub fn find_duplicates(graph: &DependencyGraph) -> Vec<DuplicateSet> {
    let mut packages: HashMap<&str, Vec<&Package>> = HashMap::new();
    for node in graph.graph.node_indices() {
        let pkg = &graph.graph[node];
        packages.entry(&pkg.name)
            .or_default()
            .push(pkg);
    }
    packages.into_iter()
        .filter(|(_, versions)| versions.len() > 1)
        .map(|(name, versions)| DuplicateSet {
            name: name.to_string(),
            versions: versions.iter().map(|p| p.version.clone()).collect(),
        })
        .collect()
}

// -----------------------------
// Version Metadata (Crates.io Only)
// -----------------------------

// Structures to deserialize the Cargo registry API response.
#[derive(Deserialize)]
struct CrateVersion {
    num: String,
    created_at: String,
    downloads: u64,
    yanked: bool,
}

#[derive(Deserialize)]
struct VersionsResponse {
    versions: Vec<CrateVersion>,
}

/// Holds metadata fetched from the Cargo registry.
/// Only the following metrics are stored:
/// - version number
/// - publication date
/// - download count
/// - yanked status
#[derive(Debug)]
pub struct VersionMetadata {
    pub version: Version,
    pub published_at: DateTime<Utc>,
    pub download_count: u64,
    pub is_yanked: bool,
}

impl VersionMetadata {
    /// Computes a score based on:
    /// - The semantic version (with weighted major, minor, and patch components)
    /// - The publication age (penalizing versions that are too new or too old)
    /// - The download count (using a logarithmic boost)
    /// - A heavy penalty if the version is yanked
    pub fn score(&self) -> f64 {
        let now = Utc::now();
        let age_days = (now - self.published_at).num_days() as f64;
        
        // Base score from version components.
        let mut score = self.version.major as f64 * 1000.0 +
                        self.version.minor as f64 * 10.0 +
                        self.version.patch as f64;
        
        // Penalize very new versions (< 14 days)
        if age_days < 14.0 {
            score -= (14.0 - age_days) * 5.0;
        }
        
        // Penalize old versions (> 365 days)
        if age_days > 365.0 {
            score -= (age_days - 365.0) / 30.0;
        }
        
        // Boost based on download count (using log10)
        score += (self.download_count as f64).log10() * 10.0;
        
        // Heavily penalize yanked versions
        if self.is_yanked {
            score -= 10000.0;
        }
        
        score
    }
}

/// Fetches metadata for a given version of a crate from the Cargo registry.
async fn fetch_version_metadata_api(crate_name: &str, version_str: &str) -> Option<VersionMetadata> {
    // Construct the API URL for the crate versions.
    let url = format!("https://crates.io/api/v1/crates/{}/versions", crate_name);
    
    // Perform the HTTP GET request.
    let resp = reqwest::get(&url).await.ok()?;
    let versions_response: VersionsResponse = resp.json().await.ok()?;
    
    // Find the version data matching the requested version.
    let version_data = versions_response.versions.into_iter()
        .find(|v| v.num == version_str)?;
    
    // Parse the publication date (RFC3339 format).
    let published_at = DateTime::parse_from_rfc3339(&version_data.created_at)
        .ok()?
        .with_timezone(&Utc);
    
    Some(VersionMetadata {
        version: Version::parse(version_str).ok()?,
        published_at,
        download_count: version_data.downloads,
        is_yanked: version_data.yanked,
    })
}

// -----------------------------
// Optimal Version Selection
// -----------------------------

/// Asynchronously selects the optimal version based solely on Cargo registry metrics.
/// For each candidate version, the following steps are taken:
/// 1. Compute a base score from the version components (major, minor, patch),
///    with an additional bonus for stable (non pre-release) versions.
/// 2. Fetch real metadata from crates.io and incorporate its score.
/// 3. Return the version with the highest overall score.
pub async fn find_optimal_version(crate_name: &str, versions: &[String]) -> Option<String> {
    let mut version_scores: Vec<(String, f64)> = Vec::new();

    for version_str in versions {
        // Parse the version.
        let version = match Version::parse(version_str) {
            Ok(v) => v,
            Err(_) => continue,
        };
        
        let mut score = 0.0;
        // Bonus for stable versions (i.e. no pre-release components).
        if version.pre.is_empty() {
            score += 1000.0;
        }
        // Add weighted version components.
        score += version.major as f64 * 10000.0;
        score += version.minor as f64 * 100.0;
        score += version.patch as f64;
        
        // Fetch metadata from Cargo registry and add its score.
        if let Some(metadata) = fetch_version_metadata_api(crate_name, version_str).await {
            score += metadata.score();
        }
        
        version_scores.push((version_str.clone(), score));
    }

    // Sort the versions by descending score.
    version_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    version_scores.first().map(|(v, _)| v.clone())
}

