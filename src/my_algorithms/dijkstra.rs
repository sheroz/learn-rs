// Implements Dijkstra’s Shortest Path Algorithm
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// https://www.youtube.com/watch?v=pVfj6mxhdMw
// sample: https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/

use std::collections::HashMap;

pub type Graph = HashMap<String, HashMap<String, u32>>;

#[derive(Debug, PartialEq)]
pub struct ShortestPath {
    path: Vec<String>,
    distance: u32,
}

pub fn shortest_path(graph: &Graph, from: &str, to: &str) -> Option<ShortestPath> {
    struct PathRecord {
        name: String,
        distance: Option<u32>,
        prev_node: Option<String>,
        visited: bool,
    }

    // check if from and to nodes exists in graph
    if graph.get(from).is_none() || graph.get(to).is_none() {
        return None;
    }

    let mut path_table: Vec<PathRecord> = Vec::new();
    for node in graph.keys() {
        let path_record = PathRecord {
            name: node.to_string(),
            distance: None,
            prev_node: None,
            visited: false,
        };
        path_table.push(path_record);
    }

    // make starting node distance as zero
    path_table
        .iter_mut()
        .find(|r| r.name == from)
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
                    record.prev_node = Some(node_name.clone());
                }
            }
        }
    }

    let mut path: Vec<String> = Vec::new();
    let path_record = path_table.iter().find(|r| r.name == to).unwrap();
    let distance = path_record.distance.unwrap();
    let mut prev_name = path_record.prev_node.as_ref().unwrap().clone();

    path.push(to.to_string());
    while prev_name != from {
        path.push(prev_name.clone());

        let path_record_option = path_table.iter().find(|r| r.name == prev_name);
        if path_record_option.is_none() {
            break;
        }

        prev_name = path_record_option
            .unwrap()
            .prev_node
            .as_ref()
            .unwrap()
            .clone();
    }

    path.push(from.to_string());
    path.reverse();

    Some(ShortestPath {
        path: path,
        distance: distance,
    })
}

pub fn generate_graph_sample() -> Graph {
    let mut graph = Graph::new();
    add_edge(&mut graph, &"0", &"1", 4);
    add_edge(&mut graph, &"0", &"7", 8);
    add_edge(&mut graph, &"1", &"2", 8);
    add_edge(&mut graph, &"1", &"7", 11);
    add_edge(&mut graph, &"2", &"3", 7);
    add_edge(&mut graph, &"2", &"8", 2);
    add_edge(&mut graph, &"2", &"5", 4);
    add_edge(&mut graph, &"3", &"4", 9);
    add_edge(&mut graph, &"3", &"5", 14);
    add_edge(&mut graph, &"4", &"5", 10);
    add_edge(&mut graph, &"5", &"6", 2);
    add_edge(&mut graph, &"6", &"7", 1);
    add_edge(&mut graph, &"6", &"8", 6);
    add_edge(&mut graph, &"7", &"8", 7);
    graph
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
    fn shortest_path_test() {
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

        Output:
        The minimum distance from 0 to 1 = 4.  0->1
        The minimum distance from 0 to 2 = 12. 0->1->2
        The minimum distance from 0 to 3 = 19. 0->1->2->3
        The minimum distance from 0 to 4 = 21. 0->7->6->5->4
        The minimum distance from 0 to 5 = 11. 0->7->6->5
        The minimum distance from 0 to 6 = 9.  0->7->6
        The minimum distance from 0 to 7 = 8.  0->7
        The minimum distance from 0 to 8 = 14. 0->1->2->8
        */

        let graph = generate_graph_sample();
        assert_eq!(graph.len(), 9);

        let shortest_path = super::shortest_path(&graph, "0", "1");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "1"].iter().map(|n| n.to_string()).collect(),
                distance: 4
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "2");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "1", "2"].iter().map(|n| n.to_string()).collect(),
                distance: 12
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "3");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "1", "2", "3"].iter().map(|n| n.to_string()).collect(),
                distance: 19
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "4");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "7", "6", "5", "4"]
                    .iter()
                    .map(|n| n.to_string())
                    .collect(),
                distance: 21
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "5");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "7", "6", "5"].iter().map(|n| n.to_string()).collect(),
                distance: 11
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "6");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "7", "6"].iter().map(|n| n.to_string()).collect(),
                distance: 9
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "7");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "7"].iter().map(|n| n.to_string()).collect(),
                distance: 8
            }
        );

        let shortest_path = super::shortest_path(&graph, "0", "8");
        assert_eq!(
            shortest_path.unwrap(),
            ShortestPath {
                path: ["0", "1", "2", "8"].iter().map(|n| n.to_string()).collect(),
                distance: 14
            }
        );
    }
}
