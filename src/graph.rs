use crate::parser::Package;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

pub struct DependencyGraph {
    pub graph: Graph<Package, ()>,
    pub index_map: HashMap<String, NodeIndex>,
}

impl DependencyGraph{
    pub fn from_lockfile(packages: Vec<Package>) -> Self{
        let mut graph = Graph::new();
        let mut index_map = HashMap::new();
        for pkg in packages{
            let key = format!("{} {}", pkg.name, pkg.version);
            let idx = graph.add_node(pkg);
            index_map.insert(key, idx);
        }
        for (_key, &idx) in &index_map{
            let dependencies = graph[idx].dependencies.clone();
            if let Some(dependencies) = dependencies{
                for dep in dependencies{
                    if let Some(&dep_idx) = index_map.get(&dep){
                        graph.add_edge(idx, dep_idx, ());
                    }
                }
            }
        }
        Self{graph, index_map}
    }
}