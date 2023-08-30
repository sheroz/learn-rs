// https://www.programiz.com/dsa/binary-search-tree

use crate::tree::binary_tree::*;
use std::{collections::{HashSet, VecDeque}, rc::Rc};

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
                    node_ref.borrow_mut().left = Self::insert_recursion(left, new_node_ref.clone());
                } else {
                    let right = node_ref.clone().borrow().right.clone(); 
                    node_ref.borrow_mut().right = Self::insert_recursion(right, new_node_ref.clone());
                }
                // new_node_ref.borrow_mut().parent = Rc::downgrade(&node_ref);
                return Some(node_ref);
            }
        }
    }

    pub fn insert_iterative(root: Option<BinaryTreeNodeRef>, new_node: BinaryTreeNodeRef) -> Option<BinaryTreeNodeRef> {

        let mut start = root.clone();
        let mut insert_node = None;

        while start.is_some() {
            insert_node = start.clone();
            let node = insert_node.as_ref().unwrap().borrow();
            if new_node.borrow().data < node.data {
                start = node.left.clone();
            }
            else {
                start = node.right.clone();
            }
        }

        if insert_node.is_none() {
            insert_node = Some(new_node.clone());
        }
        else {
            let node_ref = insert_node.as_ref().unwrap();
            let mut node = node_ref.borrow_mut();
            if new_node.borrow().data < node.data {
                node.left = Some(new_node.clone());
            }
            else {
                node.right = Some(new_node.clone());
            }
            new_node.borrow_mut().parent = Rc::downgrade(&node_ref.clone());
        }
        insert_node
    }

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
    use super::*;
    use crate::tree::binary_tree::test_utils::*;
    
    #[test]
    fn insert_recursion() {
        let list= populate_node_list();
        
        let mut root = None;
        for node_ref in &list {
            root = BinarySearchTree::insert_recursion(root, node_ref.clone());
        }
        assert!(root.is_some());
        let root_node = root.unwrap();
        println!("{:#?}", root_node);

        assert_eq!(BinaryTree::count(&root_node), list.len());

        assert!(BinarySearchTree::is_binary_search_tree(&root_node));

        assert!(BinarySearchTree::is_binary_search_tree_v2(&root_node));
    }

    #[test]
    fn insert_iterative() {
        let list= populate_node_list();
        
        let mut node = None;
        for node_ref in &list {
            node = BinarySearchTree::insert_iterative(node, node_ref.clone());
        }
        let root = BinaryTree::get_root(&node.unwrap());
        println!("{:#?}",root);
        assert_eq!(BinaryTree::count(&root), list.len());

        assert!(BinarySearchTree::is_binary_search_tree(&root));

        assert!(BinarySearchTree::is_binary_search_tree_v2(&root));
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
