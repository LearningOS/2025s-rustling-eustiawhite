/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    // 实现 add_edge 方法：边以 (src, dest, weight) 的形式传入
    // 当添加边时，先确保两个节点存在，然后将边添加到两个节点的邻接列表中
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (src, dest, weight) = edge;
        // 如果不存在，则添加节点
        self.add_node(src);
        self.add_node(dest);

        let table = self.adjacency_table_mutable();
        // 在 src 的邻接列表中添加 (dest, weight)
        if let Some(neighbors) = table.get_mut(src) {
            neighbors.push((dest.to_string(), weight));
        }
        // 在 dest 的邻接列表中添加 (src, weight)
        if let Some(neighbors) = table.get_mut(dest) {
            neighbors.push((src.to_string(), weight));
        }
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    // 默认实现 add_node：如果节点不存在则添加，并返回 true，否则返回 false
    fn add_node(&mut self, node: &str) -> bool {
        let table = self.adjacency_table_mutable();
        if table.contains_key(node) {
            false
        } else {
            table.insert(node.to_string(), Vec::new());
            true
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32));
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        // 检查所有期望的边是否都在 graph.edges() 中
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
