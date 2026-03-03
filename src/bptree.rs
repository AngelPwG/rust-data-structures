use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

const LEAF_T: u8 = 3;
const INTERNAL_T: u8 = 4;
type NodeRef<T> = Rc<RefCell<Node<T>>>;

pub struct BPlusTree<T: Display + PartialOrd>{
    root: Option<NodeRef<T>>
}
pub struct Node<T: Display + PartialOrd>{
    keys: Vec<u64>,
    t: u8,
    pub node_type: NodeType<T>,
}
pub enum NodeType<T: Display + PartialOrd>{
    Internal{children: Vec<NodeRef<T>>},
    Leaf{data: Vec<T>, next_leaf: Option<NodeRef<T>>}
}

impl<T: Display + PartialOrd> Node<T>{
    pub fn new_node(t: u8, is_leaf: bool) -> NodeRef<T>{
        let node_type = if is_leaf{
            NodeType::Leaf {data: Vec::new(), next_leaf: None}
        } else {
            NodeType::Internal {children: Vec::new()}
        };
        Rc::new(RefCell::new(Node{
            keys: Vec::new(),
            t, 
            node_type
        }))
    }
    fn is_full(&self) -> bool { 
        self.keys.len() >= (self.t * 2 - 1) as usize
    }
    fn split_child(&mut self, children_index: usize){
        if let NodeType::Internal {children, ..} = &mut self.node_type {
            let second_node;
            {
                let mut full_node = children[children_index].borrow_mut();
                let full_node = &mut *full_node;
                match &mut full_node.node_type {
                   
                    NodeType::Leaf { data, next_leaf } => {
                        second_node = Self::new_node(full_node.t, true);
                        let mut second_mut = second_node.borrow_mut();
                        let split_index = full_node.t - 1;
                        second_mut.keys = full_node.keys.split_off((split_index) as usize);
                        if let NodeType::Leaf { data: data2, next_leaf: next_leaf2, .. } = &mut second_mut.node_type {
                            *data2 = data.split_off((full_node.t - 1) as usize);
                            *next_leaf2 = next_leaf.take();
                            self.keys.insert(children_index, second_mut.keys[0].clone());
                            *next_leaf = Some(Rc::clone(&second_node));
                        }
                    },

                    NodeType::Internal { children } => {
                        second_node = Self::new_node(full_node.t, false);
                        let mut second_mut = second_node.borrow_mut();
                        second_mut.keys = full_node.keys.split_off((full_node.t - 1) as usize);
                        if let NodeType::Internal { children: children2, .. } = &mut second_mut.node_type {
                            *children2 = children.split_off((full_node.t) as usize);
                            self.keys.insert(children_index, second_mut.keys.remove(0));
                        }
                    }
                }
            }
            children.insert(children_index + 1, second_node);
        }
    }
}
impl<T: Display + PartialOrd> BPlusTree<T>{
    pub fn new() -> Self{
        BPlusTree { root: None}
    }
    pub fn insert(&mut self, key: u64, value: T) -> bool{
        match &mut self.root{
            None => {
                self.root = Some(Node::new_node(LEAF_T, true));
                if let Some(node) = &mut self.root {
                    let mut node = node.borrow_mut();
                    node.keys.push(key);
                    if let NodeType::Leaf{data, ..} = &mut node.node_type {
                        data.push(value)
                    }
                    return true;
                }
                true
            },
            Some(root) => {
                if root.borrow().is_full() {
                    let mut new_root = Node::<T>::new_node(INTERNAL_T, false);
                    if let NodeType::Internal {children} = &mut new_root.borrow_mut().node_type {
                        children.insert(0, Rc::clone(&root));
                    }
                    new_root.borrow_mut().split_child(0);
                    self.root = Some(Rc::clone(&new_root));
                    return Self::insert_non_full(&mut new_root, key, value)
                }
                Self::insert_non_full(root, key, value)
            }
        }
    }
    fn insert_non_full(node: &mut NodeRef<T>, key: u64, value: T) -> bool{
    
        let node_rc: NodeRef<T> = Rc::clone(node);
        
        let mut i = 0;
        {
            let node = node_rc.borrow();
            while i < node.keys.len() && key > node.keys[i] {
                i += 1;
            }

            if i < node.keys.len() && key == node.keys[i] {
                return false;
            }
        }

        let mut node_lock = node_rc.borrow_mut();
        let node = &mut *node_lock;    

        let mut is_children_full = false;
        if let NodeType::Internal{children} = &mut node.node_type {
            is_children_full = children[i].borrow().is_full();
        }

        if is_children_full{
            node.split_child(i);
            if key > node.keys[i]{
                i += 1;
            }
        }
        
        match &mut node.node_type {
            NodeType::Leaf { data, .. } => {
                node.keys.insert(i, key);
                data.insert(i, value);
                true
            },
            NodeType::Internal { children } => {
                let mut next_child = Rc::clone(&children[i]);
                drop(node_lock);
                Self::insert_non_full(&mut next_child, key, value)
            }
        }
    }
    pub fn search(&self, key: u64) -> Option<(NodeRef<T>, usize)>{
        match &self.root{
            None => None,
            Some(node) => Self::search_in(Rc::clone(node), key),
        }
    }
    fn search_in(node: NodeRef<T>, key: u64) -> Option<(NodeRef<T>, usize)>{
        let mut i = 0;
        
        let lock_n = node.borrow();
        match &lock_n.node_type{
            NodeType::Leaf {data, ..} => {
                while i < lock_n.keys.len() && key > lock_n.keys[i]{
                    i += 1;
                }
                if i < lock_n.keys.len() && lock_n.keys[i] == key{
                    drop(lock_n);
                    return Some((Rc::clone(&node), i));
                }
                None
            }
            NodeType::Internal {children} => {
                while i < lock_n.keys.len() && key >= lock_n.keys[i]{
                    i += 1;
                }   
                let next_child = Rc::clone(&children[i]);
                drop(lock_n);
                Self::search_in(next_child, key)
            }
        }
    }
    pub fn printRecursive(&self) {
        match &self.root {
            Some(node) => Self::printRecursively(&Rc::clone(node), 0),
            None => println!("No records."),
        }
    }
    fn printRecursively(node: &NodeRef<T>, level: usize){
        print!("{} Level {level}", " ".repeat(level));
        println!("{:?}", node.borrow().keys);
        match &node.borrow().node_type {
            NodeType::Internal{children} => {
                for child in children {
                    Self::printRecursively(&Rc::clone(child), level + 1);
                }
            }
            _ => {}
        }
    }
    fn borrowOrMerge(node: &NodeRef<T>, index: usize){
        if let NodeType::Internal{children} = &mut node.borrow_mut().node_type {
            let mut hasLeft = true;
            let mut hasRight = true;
            if index == children.len() - 1 {
                hasRight = false;
            }
            if index == 0{
                hasLeft = false;
            }
            if hasLeft && (children[index - 1].borrow().keys.len() > (node.borrow().t - 1) as usize) {
                
            } else if hasRight && (children[index + 1].borrow().keys.len() > (node.borrow().t - 1) as usize) {

            } else if hasLeft {

            } else {

            }
        }
    }
    fn borrowFromLeft(padre: &mut NodeRef<T>, hijo: &mut NodeRef<T>, izq: &mut NodeRef<T>, index: usize){
        hijo.borrow_mut().keys.add(0, izq.borrow_mut().keys.remove(0));
        match &mut hijo.borrow_mut()

    }
}


