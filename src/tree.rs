// https://rust-unofficial.github.io/too-many-lists/

use std::{rc::{Rc, Weak}, cell::{RefCell, Cell}, borrow::BorrowMut, ops::{DerefMut, Deref}};
use uuid::Uuid;

#[derive(Debug)]
pub struct Tree {
    pub root: Option<TreeNodeRef>,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub uuid: Uuid,
    pub value: u32,
    pub data: String,
    pub parent: RefCell<Weak<TreeNode>>,
    pub children: RefCell<Vec<TreeNodeRef>>,
}

impl Deref for TreeNode {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for TreeNode {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.data
    }
}

type TreeNodeRef = Rc<TreeNode>;

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            uuid: Uuid::new_v4(),
            value: 0,
            data: "".to_string(),
            parent: RefCell::new(Weak::<TreeNode>::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub fn add_child(self, child: TreeNode) -> Rc<Self>{

        let rc = Rc::new(self);
        *child.parent.borrow_mut() = Rc::downgrade(&rc);

        let child_ref = Rc::new(child);
        rc.children.borrow_mut().push(child_ref);
        rc
    }

}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn add_child(&self, parent: TreeNodeRef, child: TreeNode) {
        *child.parent.borrow_mut() = Rc::downgrade(&parent);

        let child_ref = Rc::new(child);
        parent.children.borrow_mut().push(child_ref);
    }

    pub fn remove(_uuid: Uuid) {
        
    }

    pub fn search(_uuid: Uuid) -> Option<TreeNode> {
        None
    }

    pub fn flatten() -> Vec<TreeNode> {
        Vec::<TreeNode>::new()
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

        let root_ref = Rc::new(root);
        tree.root = Some(root_ref.clone());

        let mut node1 = TreeNode::new();
        node1.data = "node1".to_string();
        tree.add_child(root_ref.clone(), node1);

        let mut node2 = TreeNode::new();
        node2.data = "node2".to_string();
        tree.add_child(root_ref.clone(), node2);

        let mut node3 = TreeNode::new();
        node3.data = "node3".to_string();
        tree.add_child(root_ref.clone(), node3);

        let mut node4 = TreeNode::new();
        node4.data = "node4".to_string();

        let mut node4_1 = TreeNode::new();
        let mut node4_2 = TreeNode::new();
        //let a1 = &ref_node4_1.data;

        //let mut ref_node4_2 = Rc::new(TreeNode::new());
        //(*ref_node4_2.borrow_mut()).data = "ref_node4_2".to_string();

        // let t = node4.add_child(node4_1).clone();
        // t.into().add_child(node4_2);

        tree.add_child(root_ref.clone(), node4);

        println!("{:#?}", tree);
    }
}