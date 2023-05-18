// Implements Dijkstra’s Shortest Path Algorithm
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// https://www.youtube.com/watch?v=pVfj6mxhdMw
// sample: https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub type Graph = HashMap<String, HashMap<String, u32>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathNode {
    node: String,
    distance: u32,
    previous: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathTree {
    node: String,
    edges: Vec<ShortestPathNode>,
}

#[derive(Debug, PartialEq)]
pub struct ShortestPath {
    from: String,
    to: String,
    distance: u32,
    path: Vec<String>,
}

pub fn shortest_path_map(graph: &Graph, from: &str, to: &str) -> Option<ShortestPath> {
    if graph.get(from).is_none() || graph.get(to).is_none() {
        return None;
    }

    let shortest_path_tree = build_shortest_path_tree(&graph, &from);
    if shortest_path_tree.is_none() {
        return None;
    }

    let mut path: Vec<String> = Vec::new();
    let edges = shortest_path_tree.unwrap().edges;
    let edge = edges.iter().find(|e| e.node == to).unwrap();
    let distance = edge.distance;
    let mut prev_name = edge.previous.clone();

    path.push(to.to_string());
    while prev_name != from {
        path.push(prev_name.clone());

        let path_record_option = edges.iter().find(|e| e.node == prev_name);
        if path_record_option.is_none() {
            panic!("Previous node not found: {}", prev_name);
        }

        prev_name = path_record_option.unwrap().previous.clone();
    }

    path.push(from.to_string());
    path.reverse();

    Some(ShortestPath {
        from: from.to_string(),
        to: to.to_string(),
        distance: distance,
        path: path,
    })
}

pub fn build_shortest_path_tree(graph: &Graph, source: &str) -> Option<ShortestPathTree> {
    struct PathRecord {
        name: String,
        distance: Option<u32>,
        prev_node: String,
        visited: bool,
    }

    // check if from and to nodes exists in graph
    if graph.get(source).is_none() {
        return None;
    }

    let mut path_table: Vec<PathRecord> = Vec::new();
    for node in graph.keys() {
        let path_record = PathRecord {
            name: node.to_string(),
            distance: None,
            prev_node: "".to_string(),
            visited: false,
        };
        path_table.push(path_record);
    }

    // make starting node distance as zero
    path_table
        .iter_mut()
        .find(|r| r.name == source)
        .unwrap()
        .distance = Some(0);

    loop {
        // check for min distance and not visited
        let mut records: Vec<_> = path_table
            .iter_mut()
            .filter(|r| !r.visited && r.distance.is_some())
            .collect();
        if records.is_empty() {
            break;
        }

        records.sort_by(|a, b| b.distance.cmp(&a.distance));

        let mut cur_record = records.pop().unwrap();

        let node_name = cur_record.name.clone();
        let node_distance = cur_record.distance.unwrap();
        cur_record.visited = true;

        let node = graph.get(&node_name);
        if node.is_none() {
            panic!("The node {} does not exists in the given graph!", node_name);
        }

        let edges = node.unwrap();
        for (edge_name, edge_distance) in edges {
            let path_record = path_table
                .iter_mut()
                .find(|r| !r.visited && r.name == *edge_name);
            if path_record.is_some() {
                let record = path_record.unwrap();
                if record.distance.is_none()
                    || record.distance.unwrap() > (edge_distance + node_distance)
                {
                    record.distance = Some(edge_distance + node_distance);
                    record.prev_node = node_name.clone();
                }
            }
        }
    }

    let mut edges: Vec<ShortestPathNode> = Vec::new();
    for record in path_table {
        let edge = ShortestPathNode {
            node: record.name.clone(),
            previous: record.prev_node.clone(),
            distance: record.distance.unwrap(),
        };
        edges.push(edge);
    }
    edges.sort_by(|a, b| a.node.cmp(&b.node));

    Some(ShortestPathTree {
        node: source.to_string(),
        edges,
    })
}

