// https://www.programiz.com/dsa/binary-search-tree

use std::collections::{HashSet, VecDeque};

use crate::tree::binary_tree::*;

pub struct BinarySearchTree {
    pub tree: BinaryTree,
}

impl BinarySearchTree {
    pub fn new(tree: BinaryTree) -> Self {
        BinarySearchTree { tree }
    }

    pub fn is_binary_search_tree2(node: BinaryTreeNodeRef) -> bool {
        let flatten = BinaryTree::flatten_left_to_right(node);
        let values: Vec<_> = flatten.iter().map(|n| n.borrow().data).collect();
        values.windows(2).all(|v| v[0] < v[1])
    }

    pub fn is_binary_search_tree1(node: BinaryTreeNodeRef) -> bool {
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
        queue.push_back(node.clone());
        while let Some(node) = queue.pop_front() {
            let n = node.borrow();

            let v = n.data;
            if keys.contains(&v) {
                return false; // (4)
            }
            keys.insert(v);

            if let Some(left) = n.left.as_ref() {
                let subtree = BinaryTree::flatten_top_down(left.clone());
                let values = subtree.iter().map(|r| r.borrow().data).collect::<Vec<_>>();
                for data in values {
                    if data >= v {
                        return false; // (1)
                    }
                }
                queue.push_back(left.clone()); // (3)
            }

            if let Some(right) = n.right.as_ref() {
                let subtree = BinaryTree::flatten_top_down(right.clone());
                let values = subtree.iter().map(|r| r.borrow().data).collect::<Vec<_>>();
                for data in values {
                    if data <= v {
                        return false; // (2)
                    }
                }
                queue.push_back(right.clone()); // (3)
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
    fn is_binary_search_tree1() {
        let root = populate_balanced_binary_search_tree();
        assert!(BinarySearchTree::is_binary_search_tree1(root));
    }

    #[test]
    fn non_binary_search_tree1() {
        let root = populate_balanced_binary_tree();
        assert!(!BinarySearchTree::is_binary_search_tree1(root));
    }

    #[test]
    fn is_binary_search_tree2() {
        let root = populate_balanced_binary_search_tree();
        assert!(BinarySearchTree::is_binary_search_tree2(root));
    }

    #[test]
    fn non_binary_search_tree2() {
        let root = populate_balanced_binary_tree();
        assert!(!BinarySearchTree::is_binary_search_tree2(root));
    }
}
