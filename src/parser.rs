use serde::Deserialize;

//public struct the defines the entire lockfile 
#[derive(Debug, Deserialize)]
pub struct Lockfile {
    pub package: Vec<Package>,
}

//defines a package strut containing the name, version, and dependencies of a package
#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String, 
    pub version: String,
    pub dependencies: Option<Vec<String>>,
}

//parses the contents of the cargo.lock file and returns a Lockfile struct
pub fn parse_cargo_lock(contents: &str) -> Result<Lockfile, toml::de::Error> {
    toml::from_str(contents)
} 
