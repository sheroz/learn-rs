// Implements Dijkstra’s Shortest Path Algorithm
// Useful resources:
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// https://www.youtube.com/watch?v=pVfj6mxhdMw
// https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPath {
    from: String,
    to: String,
    distance: u32,
    path: Vec<String>,
}

pub fn shortest_path_from_tree(
    to: &str,
    shortest_path_tree: &ShortestPathTree,
) -> Option<ShortestPath> {
    let mut path: Vec<String> = Vec::new();
    let edges = &shortest_path_tree.edges;
    let from = shortest_path_tree.node.clone();
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
        distance,
        path,
    })
}

pub fn shortest_path(graph: &Graph, from: &str, to: &str) -> Option<ShortestPath> {
    if graph.get(from).is_none() || graph.get(to).is_none() {
        return None;
    }

    let shortest_path_tree = shortest_path_tree(&graph, &from);
    if shortest_path_tree.is_none() {
        return None;
    }

    shortest_path_from_tree(&to, &shortest_path_tree.unwrap())
}

pub fn shortest_path_tree(graph: &Graph, source: &str) -> Option<ShortestPathTree> {
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

pub fn generate_test_sample1() -> (Graph, ShortestPathTree, Vec<ShortestPath>) {
    let input_graph_edges = [
        ("A", "B", 4),
        ("A", "H", 8),
        ("B", "C", 8),
        ("B", "H", 11),
        ("C", "D", 7),
        ("C", "I", 2),
        ("C", "F", 3),
        ("D", "E", 9),
        ("D", "F", 14),
        ("E", "F", 10),
        ("F", "G", 2),
        ("G", "H", 1),
        ("G", "I", 6),
        ("H", "I", 7),
    ];

    let mut graph = Graph::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge(&mut graph, x.0, x.1, x.2));

    /*
    Graph
        A   B   C   D   E   F   G   H   I
    --------------------------------------
    A | 0,  4,  0,  0,  0,  0,  0,  8,  0
    B | 4,  0,  8,  0,  0,  0,  0, 11,  0
    C | 0,  8,  0,  7,  0,  4,  0,  0,  2
    D | 0,  0,  7,  0,  9, 14,  0,  0,  0
    E | 0,  0,  0,  9,  0, 10,  0,  0,  0
    F | 0,  0,  4, 14, 10,  0,  2,  0,  0
    G | 0,  0,  0,  0,  0,  2,  0,  1,  6
    H | 8, 11,  0,  0,  0,  0,  1,  0,  7
    I | 0,  0,  2,  0,  0,  0,  6,  7,  0

    Shortest-path tree:
    "B"  4 "A"
    "C" 12 "B"
    "D" 19 "C"
    "E" 21 "F"
    "F" 11 "G"
    "G"  9 "H"
    "H"  8 "A"
    "I" 14 "C"

    Minimum distance from A:
    B = 4,  A->B
    C = 12, A->B->C
    D = 19, A->B->C->D
    E = 21, A->H->G->F->E
    F = 11, A->H->G->F
    G = 9,  A->H->G
    H = 8,  A->H
    I = 14, A->B->C->I
    */

    let input_shortest_path_tree = [
        ("A", 0, ""),
        ("B", 4, "A"),
        ("C", 12, "B"),
        ("D", 19, "C"),
        ("E", 21, "F"),
        ("F", 11, "G"),
        ("G", 9, "H"),
        ("H", 8, "A"),
        ("I", 14, "C"),
    ];

    let mut shortest_path_edges: Vec<ShortestPathNode> =
        Vec::with_capacity(input_shortest_path_tree.len());
    input_shortest_path_tree.iter().for_each(|x| {
        shortest_path_edges.push(ShortestPathNode {
            node: x.0.to_string(),
            distance: x.1,
            previous: x.2.to_string(),
        })
    });

    let expected_shortest_path_tree = ShortestPathTree {
        node: "A".to_string(),
        edges: shortest_path_edges,
    };

    let input_shortest_paths = [
        ("A", "B", 4, vec!["A", "B"]),
        ("A", "C", 12, vec!["A", "B", "C"]),
        ("A", "D", 19, vec!["A", "B", "C", "D"]),
        ("A", "E", 21, vec!["A", "H", "G", "F", "E"]),
        ("A", "F", 11, vec!["A", "H", "G", "F"]),
        ("A", "G", 9, vec!["A", "H", "G"]),
        ("A", "H", 8, vec!["A", "H"]),
        ("A", "I", 14, vec!["A", "B", "C", "I"]),
    ];

    let mut expected_shortest_paths: Vec<ShortestPath> =
        Vec::with_capacity(input_shortest_paths.len());
    input_shortest_paths.iter().for_each(|x| {
        expected_shortest_paths.push(ShortestPath {
            from: x.0.to_string(),
            to: x.1.to_string(),
            distance: x.2,
            path: x.3.iter().map(|v| v.to_string()).collect(),
        })
    });

    (graph, expected_shortest_path_tree, expected_shortest_paths)
}

