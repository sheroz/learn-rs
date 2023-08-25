use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct BinaryTreeNode {
    pub id: Uuid,
    pub name: String,
    pub value: u32,
    pub parent: Option<BinaryTreeNodeRef>,
    pub left: Option<BinaryTreeNodeRef>,
    pub right: Option<BinaryTreeNodeRef>,
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

    pub fn flatten(&self) -> Vec<BinaryTreeNodeRef> {
        let mut nodes = Vec::new();
        let mut queue = VecDeque::new();
        let node = self.root.as_ref().unwrap();

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
}

#[cfg(test)]
pub mod test_utils {

    use super::*;
    pub const NODES_COUNT: usize = 15;

    pub fn populate_balanced_binary_tree() -> BinaryTree {
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
        is_left  = (n % 2) != 0
        is_rignt = (n % 2) == 0
        */

        let mut tree = BinaryTree::new();
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
                node.value = n as u32;
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

        tree.root = Some(nodes[0].clone());
        tree
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::binary_tree::test_utils::*;

    #[test]
    fn populate_test() {
        let tree = populate_balanced_binary_tree();
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
    fn populate_test2() {
        let tree = populate_balanced_binary_tree();
        assert!(tree.root.is_some());
        let nodes = tree.flatten();
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
        let tree = populate_balanced_binary_tree();
        assert!(tree.root.is_some());
        assert_eq!(tree.count(), NODES_COUNT);
    }

    #[test]
    fn flatten() {
        let tree = populate_balanced_binary_tree();
        assert!(tree.root.is_some());

        let nodes = tree.flatten();
        assert_eq!(nodes.len(), NODES_COUNT);

        for (index, node_ref) in nodes.iter().enumerate() {
            assert_eq!(node_ref.borrow().name, format!("n{index}"));
        }
    }
}
