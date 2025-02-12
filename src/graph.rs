use crate::parser::package;

use petgraph::graph::{Graph, NodeIndex};

use std::collections::HashMap;

pub struct DependancyGraph {
    graph: Graph<package, ()>,
    nodes: HashMap<String, NodeIndex>,
}

impl DependancyGraph{
    pub fn from_lockfile(packages: vec<package>) -> Self{
        let mut graph = Graph::new();
        let mut index_map = HashMap::new();
        for pkg in packages{
            let key = format!("{}-{}", pkg.name, pkg.version);
            let idx = graph.add_node(pkg);
            index_map.insert(key, idx);
        }
        for (key, &idx) in &index_map{
            let pkg = &graph[idx];
            if let Some(dependencies) = &pkg.dependencies{
                for dep in dependencies{
                    if let Some(&dep_idx) = index_map.get(dep){
                        graph.add_edge(idx, dep_idx, ());
                    }
                }
            }
        }
        Self{graph, index_map}
    }
}