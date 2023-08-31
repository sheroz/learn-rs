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

    pub fn insert_recursion(node: Option<BinaryTreeNodeRef>,
        new_node_ref: &BinaryTreeNodeRef,
    ) -> Option<BinaryTreeNodeRef> {
        match node {
            None => return Some(new_node_ref.clone()),
            Some(node_ref) => {
                let mut parent = None;
                if node_ref.borrow().data > new_node_ref.borrow().data {
                    let left = node_ref.borrow().left.clone();
                    if left.is_none() {
                        parent = Some(Rc::downgrade(&node_ref));
                    }
                    node_ref.borrow_mut().left = Self::insert_recursion(left, new_node_ref);
                } else {
                    let right = node_ref.borrow().right.clone(); 
                    if right.is_none() {
                        parent = Some(Rc::downgrade(&node_ref));
                    }
                    node_ref.borrow_mut().right = Self::insert_recursion(right, new_node_ref);
                }
                if parent.is_some() {
                    new_node_ref.borrow_mut().parent = parent.unwrap();
                }
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
            new_node.borrow_mut().parent = Rc::downgrade(&node_ref);
            let mut node = node_ref.borrow_mut();
            if new_node.borrow().data < node.data {
                node.left = Some(new_node.clone());
            }
            else {
                node.right = Some(new_node.clone());
            }
        }
        return insert_node;
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
        // this is an optimized version of v2
        let mut visited = None;

        let mut root = Some(node.clone());
        let mut leftdone = false;
        while let Some(root_ref) = root.as_ref() {
            let mut current_ref = root_ref.clone();
            if !leftdone {
                if let Some(leftmost) = BinaryTree::leftmost(&current_ref) {
                    current_ref = leftmost.clone();
                }
            }

            if let Some(node) = visited {
                if current_ref <= node {
                    return false;
                }
            }
            visited = Some(current_ref.clone());

            root = Some(current_ref.clone());
            leftdone = true;

            let root_node = current_ref.borrow();

            if let Some(right) = root_node.right.as_ref() {
                leftdone = false;
                root = Some(right.clone());
            } else if let Some(parent) = root_node.parent.upgrade() {
                let mut root_parent = Some(parent.clone());
                let mut parent_right = parent.clone().borrow().right.clone();
                while root_parent.is_some() {
                    if !BinaryTree::is_same(&root, &parent_right) {
                        break;
                    }

                    root = root_parent;
                    root_parent = if root.is_some() {
                        root.clone().unwrap().borrow().parent.upgrade()
                    } else {
                        None
                    };

                    parent_right = if root_parent.is_some() {
                        root_parent.clone().unwrap().borrow().right.clone()
                    } else {
                        None
                    };
                }
                root = root_parent;
            } else {
                break;
            }
        }
        true
    }

    pub fn is_binary_search_tree_v2(node: &BinaryTreeNodeRef) -> bool {
        let flatten = BinaryTree::flatten_inorder(node.clone());
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
            root = BinarySearchTree::insert_recursion(root, node_ref);
        }
        assert!(root.is_some());
        let root_node = root.unwrap();
        assert_eq!(BinaryTree::count(&root_node), list.len());
        assert!(BinarySearchTree::is_binary_search_tree_v1(&root_node));
        assert!(BinarySearchTree::is_binary_search_tree_v2(&root_node));
        assert!(BinarySearchTree::is_binary_search_tree_v3(&root_node));
        assert!(BinarySearchTree::is_binary_search_tree(&root_node));
    }

    #[test]
    fn insert_iterative() {
        let list= populate_node_list();
        
        let mut node = None;
        for node_ref in &list {
            node = BinarySearchTree::insert_iterative(node, node_ref.clone());
        }
        let root = BinaryTree::get_root(&node.unwrap());
        assert_eq!(BinaryTree::count(&root), list.len());
        assert!(BinarySearchTree::is_binary_search_tree_v1(&root));
        assert!(BinarySearchTree::is_binary_search_tree_v2(&root));
        assert!(BinarySearchTree::is_binary_search_tree_v3(&root));
        assert!(BinarySearchTree::is_binary_search_tree(&root));
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

        let flatten = BinaryTree::flatten_inorder(root);
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
