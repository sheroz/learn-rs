use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

struct BinaryTreeNode {
    id: Uuid,
    value: u32,
    parent: Option<BinaryTreeNodeRef>,
    left: Option<BinaryTreeNodeRef>,
    right: Option<BinaryTreeNodeRef>,
}

type BinaryTreeNodeRef = Rc<RefCell<BinaryTreeNode>>;

struct BinaryTree {
    root: Option<BinaryTreeNodeRef>,
}

impl BinaryTree {
    pub fn new() -> BinaryTreeNodeRef {
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

    fn populate() {}
}
