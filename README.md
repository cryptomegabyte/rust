# Ali Kayani's Rust Portfolio

Welcome to my Rust programming portfolio! This repository contains various Rust projects that demonstrate my skills and knowledge in Rust programming language.

## Projects

### Binary Tree

A generic binary search tree implementation in Rust that supports various operations:

- **Insert**: Add values to the tree
- **Search**: Find values in the tree
- **Delete**: Remove values from the tree
- **Min/Max**: Find minimum and maximum values in the tree

The implementation is generic and works with any type that implements `Ord`, `Debug`, and `Clone` traits.

**Key Features:**

- Generic implementation supporting any comparable type
- Comprehensive test suite
- Example demonstrating usage with both integers and strings

[View Binary Tree Project](./binary_tree/)

## Running the Projects

Each project has its own README with specific instructions, but generally:

```bash
# Clone the repository
git clone <repository-url>
cd rust

# Navigate to a specific project
cd binary_tree

# Run tests
cargo test

# Run examples
cargo run --example binary_tree_example
```

## Skills Demonstrated

- Generic programming in Rust
- Ownership and borrowing patterns
- Recursive algorithms
- Test-driven development
- Documentation
