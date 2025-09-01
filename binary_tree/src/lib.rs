use std::cmp::Ordering;
use std::fmt::Debug;

/// A binary tree node that can store any comparable value
#[derive(Debug, Clone)]
pub struct Node<T: Ord + Debug + Clone> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

/// Binary search tree implementation
#[derive(Debug, Clone)]
pub struct BinaryTree<T: Ord + Debug + Clone> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug + Clone> Node<T> {
    /// Create a new node with the given value
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord + Debug + Clone> BinaryTree<T> {
    // Tree traversal methods
    
    /// Performs an inorder traversal of the tree (Left-Root-Right)
    /// Returns a vector of references to values in sorted order
    pub fn inorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_traversal_recursive(&self.root, &mut result);
        result
    }
    
    fn inorder_traversal_recursive<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node_ref) = node {
            // Visit left subtree
            Self::inorder_traversal_recursive(&node_ref.left, result);
            // Visit root
            result.push(&node_ref.value);
            // Visit right subtree
            Self::inorder_traversal_recursive(&node_ref.right, result);
        }
    }
    
    /// Performs a preorder traversal of the tree (Root-Left-Right)
    pub fn preorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::preorder_traversal_recursive(&self.root, &mut result);
        result
    }
    
    fn preorder_traversal_recursive<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node_ref) = node {
            // Visit root
            result.push(&node_ref.value);
            // Visit left subtree
            Self::preorder_traversal_recursive(&node_ref.left, result);
            // Visit right subtree
            Self::preorder_traversal_recursive(&node_ref.right, result);
        }
    }
    
    /// Performs a postorder traversal of the tree (Left-Right-Root)
    pub fn postorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::postorder_traversal_recursive(&self.root, &mut result);
        result
    }
    
    fn postorder_traversal_recursive<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node_ref) = node {
            // Visit left subtree
            Self::postorder_traversal_recursive(&node_ref.left, result);
            // Visit right subtree
            Self::postorder_traversal_recursive(&node_ref.right, result);
            // Visit root
            result.push(&node_ref.value);
        }
    }
    
    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        Self::height_recursive(&self.root)
    }
    
    fn height_recursive(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            Some(node_ref) => {
                let left_height = Self::height_recursive(&node_ref.left);
                let right_height = Self::height_recursive(&node_ref.right);
                1 + left_height.max(right_height)
            },
            None => 0
        }
    }
    /// Create a new empty binary tree
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Insert a value into the tree
    pub fn insert(&mut self, value: T) {
        let root = &mut self.root;
        
        if root.is_none() {
            *root = Some(Box::new(Node::new(value)));
            return;
        }
        
        Self::insert_recursive(root, value);
    }
    
    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) {
        if let Some(node_ref) = node {
            match value.cmp(&node_ref.value) {
                Ordering::Less => {
                    if node_ref.left.is_none() {
                        node_ref.left = Some(Box::new(Node::new(value)));
                    } else {
                        Self::insert_recursive(&mut node_ref.left, value);
                    }
                },
                Ordering::Greater => {
                    if node_ref.right.is_none() {
                        node_ref.right = Some(Box::new(Node::new(value)));
                    } else {
                        Self::insert_recursive(&mut node_ref.right, value);
                    }
                },
                Ordering::Equal => {
                    // Value already exists, do nothing or update
                    // Here we choose to do nothing
                }
            }
        }
    }

    /// Search for a value in the tree
    pub fn search(&self, value: &T) -> bool {
        Self::search_recursive(&self.root, value)
    }
    
    fn search_recursive(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        if let Some(node_ref) = node {
            match value.cmp(&node_ref.value) {
                Ordering::Less => Self::search_recursive(&node_ref.left, value),
                Ordering::Greater => Self::search_recursive(&node_ref.right, value),
                Ordering::Equal => true,
            }
        } else {
            false
        }
    }

    /// Get the minimum value in the tree
    pub fn min_value(&self) -> Option<&T> {
        Self::min_value_recursive(&self.root)
    }
    
    fn min_value_recursive(node: &Option<Box<Node<T>>>) -> Option<&T> {
        if let Some(node_ref) = node {
            if node_ref.left.is_none() {
                Some(&node_ref.value)
            } else {
                Self::min_value_recursive(&node_ref.left)
            }
        } else {
            None
        }
    }

    /// Get the maximum value in the tree
    pub fn max_value(&self) -> Option<&T> {
        Self::max_value_recursive(&self.root)
    }
    
    fn max_value_recursive(node: &Option<Box<Node<T>>>) -> Option<&T> {
        if let Some(node_ref) = node {
            if node_ref.right.is_none() {
                Some(&node_ref.value)
            } else {
                Self::max_value_recursive(&node_ref.right)
            }
        } else {
            None
        }
    }
    

    /// Delete a value from the tree
    pub fn delete(&mut self, value: &T) {
        self.root = Self::delete_recursive(self.root.take(), value);
    }
    
    fn delete_recursive(node: Option<Box<Node<T>>>, value: &T) -> Option<Box<Node<T>>> {
        if let Some(mut boxed_node) = node {
            if value < &boxed_node.value {
                boxed_node.left = Self::delete_recursive(boxed_node.left, value);
                Some(boxed_node)
            } else if value > &boxed_node.value {
                boxed_node.right = Self::delete_recursive(boxed_node.right, value);
                Some(boxed_node)
            } else {
                // Node to delete found
                
                // Case 1: No children or only one child
                if boxed_node.left.is_none() {
                    return boxed_node.right;
                } else if boxed_node.right.is_none() {
                    return boxed_node.left;
                }
                
                // Case 2: Two children
                // Find the inorder successor (smallest in right subtree)
                let mut successor = boxed_node.right.as_mut().unwrap();
                while successor.left.is_some() {
                    successor = successor.left.as_mut().unwrap();
                }
                
                // Copy the successor's value
                let successor_value = successor.value.clone();
                boxed_node.value = successor_value;
                
                // Delete the successor
                boxed_node.right = Self::delete_recursive(boxed_node.right, &boxed_node.value);
                
                Some(boxed_node)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree_is_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_insert_and_search() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        
        assert!(!tree.is_empty());
        assert!(tree.search(&5));
        assert!(tree.search(&3));
        assert!(tree.search(&7));
        assert!(!tree.search(&1));
    }

    #[test]
    fn test_min_max_values() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.min_value(), None);
        assert_eq!(tree.max_value(), None);
        
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);
        
        assert_eq!(tree.min_value(), Some(&1));
        assert_eq!(tree.max_value(), Some(&9));
    }

    #[test]
    fn test_delete() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);
        
        assert!(tree.search(&3));
        tree.delete(&3);
        assert!(!tree.search(&3));
        
        assert!(tree.search(&5));
        tree.delete(&5);
        assert!(!tree.search(&5));
        
        // Check that the tree is still valid
        assert!(tree.search(&1));
        assert!(tree.search(&7));
        assert!(tree.search(&9));
    }
    
    #[test]
    fn test_inorder_traversal() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);
        
        let result = tree.inorder_traversal();
        assert_eq!(result, vec![&1, &3, &5, &7, &9]);
    }
    
    #[test]
    fn test_preorder_traversal() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);
        
        // The exact preorder traversal depends on the insertion order and tree structure
        // For our implementation with the given insertion order, it should be:
        let result = tree.preorder_traversal();
        assert_eq!(result, vec![&5, &3, &1, &7, &9]);
    }
    
    #[test]
    fn test_postorder_traversal() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);
        
        // The exact postorder traversal depends on the insertion order and tree structure
        // For our implementation with the given insertion order, it should be:
        let result = tree.postorder_traversal();
        assert_eq!(result, vec![&1, &3, &9, &7, &5]);
    }
    
    #[test]
    fn test_height() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.height(), 0);
        
        tree.insert(5);
        assert_eq!(tree.height(), 1);
        
        tree.insert(3);
        assert_eq!(tree.height(), 2);
        
        tree.insert(7);
        assert_eq!(tree.height(), 2);
        
        tree.insert(1);
        assert_eq!(tree.height(), 3);
    }
}
