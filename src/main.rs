use learn_rs::*;

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
}