// Implements Dijkstra’s Shortest Path Algorithm
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// https://www.youtube.com/watch?v=pVfj6mxhdMw

use std::collections::HashMap;

pub type Graph = HashMap<String, Node>;

pub struct Node {
    name: String,
    edges: HashMap<String, u32>,
}

pub struct ShortestPath {
    path: Vec<String>,
    distance: u32,
}

pub fn shortest_path(graph: Graph, a: String, b: String) -> Option<ShortestPath> {
    None
}

pub fn generate_graph() -> Graph {
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
        a_node.unwrap().edges.insert(b.to_string(), distance);
    } 
    else {
        let mut edges = HashMap::new();
        edges.insert(a.to_string(), distance);
        let node = Node {
            name: a.to_string(),
            edges: edges
        };
        graph.insert(a.to_string(), node);
    }

    let b_node = graph.get_mut(b);
    if b_node.is_some() {
        b_node.unwrap().edges.insert(b.to_string(), distance);
    } 
    else {
        let mut edges = HashMap::new();
        edges.insert(b.to_string(), distance);
        let node = Node {
            name: b.to_string(),
            edges: edges
        };
        graph.insert(b.to_string(), node);
    }
}

#[cfg(test)]
mod tests {
    use super::generate_graph;

    #[test]
    fn shortest_path_test() {

        /*

        let graph =
            [ [ 0, 4, 0, 0, 0, 0, 0, 8, 0 ],
              [ 4, 0, 8, 0, 0, 0, 0, 11, 0 ],
              [ 0, 8, 0, 7, 0, 4, 0, 0, 2 ],
              [ 0, 0, 7, 0, 9, 14, 0, 0, 0],
              [ 0, 0, 0, 9, 0, 10, 0, 0, 0 ],
              [ 0, 0, 4, 14, 10, 0, 2, 0, 0],
              [ 0, 0, 0, 0, 0, 2, 0, 1, 6 ],
              [ 8, 11, 0, 0, 0, 0, 1, 0, 7 ],
              [ 0, 0, 2, 0, 0, 0, 6, 7, 0 ] ]
              
        Output: 0 4 12 19 21 11 9 8 14
        Explanation: The distance from 0 to 1 = 4.
        The minimum distance from 0 to 2 = 12. 0->1->2
        The minimum distance from 0 to 3 = 19. 0->1->2->3
        The minimum distance from 0 to 4 = 21. 0->7->6->5->4
        The minimum distance from 0 to 5 = 11. 0->7->6->5
        The minimum distance from 0 to 6 = 9. 0->7->6
        The minimum distance from 0 to 7 = 8. 0->7
        The minimum distance from 0 to 8 = 14. 0->1->2->8
        
         */

        let mut graph = generate_graph();
        let result = super::shortest_path(graph, "0".to_string(), "8".to_string());

    }
}
