// https://rust-unofficial.github.io/too-many-lists/
// https://doc.rust-lang.org/nightly/nomicon/

use std::rc::Rc;
use std::cell::RefCell;
use uuid::Uuid;

#[derive(Debug)]
pub struct Tree {
    pub root: Option<TreeNodeRef>,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub uuid: Uuid,
    pub number: u32,
    pub text: String,
    pub parent: Option<TreeNodeRef>,
    pub children: Option<Vec<TreeNodeRef>>
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            uuid: Uuid::new_v4(),
            number: 0,
            text: "".to_string(),
            parent: None,
            children: None,
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn into_node_ref(node: TreeNode) -> TreeNodeRef {
        Rc::new(RefCell::new(node))
    }

    pub fn add_child(&self, parent: TreeNodeRef, child: TreeNodeRef) {
        child.as_ref().borrow_mut().parent = Some(parent.clone());

        let mut node = parent.as_ref().borrow_mut();
        if node.children.is_some() {
            node.children.as_mut().unwrap().push(child);
        } else {
            node.children = Some(vec![child]);
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
        root.text = "root".to_string();

        let root_ref = Rc::new(RefCell::new(root));
        tree.root = Some(root_ref.clone());

        let mut node1 = TreeNode::new();
        node1.text = "node1".to_string();
        let node1_ref = Tree::into_node_ref(node1);
        tree.add_child(root_ref.clone(), node1_ref);

        let mut node2 = TreeNode::new();
        node2.text = "node2".to_string();
        let node2_ref = Tree::into_node_ref(node2);
        tree.add_child(root_ref.clone(), node2_ref);

        let mut node3 = TreeNode::new();
        node3.text = "node3".to_string();
        let node3_ref = Tree::into_node_ref(node3);
        tree.add_child(root_ref.clone(), node3_ref);

        let mut node4 = TreeNode::new();
        node4.text = "node4".to_string();

        let mut node41 = TreeNode::new();
        node41.text = "node41".to_string();
        let node41_ref = Tree::into_node_ref(node41);

        let mut node42 = TreeNode::new();
        node42.text = "node42".to_string();
        let node42_ref = Tree::into_node_ref(node42);

        let node4_ref = Rc::new(RefCell::new(node4));
        tree.add_child(node4_ref.clone(), node41_ref);
        tree.add_child(node4_ref.clone(), node42_ref);

        tree.add_child(root_ref, node4_ref);

        // println!("{:#?}", tree);
    }
}