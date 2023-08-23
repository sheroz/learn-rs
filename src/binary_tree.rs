use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

pub struct BinaryTreeNode {
    id: Uuid,
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
        BinaryTree{root: None}
    }

    pub fn new_node() -> BinaryTreeNodeRef {
        Rc::new(RefCell::new(BinaryTreeNode {
            id: Uuid::new_v4(),
            value: 0,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populate() {
        /*  
                     n0 
                /           \
              n1              n2
            /    \          /     \
          n3      n4       n5      n6
         /   \   /   \   /   \    /   \
        n7   n8 n9  n10 n11  n12 n13  n14

        */

        let mut tree = BinaryTree::new();
        const NODES_COUNT: usize = 15;
        let mut nodes = Vec::<BinaryTreeNodeRef>::with_capacity(NODES_COUNT);
        (0..NODES_COUNT).for_each(|_| nodes.push(BinaryTree::new_node()));

        tree.root = Some(nodes[0].clone());

        nodes[1].borrow_mut().parent = Some(nodes[0].clone());
        nodes[2].borrow_mut().parent = Some(nodes[0].clone());
        nodes[0].borrow_mut().left = Some(nodes[1].clone());
        nodes[0].borrow_mut().right = Some(nodes[2].clone());

        nodes[3].borrow_mut().parent = Some(nodes[1].clone());
        nodes[4].borrow_mut().parent = Some(nodes[1].clone());
        nodes[1].borrow_mut().left = Some(nodes[3].clone());
        nodes[1].borrow_mut().right = Some(nodes[4].clone());

        nodes[5].borrow_mut().parent = Some(nodes[2].clone());
        nodes[6].borrow_mut().parent = Some(nodes[2].clone());
        nodes[2].borrow_mut().left = Some(nodes[5].clone());
        nodes[2].borrow_mut().right = Some(nodes[6].clone());

        nodes[7].borrow_mut().parent = Some(nodes[3].clone());
        nodes[8].borrow_mut().parent = Some(nodes[3].clone());
        nodes[3].borrow_mut().left = Some(nodes[7].clone());
        nodes[3].borrow_mut().right = Some(nodes[8].clone());

        nodes[9].borrow_mut().parent = Some(nodes[4].clone());
        nodes[10].borrow_mut().parent = Some(nodes[4].clone());
        nodes[4].borrow_mut().left = Some(nodes[9].clone());
        nodes[4].borrow_mut().right = Some(nodes[10].clone());

        nodes[11].borrow_mut().parent = Some(nodes[5].clone());
        nodes[12].borrow_mut().parent = Some(nodes[5].clone());
        nodes[5].borrow_mut().left = Some(nodes[11].clone());
        nodes[5].borrow_mut().right = Some(nodes[12].clone());

        nodes[13].borrow_mut().parent = Some(nodes[6].clone());
        nodes[14].borrow_mut().parent = Some(nodes[6].clone());
        nodes[6].borrow_mut().left = Some(nodes[13].clone());
        nodes[6].borrow_mut().right = Some(nodes[14].clone());
    }

    #[test]
    fn binary_tree_populate_test() {
        populate();
    }
}
