use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct BinaryTreeNode {
    id: Uuid,
    name: String,
    value: u32,
    parent: Option<BinaryTreeNodeRef>,
    left: Option<BinaryTreeNodeRef>,
    right: Option<BinaryTreeNodeRef>,
}

pub type BinaryTreeNodeRef = Rc<RefCell<BinaryTreeNode>>;

pub struct BinaryTree {
    pub root: Option<BinaryTreeNodeRef>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn new_node() -> BinaryTreeNodeRef {
        Rc::new(RefCell::new(BinaryTreeNode {
            id: Uuid::new_v4(),
            name: "".to_string(),
            value: 0,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    const NODES_COUNT: usize = 15;

    fn populate() -> BinaryTree {
        /*
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
        is_left = n%2 != 0
        is_rignt = n%2 == 0
        */

        let mut tree = BinaryTree::new();
        let mut nodes = Vec::<BinaryTreeNodeRef>::with_capacity(NODES_COUNT);
        (0..NODES_COUNT).for_each(|n| {
            let node = BinaryTree::new_node();
            node.borrow_mut().name = format!("n{}", n);
            nodes.push(node)
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

        tree.root = Some(nodes[0].clone());
        tree
    }

    #[test]
    fn binary_tree_populate_test() {
        let tree = populate();
        assert!(tree.root.is_some());
        let root = tree.root.unwrap();
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
    fn binary_tree_populate_test2() {
        let tree = populate();
        assert!(tree.root.is_some());
        let mut nodes = Vec::<BinaryTreeNodeRef>::with_capacity(NODES_COUNT);
        let mut queue = VecDeque::new();
        let root = tree.root.unwrap();

        queue.push_back(root.clone());
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

        assert_eq!(nodes.len(), NODES_COUNT);

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
            if left < NODES_COUNT {
                assert_eq!(
                    node.left.as_ref().unwrap().borrow().name,
                    format!("n{}", left)
                );
            }
            let right = left + 1;
            if left < NODES_COUNT {
                assert_eq!(
                    node.right.as_ref().unwrap().borrow().name,
                    format!("n{}", right)
                );
            }
        }
    }
}
