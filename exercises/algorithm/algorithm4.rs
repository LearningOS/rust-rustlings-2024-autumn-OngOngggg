/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::new(value.clone()));
        match self.root{
            Some(ref mut node) =>{
                node.insert(value);
            },
            None =>{
                self.root = Some(new_node);
            },
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        fn search_node<T: Ord>(node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
            match node {
                None => false, 
                Some(ref boxed_node) => {
                    if value == boxed_node.value {
                        true 
                    } else if value < boxed_node.value {
                        search_node(&boxed_node.left, value) 
                    } else {
                        search_node(&boxed_node.right, value) 
                    }
                }
            }
        }

        // 从根节点开始搜索
        search_node(&self.root, value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value<self.value{
            match self.left{
                None =>{
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
                Some(ref mut left_node) => {
                    left_node.insert(value);
                }
            }
        }
        else if value>self.value{
            match self.right{
                None =>{
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
                Some(ref mut right_node) => {
                    right_node.insert(value);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