pub fn generate_test_sample1() -> (Graph, ShortestPathTree) {
    let mut graph = Graph::new();
    add_edge(&mut graph, "0", "1", 4);
    add_edge(&mut graph, "0", "7", 8);
    add_edge(&mut graph, "1", "2", 8);
    add_edge(&mut graph, "1", "7", 11);
    add_edge(&mut graph, "2", "3", 7);
    add_edge(&mut graph, "2", "8", 2);
    add_edge(&mut graph, "2", "5", 4);
    add_edge(&mut graph, "3", "4", 9);
    add_edge(&mut graph, "3", "5", 14);
    add_edge(&mut graph, "4", "5", 10);
    add_edge(&mut graph, "5", "6", 2);
    add_edge(&mut graph, "6", "7", 1);
    add_edge(&mut graph, "6", "8", 6);
    add_edge(&mut graph, "7", "8", 7);
    
    /*
    Graph

        0   1   2   3   4   5   6   7   8
    --------------------------------------
    0 | 0,  4,  0,  0,  0,  0,  0,  8,  0
    1 | 4,  0,  8,  0,  0,  0,  0, 11,  0
    2 | 0,  8,  0,  7,  0,  4,  0,  0,  2
    3 | 0,  0,  7,  0,  9, 14,  0,  0,  0
    4 | 0,  0,  0,  9,  0, 10,  0,  0,  0
    5 | 0,  0,  4, 14, 10,  0,  2,  0,  0
    6 | 0,  0,  0,  0,  0,  2,  0,  1,  6
    7 | 8, 11,  0,  0,  0,  0,  1,  0,  7
    8 | 0,  0,  2,  0,  0,  0,  6,  7,  0

    Shortest-path tree:
    "1"  4 "0"
    "2" 12 "1"
    "3" 19 "2"
    "4" 21 "5"
    "5" 11 "6"
    "6"  9 "7"
    "7"  8 "0"
    "8" 14 "2"

    Minimum distance:
    from 0 to 1 = 4.  0->1
    from 0 to 2 = 12. 0->1->2
    from 0 to 3 = 19. 0->1->2->3
    from 0 to 4 = 21. 0->7->6->5->4
    from 0 to 5 = 11. 0->7->6->5
    from 0 to 6 = 9.  0->7->6
    from 0 to 7 = 8.  0->7
    from 0 to 8 = 14. 0->1->2->8
    */

    let json_value = json!({
        "node": "0",
        "edges": [
            { "node": "0", "distance": 0,  "previous": ""},
            { "node": "1", "distance": 4,  "previous": "0"},
            { "node": "2", "distance": 12, "previous": "1"},
            { "node": "3", "distance": 19, "previous": "2"},
            { "node": "4", "distance": 21, "previous": "5"},
            { "node": "5", "distance": 11, "previous": "6"},
            { "node": "6", "distance": 9,  "previous": "7"},
            { "node": "7", "distance": 8,  "previous": "0"},
            { "node": "8", "distance": 14, "previous": "2"}
        ]
    });

    let shortest_path_tree: ShortestPathTree = serde_json::from_value(json_value).unwrap();

    (graph, shortest_path_tree)
}

pub fn generate_test_sample2() -> (Graph, ShortestPathTree) {
    let mut graph = Graph::new();
    add_edge(&mut graph, "A", "B", 6);
    add_edge(&mut graph, "A", "D", 1);
    add_edge(&mut graph, "B", "D", 2);
    add_edge(&mut graph, "B", "E", 2);
    add_edge(&mut graph, "B", "C", 5);
    add_edge(&mut graph, "C", "E", 5);
    add_edge(&mut graph, "D", "E", 1);

    /*
    Shortest-path tree:
    A 0
    B 3 D
    C 7 E
    D 1 A
    E 2 D

    B = 3, A->D->B
    C = 7, A->D->E->C
    D = 1, A->D
    E = 2, A->D->E
    */

    let result_json = r#" {
        "node": "A",
        "edges": [
            { "node": "A", "distance": 0, "previous": ""},
            { "node": "B", "distance": 3, "previous": "D"},
            { "node": "C", "distance": 7, "previous": "E"},
            { "node": "D", "distance": 1, "previous": "A"},
            { "node": "E", "distance": 2, "previous": "D"}
        ]
    }
    "#;

    let shortest_path_tree: ShortestPathTree = serde_json::from_str(&result_json).unwrap();

    (graph, shortest_path_tree)
}

