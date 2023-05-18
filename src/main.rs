use my_rust::*;

fn main() {
    my_calc::calc_sample();

    my_collections::sample_array::array_sample();
    my_collections::sample_array::array_slice_sample();

    my_collections::sample_vectors::vec_sample();
    my_collections::sample_vectors::vec_deque_sample();

    my_collections::sample_linked_list::linked_list_sample();

    my_collections::sample_maps::hashmap_sample();
    my_collections::sample_maps::btreemap_sample();

    my_collections::sample_sets::hashset_sample();
    my_collections::sample_sets::btreeset_sample();

    println!("fibonacci(7) = {}", my_algorithms::fibonacci::fibonacci(7));

    let sample_text = "A man, a plan, a canal: Panama!";
    println!("Check for palindrome: {}\n{}\n", sample_text, my_algorithms::palindrome::is_palindrome(sample_text));

    // Dijkstra’s Shortest Path Algorithm
    let (graph,_, _) = my_algorithms::dijkstra::generate_test_sample1();
    println!("Graph {:?}", &graph);
    let mut keys: Vec<_> = graph.keys().collect();
    keys.sort();
    println!("\nGraph view by sorted keys:");
    for node in keys {
        println!("{:?}", graph.get(node).unwrap());
    }

    let shortest_path = my_algorithms::dijkstra::shortest_path_map(&graph, "0", "8");
    println!("Shortest path from 0 to 8 is :\n {:?}\n", shortest_path);
}