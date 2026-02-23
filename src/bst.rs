use std::fmt::Display;

struct TreeNode<T: Display + PartialOrd>{
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct Tree<T: Display + PartialOrd>{
    root: Option<Box<TreeNode<T>>>
}

impl<T: Display + PartialOrd> Tree<T>{
    pub fn insert(&mut self, value: T){
        Self::inner_insert(&mut self.root, value);
    }
    fn inner_insert(node: &mut Option<Box<TreeNode<T>>>, value: T){
        match node {
            None => *node = Some(Box::<TreeNode<T>>::new(TreeNode::<T>{ val: value, left: None, right: None})),
            Some(n) => {
                if n.val > value {
                    Self::inner_insert(&mut n.left, value);
                } else if n.val < value {
                    Self::inner_insert(&mut n.right, value);
                } else {
                    println!("Duplicated entry: {}", value)
                }
            }
        }
    }

    pub fn print_in_order(&self) {
        Self::inner_print(&self.root);
        println!();
    }

    fn inner_print(node: &Option<Box<TreeNode<T>>>) { match node {
            None => return,
            Some(n) => {
                Self::inner_print(&n.left);
                println!("{}", n.val);
                Self::inner_print(&n.right);
            }
        }
    }
}