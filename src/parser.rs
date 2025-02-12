use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lockfile {
    pub package: Vec<Package>,
}

#[derive(Debug, Deserialize)]
pub struct package {
    pub name: String, 
    pub version: String,
    pub dependencies: Option<Vec<String>>,
}

pub fn parse_cargo_lock(contents: &str) -> Result<Lockfile, toml::de::Error> {
    toml::from_str(contents)
} 
