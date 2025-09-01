use binary_tree::BinaryTree;
use std::fmt::Debug;

fn print_tree_info<T: Ord + Debug + Clone>(tree: &BinaryTree<T>) {
    println!("Is empty: {}", tree.is_empty());
    
    if let Some(min) = tree.min_value() {
        println!("Minimum value: {:?}", min);
    }
    
    if let Some(max) = tree.max_value() {
        println!("Maximum value: {:?}", max);
    }
    
    println!("Tree height: {}", tree.height());
    
    println!("\nTraversals:");
    println!("Inorder: {:?}", tree.inorder_traversal());
    println!("Preorder: {:?}", tree.preorder_traversal());
    println!("Postorder: {:?}", tree.postorder_traversal());
}

fn main() {
    // Create a tree with integers
    println!("Creating an integer binary tree...");
    let mut int_tree = BinaryTree::new();
    
    // Insert some values
    println!("Inserting values: 10, 5, 15, 3, 7, 12, 20");
    int_tree.insert(10);
    int_tree.insert(5);
    int_tree.insert(15);
    int_tree.insert(3);
    int_tree.insert(7);
    int_tree.insert(12);
    int_tree.insert(20);
    
    println!("\nInteger Tree Information:");
    print_tree_info(&int_tree);
    
    // Search for values
    println!("\nSearching for values:");
    println!("Contains 7: {}", int_tree.search(&7));
    println!("Contains 11: {}", int_tree.search(&11));
    
    // Delete some values
    println!("\nDeleting value 5...");
    int_tree.delete(&5);
    println!("Contains 5 after deletion: {}", int_tree.search(&5));
    
    // Create a tree with strings
    println!("\nCreating a string binary tree...");
    let mut string_tree = BinaryTree::new();
    
    // Insert some values
    println!("Inserting values: \"apple\", \"banana\", \"cherry\", \"date\", \"elderberry\"");
    string_tree.insert("apple".to_string());
    string_tree.insert("banana".to_string());
    string_tree.insert("cherry".to_string());
    string_tree.insert("date".to_string());
    string_tree.insert("elderberry".to_string());
    
    println!("\nString Tree Information:");
    print_tree_info(&string_tree);
    
    // Search for values
    println!("\nSearching for values:");
    println!("Contains \"cherry\": {}", string_tree.search(&"cherry".to_string()));
    println!("Contains \"fig\": {}", string_tree.search(&"fig".to_string()));
    
    // Delete some values
    println!("\nDeleting value \"banana\"...");
    string_tree.delete(&"banana".to_string());
    println!("Contains \"banana\" after deletion: {}", string_tree.search(&"banana".to_string()));
}
