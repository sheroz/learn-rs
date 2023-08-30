// https://www.programiz.com/dsa/binary-search-tree

/*
A binary search tree (BST) is a node-based binary tree data structure that has the following properties.

1. The left subtree of a node contains only nodes with keys less than the node’s key.
2. The right subtree of a node contains only nodes with keys greater than the node’s key.
3. Both the left and right subtrees must also be binary search trees.
4. Each node (item in the tree) has a distinct key.

https://www.geeksforgeeks.org/a-program-to-check-if-a-binary-tree-is-bst-or-not/
*/

use crate::tree::binary_tree::*;
use std::collections::{HashSet, VecDeque};

pub struct BinarySearchTree {
    pub tree: BinaryTree,
}

impl BinarySearchTree {
    pub fn with_root(root: BinaryTreeNodeRef) -> Self {
        BinarySearchTree {
            tree: BinaryTree::with_root(root),
        }
    }

    pub fn insert_recursion(
        node: Option<BinaryTreeNodeRef>,
        new_node_ref: BinaryTreeNodeRef,
    ) -> Option<BinaryTreeNodeRef> {
        match node {
            None => return Some(new_node_ref),
            Some(node_ref) => {
                if node_ref.borrow().data > new_node_ref.borrow().data {
                    let left = node_ref.clone().borrow().left.clone(); 
                    node_ref.borrow_mut().left = Self::insert_recursion(left, new_node_ref);
                } else {
                    let right = node_ref.clone().borrow().right.clone(); 
                    node_ref.borrow_mut().right = Self::insert_recursion(right, new_node_ref);
                }
                return Some(node_ref);
            }
        }
    }

    pub fn insert_iterative(root: BinaryTreeNodeRef, new_node: BinaryTreeNodeRef) {
        /*
        def insert(root, key):
            # Create a new Node containing
            # the new element
            newnode = newNode(key)

            # Pointer to start traversing from root
            # and traverses downward path to search
            # where the new node to be inserted
            x = root

            # Pointer y maintains the trailing
            # pointer of x
            y = None

            while (x != None):
                y = x
                if (key < x.key):
                    x = x.left
                else:
                    x = x.right

            # If the root is None i.e the tree is
            # empty. The new node is the root node
            if (y == None):
                y = newnode

            # If the new key is less than the leaf node key
            # Assign the new node to be its left child
            elif (key < y.key):
                y.left = newnode

            # else assign the new node its
            # right child
            else:
                y.right = newnode

            # Returns the pointer where the
            # new node is inserted
            return y
         */
    }

    pub fn delete() {}

    pub fn is_binary_search_tree(node: &BinaryTreeNodeRef) -> bool {
        BinarySearchTree::is_binary_search_tree_v3(node)
    }

    pub fn search(&self, data: u32) -> Option<BinaryTreeNodeRef> {
        if self.tree.root.is_none() {
            return None;
        }

        let node = self.tree.root.as_ref().unwrap();

        let mut queue = VecDeque::new();
        queue.push_back(node.clone());

        while let Some(node) = queue.pop_front() {
            let n = node.borrow();

            let node_data = n.data;
            if data == node_data {
                return Some(node.clone());
            }

            if data < node_data {
                if let Some(left) = n.left.as_ref() {
                    queue.push_back(left.clone());
                }
            } else {
                if let Some(right) = n.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }
        }
        None
    }

    pub fn is_binary_search_tree_v3(node: &BinaryTreeNodeRef) -> bool {
        let mut queue = VecDeque::new();

        let mut prev = 0;
        let mut prev_valid = false;

        if let Some(leftmost) = BinaryTree::leftmost(node.clone()) {
            queue.push_back(leftmost);
        } else {
            queue.push_back(node.clone());
        }

        while let Some(node_ref) = queue.pop_front() {
            let node = node_ref.borrow();

            let mut cur = node.data;
            if prev_valid && cur <= prev {
                return false;
            }
            prev = cur;
            prev_valid = true;

            if let Some(parent) = node.parent.upgrade() {
                cur = parent.borrow().data;
                if cur <= prev {
                    return false;
                }
                prev = cur;
            }

            if let Some(right) = node.right.as_ref() {
                if let Some(leftmost) = BinaryTree::leftmost(right.clone()) {
                    queue.push_back(leftmost);
                } else {
                    queue.push_back(right.clone());
                }
            }
        }
        true
    }

    pub fn is_binary_search_tree_v2(node: &BinaryTreeNodeRef) -> bool {
        let flatten = BinaryTree::flatten_left_to_right(node.clone());
        let values: Vec<_> = flatten.iter().map(|n| n.borrow().data).collect();
        values.windows(2).all(|v| v[0] < v[1])
    }

    pub fn is_binary_search_tree_v1(node: &BinaryTreeNodeRef) -> bool {
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
    use std::rc::Weak;

    use super::*;
    use crate::tree::binary_tree::test_utils::*;
    
    #[test]
    fn insert() {
        let list= populate_node_ref_list();
        
        let mut root = None;
        for node_ref in &list {
            root = BinarySearchTree::insert_recursion(root, node_ref.clone());
        }
        assert!(root.is_some());
        let root_node = root.unwrap();
        assert!(BinarySearchTree::is_binary_search_tree(&root_node));

        let flatten = BinaryTree::flatten_top_down(root_node.clone());
        assert_eq!(flatten.len(), list.len());

        let flatten = BinaryTree::flatten_left_to_right(root_node);
        assert_eq!(flatten.len(), list.len());
    }

    #[test]
    fn is_binary_search_tree_v1() {
        let root = populate_balanced_binary_search_tree();
        assert!(BinarySearchTree::is_binary_search_tree_v1(&root));
    }

    #[test]
    fn non_binary_search_tree_v1() {
        let root = populate_balanced_binary_tree();
        assert!(!BinarySearchTree::is_binary_search_tree_v1(&root));
    }

    #[test]
    fn is_binary_search_tree_v2() {
        let root = populate_balanced_binary_search_tree();
        assert!(BinarySearchTree::is_binary_search_tree_v2(&root));
    }

    #[test]
    fn non_binary_search_tree_v2() {
        let root = populate_balanced_binary_tree();
        assert!(!BinarySearchTree::is_binary_search_tree_v2(&root));
    }

    #[test]
    fn is_binary_search_tree_v3() {
        let root = populate_balanced_binary_search_tree();
        assert!(BinarySearchTree::is_binary_search_tree_v3(&root));
    }

    #[test]
    fn non_binary_search_tree_v3() {
        let root = populate_balanced_binary_tree();
        assert!(!BinarySearchTree::is_binary_search_tree_v3(&root));
    }

    #[test]
    fn is_binary_search_tree() {
        let root = populate_balanced_binary_search_tree();
        assert!(BinarySearchTree::is_binary_search_tree(&root));
    }

    #[test]
    fn non_binary_search_tree() {
        let root = populate_balanced_binary_tree();
        assert!(!BinarySearchTree::is_binary_search_tree(&root));
    }

    #[test]
    fn search() {
        let root = populate_balanced_binary_search_tree();
        let bst = BinarySearchTree::with_root(root.clone());

        let flatten = BinaryTree::flatten_left_to_right(root);
        let values: Vec<_> = flatten.iter().map(|n| n.borrow().data).collect();
        let not_exist = 1 + *values.last().unwrap();
        for v in values {
            let node = bst.search(v);
            assert!(node.is_some());
            assert_eq!(node.unwrap().borrow().data, v);
        }

        let node = bst.search(not_exist);
        assert!(node.is_none());
    }
}