pub fn generate_test_sample2() -> (Graph, ShortestPathTree, Vec<ShortestPath>) {
    let input_graph_edges = [
        ("A", "B", 6),
        ("A", "D", 1),
        ("B", "D", 2),
        ("B", "E", 2),
        ("B", "C", 5),
        ("C", "E", 5),
        ("D", "E", 1),
    ];

    let mut graph = Graph::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge(&mut graph, x.0, x.1, x.2));

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

    let input_shortest_path_tree = r#"
        {
            "node": "A",
            "edges": [
                {"node": "A", "distance": 0, "previous": "" },
                {"node": "B", "distance": 3, "previous": "D"},
                {"node": "C", "distance": 7, "previous": "E"},
                {"node": "D", "distance": 1, "previous": "A"},
                {"node": "E", "distance": 2, "previous": "D"}
            ]
        }
    "#;

    let expected_shortest_path_tree: ShortestPathTree =
        serde_json::from_str(&input_shortest_path_tree).unwrap();

    let input_shortest_paths = r#"
        [
            {"from": "A", "to": "B", "distance": 3, "path": ["A", "D", "B"]},
            {"from": "A", "to": "C", "distance": 7, "path": ["A", "D", "E", "C"]},
            {"from": "A", "to": "D", "distance": 1, "path": ["A", "D"]},
            {"from": "A", "to": "E", "distance": 2, "path": ["A", "D", "E"]}
        ]
    "#;

    let expected_shortest_paths: Vec<ShortestPath> =
        serde_json::from_str(&input_shortest_paths).unwrap();

    (graph, expected_shortest_path_tree, expected_shortest_paths)
}

pub fn generate_test_sample3() -> (Graph, ShortestPathTree, Vec<ShortestPath>) {
    let input_graph_edges = [
        ("A", "B", 4),
        ("A", "C", 5),
        ("B", "C", 11),
        ("B", "D", 9),
        ("B", "E", 7),
        ("C", "E", 3),
        ("D", "E", 13),
        ("D", "F", 2),
        ("E", "F", 6),
    ];

    let mut graph = Graph::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge(&mut graph, x.0, x.1, x.2));

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

    let input_shortest_path_tree = json!({
        "node": "A",
        "edges": [
            {"node": "A", "distance": 0,  "previous": ""},
            {"node": "B", "distance": 4,  "previous": "A"},
            {"node": "C", "distance": 5,  "previous": "A"},
            {"node": "D", "distance": 13, "previous": "B"},
            {"node": "E", "distance": 8,  "previous": "C"},
            {"node": "F", "distance": 14, "previous": "E"}
        ]
    });

    let expected_shortest_path_tree: ShortestPathTree =
        serde_json::from_value(input_shortest_path_tree).unwrap();

    let input_shortest_paths = json!([
        {"from": "A", "to": "B", "distance": 4,  "path": ["A", "B"]},
        {"from": "A", "to": "C", "distance": 5,  "path": ["A", "C"]},
        {"from": "A", "to": "D", "distance": 13, "path": ["A", "B", "D"]},
        {"from": "A", "to": "E", "distance": 8,  "path": ["A", "C", "E"]},
        {"from": "A", "to": "F", "distance": 14, "path": ["A", "C", "E", "F"]}
    ]);

    let expected_shortest_paths: Vec<ShortestPath> =
        serde_json::from_value(input_shortest_paths).unwrap();

    (graph, expected_shortest_path_tree, expected_shortest_paths)
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
        let (graph, expected_shortest_path_tree, expected_shortest_paths) = generate_test_sample1();
        assert_eq!(graph.len(), 9);
        let option_shortest_path_tree =
            super::shortest_path_tree(&graph, &expected_shortest_path_tree.node);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), 8);
        for expected_shortest_path in expected_shortest_paths {
            let shortest_path =
                shortest_path_from_tree(&expected_shortest_path.to, &shortest_path_tree);
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }

    #[test]
    fn shortest_tree_sample2_test() {
        let (graph, expected_shortest_path_tree, expected_shortest_paths) = generate_test_sample2();
        assert_eq!(graph.len(), 5);

        let option_shortest_path_tree =
            super::shortest_path_tree(&graph, &expected_shortest_path_tree.node);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), 4);
        for expected_shortest_path in expected_shortest_paths {
            let shortest_path =
                shortest_path_from_tree(&expected_shortest_path.to, &shortest_path_tree);
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }

    #[test]
    fn shortest_tree_sample3_test() {
        let (graph, expected_shortest_path_tree, expected_shortest_paths) = generate_test_sample3();
        assert_eq!(graph.len(), 6);
        let option_shortest_path_tree =
            super::shortest_path_tree(&graph, &expected_shortest_path_tree.node);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), 5);
        for expected_shortest_path in expected_shortest_paths {
            let shortest_path =
                shortest_path_from_tree(&expected_shortest_path.to, &shortest_path_tree);
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }

    #[test]
    fn shortest_path_test() {
        let (graph, _, _) = generate_test_sample1();
        assert_eq!(graph.len(), 9);

        let from = "A";
        let mut to = "B";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 4,
                path: ["A", "B"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "C";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 12,
                path: ["A", "B", "C"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "D";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 19,
                path: ["A", "B", "C", "D"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "E";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 21,
                path: ["A", "H", "G", "F", "E"]
                    .iter()
                    .map(|n| n.to_string())
                    .collect()
            }
        );

        to = "F";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 11,
                path: ["A", "H", "G", "F"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "G";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 9,
                path: ["A", "H", "G"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "H";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 8,
                path: ["A", "H"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "I";
        let shortest_path = super::shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                from: from.to_string(),
                to: to.to_string(),
                distance: 14,
                path: ["A", "B", "C", "I"].iter().map(|n| n.to_string()).collect()
            }
        );
    }
}
