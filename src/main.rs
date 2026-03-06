use std::cell::Ref;
mod bptree;
use bptree::BPlusTree;
use bptree::NodeType;

fn main() {
    let mut tree: BPlusTree<String> = BPlusTree::new();

    println!("Building the B+ Tree...");

    for i in 1..=100 {
        tree.insert(i, format!("Data {}", i));
    }

    println!("\nTree built successfully! Here is the structure before deletion:");
    println!("--------------------------------------------------");
    tree.print_recursive();
    println!("--------------------------------------------------");

    println!("\nDeleting keys 40 through 60...");
    for i in 40..=60 {
        tree.delete(i);
    }

    println!("\nDeletions complete! Here is the newly balanced tree:");
    println!("--------------------------------------------------");
    tree.print_recursive();
    println!("--------------------------------------------------");

    println!("\nRunning integrity checks...");

    if tree.search(45).is_none() {
        println!("✔️  Success: Key 45 (Deleted) is completely gone.");
    } else {
        println!("❌  Error: Key 45 is still in the tree!");
    }

    if let Some(_) = tree.search(80) {
        println!("Success: Key 80 (Untouched) is still safely stored.");
    } else {
        println!("Error: Key 80 was accidentally destroyed!");
    }
}
