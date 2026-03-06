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
    pub fn print_recursive(&self) {
        match &self.root {
            Some(node) => Self::print_recursively(&Rc::clone(node), 0),
            None => println!("No records."),
        }
    }
    fn print_recursively(node: &NodeRef<T>, level: usize){
        print!("{} Level {level}", " ".repeat(level * 4));
        println!("{:?}", node.borrow().keys);
        match &node.borrow().node_type {
            NodeType::Internal{children} => {
                for child in children {
                    Self::print_recursively(&Rc::clone(child), level + 1);
                }
            }
            _ => {}
        }
    }
    fn borrow_or_merge(node: &NodeRef<T>, index: usize){
        let t = node.borrow().t;
        let has_right: Option<NodeRef<T>> = None;
        let has_left: Option<NodeRef<T>> = None;
        let child: NodeRef<T>;
        if let NodeType::Internal{children} = &node.borrow().node_type {
            child = Rc::clone(&children[index]);
            if index > 0{
                has_left = Some(Rc::clone(&children[index - 1]);
            }
            if index < children.len() - 1{
                has_right = Some(Rc::clone(&children[index + 1]));
            }
        }
        if let Some(left) = has_left && left.borrow().keys.len() > (t - 1) as usize{
            Self::borrow_from_left(Rc::clone(node), child, left, index);
        } else if let Some(right) = has_right & right.borrow().keys.len() > (t - 1) as usize{
            Self::borrow_from_right(Rc::clone(node), child, right, index);
        } else if let Some(left) {
            Self::merge_from_right(Rc::clone(node), left, child, index - 1);
        } else if let Some(right) {
            Self::merge_from_right(Rc::clone(node), child, right, index);
        }
    }
    fn borrow_from_left(padre: NodeRef<T>, hijo: NodeRef<T>, izq: NodeRef<T>, index: usize){
        let hijo_ref = &mut *hijo.borrow_mut();
        match &mut hijo_ref.node_type {
            NodeType::Internal{children} => {
                hijo_ref.keys.insert(0, padre.borrow().keys[index - 1]);
                padre.borrow_mut().keys[index - 1] = izq.borrow_mut().keys.pop().expect("unreachable");
                if let NodeType::Internal{children: children_izq} = &mut izq.borrow_mut().node_type {
                    children.insert(0, children_izq.pop().expect("unreachable"));
                }
            }
            NodeType::Leaf{data, ..} => {
                hijo_ref.keys.insert(0, izq.borrow_mut().keys.pop().expect("unreachable"));
                if let NodeType::Leaf{data: data_izq, ..} = &mut izq.borrow_mut().node_type {
                    data.insert(0, data_izq.pop().expect("unreachable"));
                }
                padre.borrow_mut().keys[index - 1] = hijo_ref.keys[0];
            }
        }
    }

    fn borrow_from_right(padre:NodeRef<T>, hijo:NodeRef<T>, der:NodeRef<T>, index: usize){
        let hijo_ref = &mut *hijo.borrow_mut();
        match &mut hijo_ref.node_type {
            NodeType::Internal{children} => {
                hijo_ref.keys.push(padre.borrow().keys[index]);
                if let NodeType::Internal {children: children_der} = &mut der.borrow_mut().node_type {
                    children.push(children_der.remove(0));
                }
                padre.borrow_mut().keys[index] = der.borrow_mut().keys.remove(0);
            }
            NodeType::Leaf{data, ..} => {
                hijo_ref.keys.push(der.borrow_mut().keys.remove(0));
                if let NodeType::Leaf { data: data_der, .. } = &mut der.borrow_mut().node_type {
                    data.push(data_der.remove(0));
                }
                padre.borrow_mut().keys[index] = der.borrow().keys[0];
            }
        }
    }

    fn merge_from_right(padre: NodeRef<T>, hijo: NodeRef<T>, der: NodeRef<T>, index: usize) {
        let hijo_ref = &mut *hijo.borrow_mut();
        match &mut hijo_ref.node_type {
            NodeType::Internal { children } => {
                hijo_ref.keys.push(padre.borrow_mut().keys.remove(index));
                hijo_ref.keys.append(&mut der.borrow_mut().keys);
                if let NodeType::Internal {children: children_der} = &mut der.borrow_mut().node_type {
                    children.append(children_der);
                }
            }
            NodeType::Leaf { data, next_leaf } => {
                hijo_ref.keys.append(&mut der.borrow_mut().keys);
                if let NodeType::Leaf { data: data_der, next_leaf: next_leaf_der } = &mut der.borrow_mut().node_type {
                    data.append(data_der);
                    *next_leaf = next_leaf_der.take();
                }
                padre.borrow_mut().keys.remove(index);
            }
        }
        if let NodeType::Internal {children} = &mut padre.borrow_mut().node_type {
            drop(children.remove(index + 1));
        }
        drop(der);
    }
}



