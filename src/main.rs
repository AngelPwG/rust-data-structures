mod bptree;
use bptree::BPlusTree;

fn check(label: &str, passed: bool) {
    if passed {
        println!("  ✔  {}", label);
    } else {
        println!("  ✘  {}", label);
    }
}

// ── Insert & Search ──────────────────────────────────────────────────────────
fn test_insert_search() {
    println!("\n[1] Insert & Search");
    let mut tree: BPlusTree<String> = BPlusTree::new();

    tree.insert(10, "ten".to_string());
    tree.insert(20, "twenty".to_string());
    tree.insert(5,  "five".to_string());
    check("search existing key 10",  tree.search(10).is_some());
    check("search existing key 5",   tree.search(5).is_some());
    check("search existing key 20",  tree.search(20).is_some());

    check("search missing key 99",   tree.search(99).is_none());

    let dup = tree.insert(10, "ten-dup".to_string());
    check("duplicate insert returns false", !dup);
    check("original value preserved after dup insert", {
        if let Some((node, i)) = tree.search(10) {
            if let bptree::NodeType::Leaf { data, .. } = &node.borrow().node_type {
                data[i] == "ten"
            } else { false }
        } else { false }
    });
}

// ── Split stress (forces multiple splits) ───────────────────────────────────
fn test_splits() {
    println!("\n[2] Splits");
    let mut tree: BPlusTree<u64> = BPlusTree::new();

    for i in 1..=200 {
        tree.insert(i, i);
    }

    let mut all_found = true;
    for i in 1..=200 {
        if tree.search(i).is_none() { all_found = false; break; }
    }
    check("all 200 keys findable after repeated splits", all_found);
    check("key 0 not found (never inserted)", tree.search(0).is_none());
    check("key 201 not found (never inserted)", tree.search(201).is_none());
}

// ── Delete ───────────────────────────────────────────────────────────────────
fn test_delete() {
    println!("\n[3] Delete");
    let mut tree: BPlusTree<u64> = BPlusTree::new();
    for i in 1..=50 { tree.insert(i, i); }

    tree.delete(25);
    check("deleted key 25 is gone",       tree.search(25).is_none());
    check("neighbour key 24 still there", tree.search(24).is_some());
    check("neighbour key 26 still there", tree.search(26).is_some());

    tree.print_recursive();
    for i in 10..=20 { tree.delete(i); }
    let mut range_gone = true;
    for i in 10..=20 { if tree.search(i).is_some() { range_gone = false; break; } }
    check("range 10-20 fully deleted", range_gone);
    tree.print_recursive();
    let mut rest_intact = true;
    for i in (1..=9).chain(21..=50) {
        if i == 25 { continue; }
        if tree.search(i).is_none() { rest_intact = false; break; }
    }
    check("all other keys intact after range delete", rest_intact);

    tree.delete(999);
    check("deleting missing key is a no-op", tree.search(1).is_some());

    for i in (1..=50).filter(|x| *x != 25 && !((10..=20).contains(x))) {
        tree.delete(i);
    }
    check("tree empty after deleting all keys", tree.search(1).is_none());
}

// ── Update ───────────────────────────────────────────────────────────────────
fn test_update() {
    println!("\n[4] Update");
    let mut tree: BPlusTree<String> = BPlusTree::new();
    for i in 1..=30 { tree.insert(i, format!("v{}", i)); }

    tree.update(15, "updated-15".to_string());
    check("updated value is reflected", {
        if let Some((node, i)) = tree.search(15) {
            if let bptree::NodeType::Leaf { data, .. } = &node.borrow().node_type {
                data[i] == "updated-15"
            } else { false }
        } else { false }
    });

    tree.update(999, "ghost".to_string());
    check("updating missing key is a no-op", tree.search(999).is_none());

    check("key 14 value unchanged", {
        if let Some((node, i)) = tree.search(14) {
            if let bptree::NodeType::Leaf { data, .. } = &node.borrow().node_type {
                data[i] == "v14"
            } else { false }
        } else { false }
    });
}

// ── Boundary & edge cases ────────────────────────────────────────────────────
fn test_edge_cases() {
    println!("\n[5] Edge Cases");
    let mut tree: BPlusTree<u64> = BPlusTree::new();

    check("search on empty tree",  tree.search(1).is_none());
    tree.delete(1);
    check("delete on empty tree does not panic", true);

    tree.insert(42, 42);
    check("single insert then find",  tree.search(42).is_some());
    tree.delete(42);
    check("single delete then gone",  tree.search(42).is_none());
    check("tree empty after sole deletion", tree.search(42).is_none());

    let mut tree2: BPlusTree<u64> = BPlusTree::new();
    for i in (1..=100).rev() { tree2.insert(i, i); }
    let mut all_found = true;
    for i in 1..=100 { if tree2.search(i).is_none() { all_found = false; break; } }
    check("100 descending inserts all searchable", all_found);

    let mut tree3: BPlusTree<u64> = BPlusTree::new();
    for i in 1..=20 { tree3.insert(i, i); }
    for i in (1..=20).step_by(2) { tree3.delete(i); }   // delete odds
    let evens_ok  = (2..=20).step_by(2).all(|i| tree3.search(i).is_some());
    let odds_gone = (1..=20).step_by(2).all(|i| tree3.search(i).is_none());
    check("even keys survive alternating delete", evens_ok);
    check("odd keys removed in alternating delete", odds_gone);
}

// ────────────────────────────────────────────────────────────────────────────
fn main() {
    println!("════════════════════════════════════");
    println!("        B+ Tree Test Suite          ");
    println!("════════════════════════════════════");

    test_insert_search();
    test_splits();
    test_delete();
    test_update();
    test_edge_cases();

    println!("\n════════════════════════════════════");
    println!("           Tests complete            ");
    println!("════════════════════════════════════\n");
} 
