use std::collections::{VecDeque, HashSet};

use crate::tree::binary_tree::*;

pub struct BinarySearchTree {
    pub tree: BinaryTree,
}

impl BinarySearchTree {
    pub fn new(tree: BinaryTree) -> Self {
        BinarySearchTree { tree }
    }

    pub fn is_binary_search_tree(tree: &BinaryTree) -> bool {
        /* 
        A binary search tree (BST) is a node-based binary tree data structure that has the following properties. 

        1. The left subtree of a node contains only nodes with keys less than the node’s key.
        2. The right subtree of a node contains only nodes with keys greater than the node’s key.
        3. Both the left and right subtrees must also be binary search trees.
        4. Each node (item in the tree) has a distinct key.

        https://www.geeksforgeeks.org/a-program-to-check-if-a-binary-tree-is-bst-or-not/
        */

        let mut keys = HashSet::<u32>::new();

        let mut queue = VecDeque::new();
        let node = tree.root.as_ref().unwrap();
        queue.push_back(node.clone());
        while let Some(node) = queue.pop_front() {

            let n = node.borrow();

            let v = n.value;
            if keys.contains(&v) {
                return false; // (4)
            }
            keys.insert(v);

            if let Some(left) = n.left.as_ref() {
                if left.borrow().value >= v {
                    return false; // (1)
                }
                queue.push_back(left.clone());
            }

            if let Some(right) = n.right.as_ref() {
                if right.borrow().value <= v {
                    return false; // (2)
                }

                queue.push_back(right.clone());
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::binary_tree::test_utils::*;

    #[test]
    fn populate_tree() {
        let tree = populate_balanced_binary_tree();
        assert_eq!(tree.count(), NODES_COUNT);
        assert!(!BinarySearchTree::is_binary_search_tree(&tree));
    }
}
