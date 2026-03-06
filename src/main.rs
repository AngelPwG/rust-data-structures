use std::cell::Ref;
mod bptree;
use bptree::BPlusTree;
use bptree::NodeType;

fn main() {
    let mut tree: BPlusTree<String> = BPlusTree::new();

    println!("🌲 Building the B+ Tree...");
    for i in 1..=100 {
        let value = format!("Data payload for key {}", i);
        tree.insert(i, value);
    }
    
    println!("✅ Successfully inserted 100 items, forcing node splits!");

    let target_key = 42;
    println!("🔍 Searching for key {}...", target_key);

    if let Some((node_rc, index)) = tree.search(target_key) {
        let locked_node = node_rc.borrow();
        
        let value_ref = Ref::map(locked_node, |node| {
            if let NodeType::Leaf { data, .. } = &node.node_type {
                &data[index]
            } else {
                unreachable!("Wait, search returned an Internal node instead of a Leaf!")
            }
        });

        println!("🎯 Found it! Key: {} -> Value: \"{}\"", target_key, *value_ref);
    } else {
        println!("❌ Key {} not found.", target_key);
    }

    if tree.search(999).is_none() {
        println!("👻 Key 999 correctly identified as missing.");
    }
    tree.print_recursive();
}