pub fn generate_test_sample3() -> (Graph, ShortestPathTree) {
    let mut graph = Graph::new();
    add_edge(&mut graph, "A", "B", 4);
    add_edge(&mut graph, "A", "C", 5);
    add_edge(&mut graph, "B", "C", 11);
    add_edge(&mut graph, "B", "D", 9);
    add_edge(&mut graph, "B", "E", 7);
    add_edge(&mut graph, "C", "E", 3);
    add_edge(&mut graph, "D", "E", 13);
    add_edge(&mut graph, "D", "F", 2);
    add_edge(&mut graph, "E", "F", 6);

    /*
    Shortest-path tree:
    A  0
    B  4 A
    C  5 A
    D 13 B
    E  8 C
    F 14 E

    A = 0
    B = 4,  A−>B
    C = 5,  A−>C
    D = 13, A−>B−>D
    E = 8,  A−>C−>E
    F = 14, A−>C−>E−>F
    */

    let json_value = json!({
        "node": "A",
        "edges": [
            { "node": "A", "distance": 0,  "previous": ""},
            { "node": "B", "distance": 4,  "previous": "A"},
            { "node": "C", "distance": 5,  "previous": "A"},
            { "node": "D", "distance": 13, "previous": "B"},
            { "node": "E", "distance": 8,  "previous": "C"},
            { "node": "F", "distance": 14, "previous": "E"}
        ]
    });

    let shortest_path_tree: ShortestPathTree = serde_json::from_value(json_value).unwrap();

    (graph, shortest_path_tree)
}

fn add_edge(graph: &mut Graph, a: &str, b: &str, distance: u32) {
    let a_node = graph.get_mut(a);
    if a_node.is_some() {
        a_node.unwrap().insert(b.to_string(), distance);
    } else {
        let mut edges = HashMap::new();
        edges.insert(b.to_string(), distance);
        graph.insert(a.to_string(), edges);
    }

    let b_node = graph.get_mut(b);
    if b_node.is_some() {
        b_node.unwrap().insert(a.to_string(), distance);
    } else {
        let mut edges = HashMap::new();
        edges.insert(a.to_string(), distance);
        graph.insert(b.to_string(), edges);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_tree_sample1_test() {
        let (graph, result) = generate_test_sample1();
        assert_eq!(graph.len(), 9);
        let option_shortest_path_tree = super::build_shortest_path_tree(&graph, &result.node);
        assert!(option_shortest_path_tree.is_some());
        assert_eq!(option_shortest_path_tree.unwrap(), result);
    }

    #[test]
    fn shortest_tree_sample2_test() {
        let (graph, result) = generate_test_sample2();
        assert_eq!(graph.len(), 5);
        let option_shortest_path_tree = super::build_shortest_path_tree(&graph, &result.node);
        assert!(option_shortest_path_tree.is_some());
        assert_eq!(option_shortest_path_tree.unwrap(), result);
    }

    #[test]
    fn shortest_tree_sample3_test() {
        let (graph, result) = generate_test_sample3();
        assert_eq!(graph.len(), 6);
        let option_shortest_path_tree = super::build_shortest_path_tree(&graph, &result.node);
        assert!(option_shortest_path_tree.is_some());
        assert_eq!(option_shortest_path_tree.unwrap(), result);
    }

    #[test]
    fn shortest_path_test() {
        let (graph,_) = generate_test_sample1();
        assert_eq!(graph.len(), 9);

        let from = "0";
        let mut to = "1";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 4,
                path: ["0", "1"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "2";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 12,
                path: ["0", "1", "2"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "3";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 19,
                path: ["0", "1", "2", "3"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "4";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 21,
                path: ["0", "7", "6", "5", "4"]
                    .iter()
                    .map(|n| n.to_string())
                    .collect()
            }
        );

        to = "5";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 11,
                path: ["0", "7", "6", "5"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "6";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 9,
                path: ["0", "7", "6"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "7";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 8,
                path: ["0", "7"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "8";
        let shortest_path = super::shortest_path_map(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 14,
                path: ["0", "1", "2", "8"].iter().map(|n| n.to_string()).collect()
            }
        );
    }
}
