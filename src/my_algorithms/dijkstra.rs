// Implements Dijkstra’s Shortest Path Algorithm
// Useful resources:
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// https://www.youtube.com/watch?v=pVfj6mxhdMw
// https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub type GraphMap = HashMap<String, HashMap<String, u32>>;
pub type GraphVector = Vec<(String, Vec<(String, u32)>)>;
pub type GraphMatrix = Vec<Vec<u32>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathTreeNode {
    node: usize,
    distance: u32,
    previous: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathTreeNodeString {
    node: String,
    distance: u32,
    previous: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathTreeString {
    node: String,
    edges: Vec<ShortestPathTreeNodeString>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathString {
    from: String,
    to: String,
    distance: u32,
    path: Vec<String>,
}

pub fn add_edge(graph: &mut GraphMap, a: &str, b: &str, distance: u32) {
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

/// Converts map based graph into vector based graph
pub fn graph_map_to_vector(graph_map: &GraphMap) -> Option<GraphVector> {
    let node_count = graph_map.len();
    let mut vector_graph: GraphVector = GraphVector::with_capacity(node_count);

    for (node, map) in graph_map {
        let mut edges: Vec<(String, u32)> = Vec::with_capacity(map.len());  
        for (node, distance) in map {
            edges.push((node.to_string(), *distance));
        }
        vector_graph.push((node.to_string(), edges));
    }

    Some(vector_graph)
}

pub fn graph_map_to_matrix(graph_map: &GraphMap) -> (Vec<String>, GraphMatrix) {
    let node_count = graph_map.len();

    let mut names: Vec<String> = graph_map.keys().map(|x| x.clone()).collect();
    names.sort();

    let mut name_lookup: HashMap<String, usize> = HashMap::with_capacity(node_count);
    for name in names.iter() {
        name_lookup.insert(name.clone(), name_lookup.len());
    }

    let mut matrix = vec![vec![0; node_count]; node_count];

    for (node1, map) in graph_map {
        let index1 = name_lookup[node1];
        for (node2, distance) in map {
            let index2 = name_lookup[node2];
            matrix[index1][index2] = *distance;
            matrix[index2][index1] = *distance;
        }
    }

    (names, matrix)

}

pub fn graph_vector_to_matrix(graph_vector: &GraphVector) -> (Vec<String>, GraphMatrix) {
    let node_count = graph_vector.len();

    let mut names: Vec<String> = graph_vector.iter().map(|x| x.0.clone()).collect();
    names.sort();

    let mut name_lookup: HashMap<String, usize> = HashMap::with_capacity(node_count);
    for name in names.iter() {
        name_lookup.insert(name.clone(), name_lookup.len());
    }

    let mut matrix = vec![vec![0; node_count]; node_count];

    for (node1, edges) in graph_vector.iter() {
        let index1 = name_lookup[node1];
        for (node2, distance) in edges.iter() {
            let index2 = name_lookup[node2];
            matrix[index1][index2] = *distance;
            matrix[index2][index1] = *distance;
        } 
    }

    (names, matrix)

}

pub fn shortest_path_from_map_graph(graph_map: &GraphMap, from: &str, to: &str) -> Option<ShortestPathString> {
    if graph_map.get(from).is_none() || graph_map.get(to).is_none() {
        return None;
    }

    let shortest_path_tree = shortest_path_tree_from_map_graph(&graph_map, &from);
    if shortest_path_tree.is_none() {
        return None;
    }

    shortest_path_from_tree_string(&to, &shortest_path_tree.unwrap())
}

pub fn shortest_path_from_tree_string(
    to: &str,
    shortest_path_tree: &ShortestPathTreeString,
) -> Option<ShortestPathString> {
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

    Some(ShortestPathString {
        from: from.to_string(),
        to: to.to_string(),
        distance,
        path,
    })
}

pub fn shortest_path_tree_from_map_graph(graph_map: &GraphMap, source: &str) -> Option<ShortestPathTreeString> {
    struct PathRecord {
        name: String,
        distance: Option<u32>,
        prev_node: String,
        visited: bool,
    }

    // check if from and to nodes exists in graph
    if graph_map.get(source).is_none() {
        return None;
    }

    let mut path_table: Vec<PathRecord> = Vec::new();
    for node in graph_map.keys() {
        let path_record = PathRecord {
            name: node.to_string(),
            distance: None,
            prev_node: node.to_string(),
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

        let node = graph_map.get(&node_name);
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

    let mut edges: Vec<ShortestPathTreeNodeString> = Vec::new();
    for record in path_table {
        let edge = ShortestPathTreeNodeString {
            node: record.name.clone(),
            previous: record.prev_node.clone(),
            distance: record.distance.unwrap(),
        };
        edges.push(edge);
    }
    edges.sort_by(|a, b| a.node.cmp(&b.node));

    Some(ShortestPathTreeString {
        node: source.to_string(),
        edges,
    })
}

pub fn shortest_path_tree_from_vector_graph(graph_vector: &GraphVector, source: &str) -> Option<ShortestPathTreeString> {

    let node_count = graph_vector.len();
    let (node_names, matrix) = graph_vector_to_matrix(&graph_vector);

    let position = node_names.iter().position(|x| x == source);
    if position.is_none() {
        return None;
    }

    let source_index = position.unwrap();
    let shortest_path_tree_nodes = shortest_path_tree(&matrix, source_index);
    if shortest_path_tree_nodes.is_none() {
        return None;
    }

    let mut tree_nodes_string: Vec<ShortestPathTreeNodeString> = Vec::with_capacity(node_count);
    for tree_node in shortest_path_tree_nodes.unwrap() {
        tree_nodes_string.push(
            ShortestPathTreeNodeString {
                node: node_names[tree_node.node].clone(),
                distance: tree_node.distance,
                previous: node_names[tree_node.previous].clone()
            }
        )
    }

    Some(ShortestPathTreeString{node: source.to_string(), edges: tree_nodes_string})
}

pub fn shortest_path_tree(graph_matrix: &GraphMatrix, source: usize) -> Option<Vec<ShortestPathTreeNode>> {

    // check source node exists in the matrix
    let node_count = graph_matrix.len();
    if  source >= node_count {
        return None;
    }

    let mut visited = vec![false; node_count];
    let mut shortest_path_tree: Vec<ShortestPathTreeNode> = Vec::with_capacity(node_count);
    for node in 0..node_count {
        shortest_path_tree.push(
            ShortestPathTreeNode {
                node,
                distance: u32::MAX,
                previous: node,
            }
        );
    }

    // starting from the source node
    let mut node = source;
    shortest_path_tree[node].distance = 0;

    while node != usize::MAX {

        let node_distance = shortest_path_tree[node].distance;
        visited[node] = true;

        let column = &graph_matrix[node];

        // applying the core dijkstra algorithm
        // calculating new distance and setting a previous node to follow from
        for index in 0..node_count {
            if !visited[index] {
                let distance = column[index];
                if  distance > 0 {
                    let mut record = &mut shortest_path_tree[index];
                    let new_distance = node_distance + distance;
                    if record.distance > new_distance {
                            record.distance = new_distance;
                            record.previous = node;
                    }
                }
            }
        }

        // checking for not visited record with min distance
        node = usize::MAX;
        let mut min_distance = u32::MAX;
        for index in 0..node_count {
            let record = &shortest_path_tree[index];
            if !visited[index] && record.distance < min_distance {
                min_distance = record.distance;
                node = index;
            }    
        }
    }

    shortest_path_tree.sort_by(|a, b| a.node.cmp(&b.node));
    
    Some(shortest_path_tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    pub fn test_sample1() -> (GraphMap, ShortestPathTreeString, Vec<ShortestPathString>) {
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
    
        let mut graph = GraphMap::new();
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
            ("A", 0, "A"),
            ("B", 4, "A"),
            ("C", 12, "B"),
            ("D", 19, "C"),
            ("E", 21, "F"),
            ("F", 11, "G"),
            ("G", 9, "H"),
            ("H", 8, "A"),
            ("I", 14, "C"),
        ];
    
        let mut shortest_path_edges: Vec<ShortestPathTreeNodeString> =
            Vec::with_capacity(input_shortest_path_tree.len());
        input_shortest_path_tree.iter().for_each(|x| {
            shortest_path_edges.push(ShortestPathTreeNodeString {
                node: x.0.to_string(),
                distance: x.1,
                previous: x.2.to_string(),
            })
        });
    
        let expected_shortest_path_tree = ShortestPathTreeString {
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
    
        let mut expected_shortest_paths: Vec<ShortestPathString> =
            Vec::with_capacity(input_shortest_paths.len());
        input_shortest_paths.iter().for_each(|x| {
            expected_shortest_paths.push(ShortestPathString {
                from: x.0.to_string(),
                to: x.1.to_string(),
                distance: x.2,
                path: x.3.iter().map(|v| v.to_string()).collect(),
            })
        });
    
        (graph, expected_shortest_path_tree, expected_shortest_paths)
    }
    
    pub fn test_sample2() -> (GraphMap, ShortestPathTreeString, Vec<ShortestPathString>) {
        let input_graph_edges = [
            ("A", "B", 6),
            ("A", "D", 1),
            ("B", "D", 2),
            ("B", "E", 2),
            ("B", "C", 5),
            ("C", "E", 5),
            ("D", "E", 1),
        ];
    
        let mut graph = GraphMap::new();
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
                    {"node": "A", "distance": 0, "previous": "A" },
                    {"node": "B", "distance": 3, "previous": "D"},
                    {"node": "C", "distance": 7, "previous": "E"},
                    {"node": "D", "distance": 1, "previous": "A"},
                    {"node": "E", "distance": 2, "previous": "D"}
                ]
            }
        "#;
    
        let expected_shortest_path_tree: ShortestPathTreeString =
            serde_json::from_str(&input_shortest_path_tree).unwrap();
    
        let input_shortest_paths = r#"
            [
                {"from": "A", "to": "B", "distance": 3, "path": ["A", "D", "B"]},
                {"from": "A", "to": "C", "distance": 7, "path": ["A", "D", "E", "C"]},
                {"from": "A", "to": "D", "distance": 1, "path": ["A", "D"]},
                {"from": "A", "to": "E", "distance": 2, "path": ["A", "D", "E"]}
            ]
        "#;
    
        let expected_shortest_paths: Vec<ShortestPathString> =
            serde_json::from_str(&input_shortest_paths).unwrap();
    
        (graph, expected_shortest_path_tree, expected_shortest_paths)
    }
    
    pub fn test_sample3() -> (GraphMap, ShortestPathTreeString, Vec<ShortestPathString>) {
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
    
        let mut graph = GraphMap::new();
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
                {"node": "A", "distance": 0,  "previous": "A"},
                {"node": "B", "distance": 4,  "previous": "A"},
                {"node": "C", "distance": 5,  "previous": "A"},
                {"node": "D", "distance": 13, "previous": "B"},
                {"node": "E", "distance": 8,  "previous": "C"},
                {"node": "F", "distance": 14, "previous": "E"}
            ]
        });
    
        let expected_shortest_path_tree: ShortestPathTreeString =
            serde_json::from_value(input_shortest_path_tree).unwrap();
    
        let input_shortest_paths = json!([
            {"from": "A", "to": "B", "distance": 4,  "path": ["A", "B"]},
            {"from": "A", "to": "C", "distance": 5,  "path": ["A", "C"]},
            {"from": "A", "to": "D", "distance": 13, "path": ["A", "B", "D"]},
            {"from": "A", "to": "E", "distance": 8,  "path": ["A", "C", "E"]},
            {"from": "A", "to": "F", "distance": 14, "path": ["A", "C", "E", "F"]}
        ]);
    
        let expected_shortest_paths: Vec<ShortestPathString> =
            serde_json::from_value(input_shortest_paths).unwrap();
    
        (graph, expected_shortest_path_tree, expected_shortest_paths)
    }
    
    pub fn test_sample4() -> (GraphMap, ShortestPathTreeString, Vec<ShortestPathString>) {
        let input_graph_edges = [
            ("A", "B", 7),
            ("A", "C", 9),
            ("A", "F", 14),
            ("B", "C", 10),
            ("B", "D", 15),
            ("C", "D", 11),
            ("C", "F", 2),
            ("D", "E", 6),
            ("E", "F", 9),
        ];
    
        let mut graph = GraphMap::new();
        input_graph_edges
            .iter()
            .for_each(|x| add_edge(&mut graph, x.0, x.1, x.2));
    
        let input_shortest_path_tree = json!({
            "node": "A",
            "edges": [
                {"node": "A", "distance": 0,  "previous": "A"},
                {"node": "B", "distance": 7,  "previous": "A"},
                {"node": "C", "distance": 9,  "previous": "A"},
                {"node": "D", "distance": 20, "previous": "C"},
                {"node": "E", "distance": 20, "previous": "F"},
                {"node": "F", "distance": 11, "previous": "C"}
            ]
        });
    
        let expected_shortest_path_tree: ShortestPathTreeString =
            serde_json::from_value(input_shortest_path_tree).unwrap();
    
        let input_shortest_paths = json!([
            {"from": "A", "to": "B", "distance": 7,  "path": ["A", "B"]},
            {"from": "A", "to": "C", "distance": 9,  "path": ["A", "C"]},
            {"from": "A", "to": "D", "distance": 20, "path": ["A", "C", "D"]},
            {"from": "A", "to": "E", "distance": 20, "path": ["A", "C", "F", "E"]},
            {"from": "A", "to": "F", "distance": 11, "path": ["A", "C", "F"]}
        ]);
    
        let expected_shortest_paths: Vec<ShortestPathString> =
            serde_json::from_value(input_shortest_paths).unwrap();
    
        (graph, expected_shortest_path_tree, expected_shortest_paths)
    }
    
    #[test]
    fn graph_matrix_sample1_test() {
        let (graph_map, expected_shortest_path_tree, expected_shortest_paths) = test_sample1();
        assert!(!graph_map.is_empty());
        assert!(!expected_shortest_path_tree.edges.is_empty());
        assert!(!expected_shortest_paths.is_empty());

        let (names, graph_matrix) = graph_map_to_matrix(&graph_map);
        assert!(!names.is_empty());
        assert_eq!(names.len(), graph_map.len());

        assert!(!graph_matrix.is_empty());
        assert_eq!(graph_matrix.len(), names.len());
    }
 
    #[test]
    fn graph_vector_sample1_test() {
        graph_vector_test(test_sample1); 
    }

    #[test]
    fn graph_vector_sample2_test() {
        graph_vector_test(test_sample2); 
    }

    #[test]
    fn graph_vector_sample3_test() {
        graph_vector_test(test_sample3); 
    }

    #[test]
    fn graph_vector_sample4_test() {
        graph_vector_test(test_sample4); 
    }

    fn graph_vector_test(fn_test_input: fn() -> (GraphMap, ShortestPathTreeString, Vec<ShortestPathString>)) {
        let (graph_map, expected_shortest_path_tree, expected_shortest_paths) = fn_test_input();
        assert!(!graph_map.is_empty());
        assert!(!expected_shortest_path_tree.edges.is_empty());
        assert!(!expected_shortest_paths.is_empty());

        let graph_vector = super::graph_map_to_vector(&graph_map).unwrap();
        assert!(!graph_vector.is_empty());

        let option_shortest_path_tree =
            super::shortest_path_tree_from_vector_graph(&graph_vector, &expected_shortest_path_tree.node);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), graph_map.len()-1);
        for expected_shortest_path in expected_shortest_paths {
            let shortest_path =
                shortest_path_from_tree_string(&expected_shortest_path.to, &shortest_path_tree);
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }

    #[test]
    fn shortest_tree_sample1_test() {
        shortest_tree_test(test_sample1)
    }
    #[test]
    fn shortest_tree_sample2_test() {
        shortest_tree_test(test_sample2)
    }
    #[test]
    fn shortest_tree_sample3_test() {
        shortest_tree_test(test_sample3)
    }

    #[test]
    fn shortest_tree_sample4_test() {
        shortest_tree_test(test_sample4)
    }

    fn shortest_tree_test(fn_test_input: fn() -> (GraphMap, ShortestPathTreeString, Vec<ShortestPathString>)) {
        let (graph_map, expected_shortest_path_tree, expected_shortest_paths) = fn_test_input();
        assert!(!graph_map.is_empty());
        assert!(!expected_shortest_path_tree.edges.is_empty());
        assert!(!expected_shortest_paths.is_empty());

        let option_shortest_path_tree =
            super::shortest_path_tree_from_map_graph(&graph_map, &expected_shortest_path_tree.node);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), graph_map.len()-1);
        for expected_shortest_path in expected_shortest_paths {
            let shortest_path =
                shortest_path_from_tree_string(&expected_shortest_path.to, &shortest_path_tree);
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }

    #[test]
    fn shortest_path_test() {
        let (graph, _, _) = test_sample1();
        assert!(!graph.is_empty());

        let from = "A";
        let mut to = "B";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 4,
                path: ["A", "B"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "C";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 12,
                path: ["A", "B", "C"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "D";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 19,
                path: ["A", "B", "C", "D"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "E";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
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
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 11,
                path: ["A", "H", "G", "F"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "G";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 9,
                path: ["A", "H", "G"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "H";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 8,
                path: ["A", "H"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "I";
        let shortest_path = super::shortest_path_from_map_graph(&graph, &from, &to);
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPathString {
                from: from.to_string(),
                to: to.to_string(),
                distance: 14,
                path: ["A", "B", "C", "I"].iter().map(|n| n.to_string()).collect()
            }
        );
    }
}
