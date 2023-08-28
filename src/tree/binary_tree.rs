// https://www.programiz.com/dsa/trees

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct BinaryTreeNode {
    pub id: Uuid,
    pub name: String,
    pub data: u32,
    pub parent: Option<BinaryTreeNodeRef>,
    pub left: Option<BinaryTreeNodeRef>,
    pub right: Option<BinaryTreeNodeRef>,
}

pub type BinaryTreeNodeRef = Rc<RefCell<BinaryTreeNode>>;

pub struct BinaryTree {
    pub root: Option<BinaryTreeNodeRef>,
}

impl BinaryTree {
    pub fn with_root(root: BinaryTreeNodeRef) -> Self {
        BinaryTree { root: Some(root) }
    }

    pub fn new_node() -> BinaryTreeNodeRef {
        Rc::new(RefCell::new(BinaryTreeNode {
            id: Uuid::new_v4(),
            name: "".to_string(),
            data: 0,
            parent: None,
            left: None,
            right: None,
        }))
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        let mut queue = VecDeque::new();
        let node = self.root.as_ref().unwrap();
        queue.push_back(node.clone());
        while let Some(node) = queue.pop_front() {
            count += 1;

            let n = node.borrow();
            if let Some(left) = n.left.as_ref() {
                queue.push_back(left.clone());
            }
            if let Some(right) = n.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
        count
    }

    pub fn flatten_top_down(node: BinaryTreeNodeRef) -> Vec<BinaryTreeNodeRef> {
        let mut nodes = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(node.clone());
        while let Some(node) = queue.pop_front() {
            nodes.push(node.clone());
            let n = node.borrow();
            if let Some(left) = n.left.as_ref() {
                queue.push_back(left.clone());
            }
            if let Some(right) = n.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
        nodes
    }

    pub fn leftmost(node_ref: BinaryTreeNodeRef) -> Option<BinaryTreeNodeRef> {
        let mut leftmost = None;
        let mut current = node_ref;
        loop {
            let node_ref = current;
            let node = node_ref.borrow();
            if let Some(left) = node.left.as_ref() {
                current = left.clone();
                leftmost = Some(left.clone());
            } else {
                return leftmost;
            }
        }
    }

    pub fn flatten_left_to_right(node_ref: BinaryTreeNodeRef) -> Vec<BinaryTreeNodeRef> {
        // inorder traversal:
        // https://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/
        // https://www.javatpoint.com/inorder-traversal
        // https://www.tutorialspoint.com/data_structures_algorithms/tree_traversal.htm
        // https://www.geeksforgeeks.org/inorder-tree-traversal-without-recursion-and-without-stack/
        //
        // best suited one:
        // https://www.geeksforgeeks.org/inorder-non-threaded-binary-tree-traversal-without-recursion-or-stack/

        let mut root = Some(node_ref.clone());

        let mut nodes = VecDeque::new();

        let mut leftdone = false;
        while let Some(root_ref) = root.clone().as_ref() {

            let mut current_ref = root_ref.clone();
            if !leftdone {
                if let Some(leftmost) = BinaryTree::leftmost(current_ref.clone()) {
                    current_ref = leftmost.clone();
                }
            }

            leftdone = true;
            nodes.push_back(current_ref.clone());

            if let Some(right) = root_ref.borrow().right.clone() {
                leftdone = false;
                root = Some(right.clone());
            } else if let Some(parent) = root_ref.borrow().parent.clone() {
                let mut root_parent = Some(parent.clone());
                let mut parent_right = parent.clone().borrow().right.clone();
                while root_parent.is_some() {

                    if BinaryTree::get_id(root) != BinaryTree::get_id(parent_right) {
                        break;
                    }

                    root = root_parent.clone();
                    root_parent = if root.is_some() {
                        root.clone().unwrap().borrow().parent.clone()
                    } else {
                        None
                    };
                    parent_right = if root_parent.is_some() {
                        root_parent.clone().unwrap().borrow().right.clone()
                    } else {
                        None
                    };
                }
                if root_parent.is_none() {
                    break;
                }
                root = root_parent.clone();
            } else {
                break;
            }
        }

        nodes.into()
    }

    fn get_id(v: Option<BinaryTreeNodeRef>) -> Option<Uuid> {
        match v {
            Some(node) => Some(node.borrow().id),
            None => None
        } 
    }

}

#[cfg(test)]
pub mod test_utils {

    use super::*;
    pub const NODES_COUNT: usize = 15;

    pub fn populate_balanced_binary_tree() -> BinaryTreeNodeRef {
        /*
        node names:
                     n0
                /           \
              n1              n2
            /    \          /     \
          n3      n4       n5      n6
         /   \   /   \   /   \    /   \
        n7   n8 n9  n10 n11  n12 n13  n14

        left_child = parent * 2 + 1
        right_child = parent * 2 + 2 = left_child + 1
        parent_of_left_child = (left_child - 1)/2
        parent_of_right_child = (right_child - 2)/2

        position:
        is_left  = (n % 2) != 0
        is_rignt = (n % 2) == 0
        */

        let mut nodes = Vec::<BinaryTreeNodeRef>::with_capacity(NODES_COUNT);
        (0..NODES_COUNT).for_each(|n| {
            let node_ref = BinaryTree::new_node();

            // a new scope needed to drop the node variable before pushing the node_ref into vector
            // or
            // it requires writing:
            // node_ref.borrow_mut().name = format!("n{}", n);
            // node_ref.borrow_mut().value = n as u32;
            {
                let mut node = node_ref.borrow_mut();
                node.name = format!("n{}", n);
                node.data = n as u32;
            }
            nodes.push(node_ref)
        });

        (0..NODES_COUNT).for_each(|n| {
            if n != 0 {
                let parent_position = if n % 2 != 0 { 1 } else { 2 };
                let parent = (n - parent_position) / 2;
                nodes[n].borrow_mut().parent = Some(nodes[parent].clone());
            }

            let left_child = n * 2 + 1;
            if left_child < NODES_COUNT {
                nodes[n].borrow_mut().left = Some(nodes[left_child].clone());
            }

            let right_child = left_child + 1;
            if left_child < NODES_COUNT {
                nodes[n].borrow_mut().right = Some(nodes[right_child].clone());
            }
        });

        nodes[0].clone()
    }

    pub fn populate_balanced_binary_search_tree() -> BinaryTreeNodeRef {
        /*
        node values:
                       8
                /             \
              4                12
            /    \           /    \
           2       6       10      14
         /   \   /  \     /  \    /  \
        1     3 5    7   9   11  13   15
        */

        let node_values = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];

        let root = populate_balanced_binary_tree();
        let flatten = BinaryTree::flatten_top_down(root.clone());
        flatten.iter().enumerate().for_each(|v| {
            v.1.borrow_mut().data = node_values[v.0];
        });
        root
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::tree::binary_tree::{test_utils::*, BinaryTree};

    #[test]
    fn populate_test() {
        let root = populate_balanced_binary_tree();
        assert_eq!(root.borrow().name, "n0".to_string());

        let n0 = root.borrow();
        assert_eq!(n0.parent, None);

        let n1 = n0.left.as_ref().unwrap().borrow();
        assert_eq!(n1.name, "n1".to_string());
        assert_eq!(n1.parent.as_ref().unwrap().borrow().name, "n0".to_string());

        let n2 = n0.right.as_ref().unwrap().borrow();
        assert_eq!(n2.name, "n2".to_string());
        assert_eq!(n2.parent.as_ref().unwrap().borrow().name, "n0".to_string());

        let n3 = n1.left.as_ref().unwrap().borrow();
        assert_eq!(n3.name, "n3".to_string());
        assert_eq!(n3.parent.as_ref().unwrap().borrow().name, "n1".to_string());

        let n4 = n1.right.as_ref().unwrap().borrow();
        assert_eq!(n4.name, "n4".to_string());
        assert_eq!(n4.parent.as_ref().unwrap().borrow().name, "n1".to_string());

        let n5 = n2.left.as_ref().unwrap().borrow();
        assert_eq!(n5.name, "n5".to_string());
        assert_eq!(n5.parent.as_ref().unwrap().borrow().name, "n2".to_string());

        let n6 = n2.right.as_ref().unwrap().borrow();
        assert_eq!(n6.name, "n6".to_string());
        assert_eq!(n6.parent.as_ref().unwrap().borrow().name, "n2".to_string());

        let n7 = n3.left.as_ref().unwrap().borrow();
        assert_eq!(n7.name, "n7".to_string());
        assert_eq!(n7.parent.as_ref().unwrap().borrow().name, "n3".to_string());

        let n8 = n3.right.as_ref().unwrap().borrow();
        assert_eq!(n8.name, "n8".to_string());
        assert_eq!(n8.parent.as_ref().unwrap().borrow().name, "n3".to_string());

        let n9 = n4.left.as_ref().unwrap().borrow();
        assert_eq!(n9.name, "n9".to_string());
        assert_eq!(n9.parent.as_ref().unwrap().borrow().name, "n4".to_string());

        let n10 = n4.right.as_ref().unwrap().borrow();
        assert_eq!(n10.name, "n10".to_string());
        assert_eq!(n10.parent.as_ref().unwrap().borrow().name, "n4".to_string());

        let n11 = n5.left.as_ref().unwrap().borrow();
        assert_eq!(n11.name, "n11".to_string());
        assert_eq!(n11.parent.as_ref().unwrap().borrow().name, "n5".to_string());

        let n12 = n5.right.as_ref().unwrap().borrow();
        assert_eq!(n12.name, "n12".to_string());
        assert_eq!(n12.parent.as_ref().unwrap().borrow().name, "n5".to_string());

        let n13 = n6.left.as_ref().unwrap().borrow();
        assert_eq!(n13.name, "n13".to_string());
        assert_eq!(n13.parent.as_ref().unwrap().borrow().name, "n6".to_string());

        let n14 = n6.right.as_ref().unwrap().borrow();
        assert_eq!(n14.name, "n14".to_string());
        assert_eq!(n14.parent.as_ref().unwrap().borrow().name, "n6".to_string());
    }

    #[test]
    fn populate_test2() {
        let root = populate_balanced_binary_tree();
        let nodes = BinaryTree::flatten_top_down(root);
        let nodes_count = nodes.len();
        assert_eq!(nodes_count, NODES_COUNT);

        for (index, node_ref) in nodes.iter().enumerate() {
            let node = node_ref.borrow();
            assert_eq!(node.name, format!("n{index}"));
            if index == 0 {
                assert_eq!(node.parent.as_ref(), None);
            } else {
                let parent_position = if index % 2 != 0 { 1 } else { 2 };
                let parent = (index - parent_position) / 2;
                assert_eq!(
                    node.parent.as_ref().unwrap().borrow().name,
                    format!("n{}", parent)
                );
            }
            let left = index * 2 + 1;
            if left < nodes_count {
                assert_eq!(
                    node.left.as_ref().unwrap().borrow().name,
                    format!("n{}", left)
                );
            }
            let right = left + 1;
            if left < nodes_count {
                assert_eq!(
                    node.right.as_ref().unwrap().borrow().name,
                    format!("n{}", right)
                );
            }
        }
    }

    #[test]
    fn count() {
        let root = populate_balanced_binary_tree();
        let tree = BinaryTree::with_root(root);
        assert!(tree.root.is_some());
        assert_eq!(tree.count(), NODES_COUNT);
    }

    #[test]
    fn flatten_top_down() {
        let root = populate_balanced_binary_search_tree();

        let flatten = BinaryTree::flatten_top_down(root);
        assert_eq!(flatten.len(), NODES_COUNT);

        for (index, node_ref) in flatten.iter().enumerate() {
            assert_eq!(node_ref.borrow().name, format!("n{index}"));
        }
    }

    #[test]
    fn leftmost() {
        let expected = HashMap::from([
            ("n0", Some("n7")),
            ("n1", Some("n7")),
            ("n2", Some("n11")),
            ("n3", Some("n7")),
            ("n4", Some("n9")),
            ("n5", Some("n11")),
            ("n6", Some("n13")),
            ("n7", None),
            ("n8", None),
            ("n9", None),
            ("n10", None),
            ("n11", None),
            ("n12", None),
            ("n13", None),
            ("n14", None),
        ]);

        let root = populate_balanced_binary_tree();
        let flatten = BinaryTree::flatten_top_down(root);
        for node_ref in flatten {
            let node = node_ref.borrow();
            let expected = expected[node.name.as_str()];
            let leftmost = BinaryTree::leftmost(node_ref.clone());
            let name;
            let found = match leftmost {
                Some(left_ref) => {
                    let node = left_ref.borrow();
                    name = node.name.clone();
                    Some(name.as_str())
                }
                None => None,
            };
            assert_eq!(expected, found);
        }
    }

    #[test]
    fn flatten_left_to_right() {
        let expected = [
            "n7", "n3", "n8", "n1", "n9", "n4", "n10", "n0", "n11", "n5", "n12", "n2", "n13", "n6",
            "n14",
        ];
        println!("expected: {:?}", expected);

        let root = populate_balanced_binary_tree();
        let flatten_nodes: Vec<_> = BinaryTree::flatten_left_to_right(root.clone());
        assert_eq!(flatten_nodes.len(), expected.len());

        let flatten: Vec<_> = flatten_nodes
            .iter()
            .map(|n| n.borrow().name.clone())
            .collect();
        // println!("flatten: {:?}", flatten);
        assert_eq!(flatten.len(), expected.len());


        for (index, node) in flatten.iter().enumerate() {
            assert_eq!(node, expected[index]);
        }
    }
}
