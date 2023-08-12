// https://rust-unofficial.github.io/too-many-lists/
// https://doc.rust-lang.org/nightly/nomicon/

use std::{rc::{Rc, Weak}, cell::{RefCell, Cell}, borrow::BorrowMut, ops::{DerefMut, Deref}};
use uuid::Uuid;

#[derive(Debug)]
pub struct Tree {
    pub root: TreeNodeRef,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub uuid: Uuid,
    pub value: u32,
    pub data: String,
    pub parent: TreeNodeRef,
    pub children: Rc<RefCell<Vec<TreeNode>>>,
}

type TreeNodeRef = Rc<RefCell<Option<TreeNode>>>;

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            uuid: Uuid::new_v4(),
            value: 0,
            data: "".to_string(),
            parent: Rc::new(RefCell::new(None)),
            children: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: Rc::new(RefCell::new(None)) }
    }

    pub fn add_child(&self, parent: TreeNodeRef, mut child: TreeNode) {
        if let Some(node) = parent.borrow().as_ref() {
            child.parent = parent.clone();

            let mut ref_mut = node.children.as_ref().borrow_mut();
            let children: &mut Vec<TreeNode> = ref_mut.as_mut();
            children.push(child);
        }
    }

    pub fn remove(_uuid: Uuid) {
        
    }

    pub fn search(_uuid: Uuid) -> Option<TreeNode> {
        unimplemented!()
    }

    pub fn flatten() -> Vec<TreeNode> {
        unimplemented!()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_child() {


        let mut tree = Tree::new();

        let mut root = TreeNode::new();
        root.data = "root".to_string();

        let root_ref = Rc::new(RefCell::new(Some(root)));
        tree.root = root_ref.clone();

        let mut node1 = TreeNode::new();
        node1.data = "node1".to_string();
        tree.add_child(root_ref.clone(), node1);

        let mut node2 = TreeNode::new();
        node2.data = "node2".to_string();
        tree.add_child(root_ref.clone(), node2);

        let mut node3 = TreeNode::new();
        node3.data = "node3".to_string();
        tree.add_child(root_ref.clone(), node3);

        // let mut node4 = TreeNode::new();
        // node4.data = "node4".to_string();

        // let mut node4_1 = TreeNode::new();
        // let mut node4_2 = TreeNode::new();
        //let a1 = &ref_node4_1.data;

        //let mut ref_node4_2 = Rc::new(TreeNode::new());
        //(*ref_node4_2.borrow_mut()).data = "ref_node4_2".to_string();
        // let parent = Rc::new(node4);
        // parent.add_child(parent.clone(), node4_1);
        // parent.add_child(parent.clone(), node4_2);

        //tree.add_child(root_ref.clone(), parent.try_into().unwrap());

        println!("{:#?}", tree);
    }
}