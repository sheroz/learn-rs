use crate::tree::binary_tree::*;

pub struct BinarySearchTree {
    pub tree: BinaryTree,
}

impl BinarySearchTree {
    pub fn new(tree: BinaryTree) -> Self {
        BinarySearchTree { tree }
    }

    pub fn is_binary_search_tree(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::binary_tree::test_utils::*;

    #[test]
    fn populate_tree() {
        let tree = populate();
        assert_eq!(tree.count(), NODES_COUNT);
        let bst = BinarySearchTree::new(tree);
        assert!(bst.is_binary_search_tree());
    }
}
