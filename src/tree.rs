// https://rust-unofficial.github.io/too-many-lists/
// https://doc.rust-lang.org/nightly/nomicon/
// https://manishearth.github.io/

use std::cell::RefCell;
use std::rc::Rc;
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
    pub children: Option<Vec<TreeNodeRef>>,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub trait TreeNodeRefMethods {
    fn build_from(item: TreeNode) -> TreeNodeRef;
}

impl TreeNodeRefMethods for TreeNodeRef {
    fn build_from(item: TreeNode) -> Self {
        Rc::new(RefCell::new(item))
    }
}

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

    pub fn add_child(&self, parent: TreeNodeRef, child: TreeNodeRef) {
        child.as_ref().borrow_mut().parent = Some(parent.clone());

        let mut node = parent.as_ref().borrow_mut();
        match node.children.as_mut() {
            Some(children) => {
                children.push(child);
            },
            None => {
                node.children = Some(vec![child]);
            }
        }
    }

    pub fn count(&self) -> u64 {
        let mut count = 0;
        if let Some(node) = &self.root {
            let mut queue = vec![node.clone()];
            while !queue.is_empty() {
                if let Some(node) = queue.pop() {
                    count += 1;
                    let children_ref = node.as_ref().borrow();
                    if let Some(children) = children_ref.children.as_ref() {
                        children.iter().for_each(|v| queue.push(v.clone()));
                    }
                }
            }
        }
        count
    }

    pub fn flatten(&self) -> Vec<TreeNodeRef> {
        let mut flatten = Vec::<TreeNodeRef>::new();

        if let Some(node) = &self.root {
            let mut queue = vec![node.clone()];
            while !queue.is_empty() {
                if let Some(node) = queue.pop() {
                    flatten.push(node.clone());
                    let children_ref = node.as_ref().borrow();
                    if let Some(children) = children_ref.children.as_ref() {
                        children.iter().for_each(|v| queue.push(v.clone()));
                    }
                }
            }
        }
        flatten
    }

    pub fn remove(&mut self, uuid: Uuid) -> Option<TreeNodeRef> {
        if let Some(node) = self.search(uuid) {
            match node.as_ref().borrow().parent.as_ref() {
                Some(parent_ref) => {
                    let parent = parent_ref.as_ref();
                    if let Some(children) = parent.borrow_mut().children.as_mut() {
                        let mut node_found = None;
                        for (index, child) in children.iter().enumerate() {
                            if child.as_ref().borrow().uuid == uuid {
                                node_found = Some(index);
                                break;
                            }
                        }
                        if let Some(index) = node_found {
                            return Some(children.remove(index));
                        }
                    }
                }
                None => {
                    self.root = None;
                }
            }
        }
        None
    }

    pub fn search(&self, uuid: Uuid) -> Option<TreeNodeRef> {
        if let Some(node) = &self.root {
            let mut queue = vec![node.clone()];
            while !queue.is_empty() {
                if let Some(node) = queue.pop() {
                    if uuid == node.as_ref().borrow().uuid {
                        return Some(node.clone());
                    }
                    let children_ref = node.as_ref().borrow();
                    if let Some(children) = children_ref.children.as_ref() {
                        children.iter().for_each(|v| queue.push(v.clone()));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn populate() -> Tree {
        let mut tree = Tree::new();

        let mut root = TreeNode::new();
        root.text = "root".to_string();

        let root_ref = TreeNodeRef::build_from(root);
        tree.root = Some(root_ref.clone());

        let mut node1 = TreeNode::new();
        node1.text = "node1".to_string();
        let node1_ref = TreeNodeRef::build_from(node1);
        tree.add_child(root_ref.clone(), node1_ref);

        let mut node2 = TreeNode::new();
        node2.text = "node2".to_string();
        let node2_ref = TreeNodeRef::build_from(node2);
        tree.add_child(root_ref.clone(), node2_ref);

        let mut node3 = TreeNode::new();
        node3.text = "node3".to_string();
        let node3_ref = TreeNodeRef::build_from(node3);
        tree.add_child(root_ref.clone(), node3_ref);

        let mut node4 = TreeNode::new();
        node4.text = "node4".to_string();

        let mut node41 = TreeNode::new();
        node41.text = "node41".to_string();
        let node41_ref = TreeNodeRef::build_from(node41);

        let mut node42 = TreeNode::new();
        node42.text = "node42".to_string();
        let node42_ref = TreeNodeRef::build_from(node42);

        let node4_ref = TreeNodeRef::build_from(node4);
        tree.add_child(node4_ref.clone(), node41_ref);
        tree.add_child(node4_ref.clone(), node42_ref);

        tree.add_child(root_ref, node4_ref);

        tree
        // println!("{:#?}", tree);
    }

    #[test]
    fn populate_test() {
        populate();
    }

    #[test]
    fn count() {
        let tree = populate();
        assert_eq!(tree.count(), 7);
    }

    #[test]
    fn flatten() {
        let tree = populate();
        let flatten = tree.flatten();
        assert_eq!(flatten.len(), 7);
        let out = flatten
            .iter()
            .map(|v| {
                format!(
                    "{}:{}",
                    v.as_ref().borrow().uuid.to_string(),
                    v.as_ref().borrow().text.clone()
                )
            })
            .collect::<Vec<_>>();
        println!("{}", out.join("\n"));
    }

    #[test]
    fn search() {
        let tree = populate();
        let flatten = tree.flatten();
        let node_ref = flatten[2].as_ref();
        let uuid_for_search = node_ref.borrow().uuid.to_string();
        println!("Looking for:\n{}", uuid_for_search);

        let uuid = Uuid::parse_str(&uuid_for_search).unwrap();

        let r = tree.search(uuid);
        assert!(r.is_some());

        let v = r.unwrap();
        let out = format!(
            "{}:{}",
            v.as_ref().borrow().uuid.to_string(),
            v.as_ref().borrow().text.clone()
        );
        println!("Found:\n{}", out);
    }

    #[test]
    fn remove() {
        let mut tree = populate();
        let count = tree.count();
        let flatten = tree.flatten();
        let out = flatten
            .iter()
            .map(|v| {
                format!(
                    "{}:{}",
                    v.as_ref().borrow().uuid.to_string(),
                    v.as_ref().borrow().text.clone()
                )
            })
            .collect::<Vec<_>>();
        println!("Tree nodes:\n{}", out.join("\n"));

        let node_ref = flatten[2].as_ref();
        let uuid_for_search = node_ref.borrow().uuid.to_string();
        println!("Looking for:\n{}", uuid_for_search);

        let uuid = Uuid::parse_str(&uuid_for_search).unwrap();

        let r = tree.remove(uuid);
        assert!(r.is_some());

        let v = r.unwrap();
        let out = format!(
            "{}:{}",
            v.as_ref().borrow().uuid.to_string(),
            v.as_ref().borrow().text.clone()
        );
        println!("Removed:\n{}", out);

        let flatten = tree.flatten();
        let out = flatten
            .iter()
            .map(|v| {
                format!(
                    "{}:{}",
                    v.as_ref().borrow().uuid.to_string(),
                    v.as_ref().borrow().text.clone()
                )
            })
            .collect::<Vec<_>>();
        println!("Tree nodes:\n{}", out.join("\n"));
        assert!(count > tree.count());
    }
}
