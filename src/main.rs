use my_rust::*;

fn main() {

    my_calc::calc_sample();
    let txt = "Hello world!";
    let hex_encoded = crypto::openssl_sample::sha256_digest(txt);    
    println!("\nOpenSSL: SHA256 hash for {}\n{}\n", txt, hex_encoded);

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
 
}