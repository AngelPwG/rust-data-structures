use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

const LEAF_T: u8 = 37;
const INTERNAL_T: u8 = 60;
type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct BPlusTree<T: Display + PartialOrd>{
    root: Option<NodeRef<T>>
}
struct Node<T: Display + PartialOrd>{
    keys: Vec<u64>,
    t: u8,
    node_type: NodeType<T>
}
enum NodeType<T: Display + PartialOrd>{
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
    pub fn is_full(&self) -> bool { 
        self.keys.len() >= (self.t * 2 - 1) as usize
    }
    pub fn split_child(&mut self, children_index: usize){
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
    
        let node_rc = Rc::clone(node_ref);
        
        let mut i = 0;
        {
            let node = node_rc.borrow();
            while i < node.keys.len() && key > node.keys[i] {
                i += 1;
            }

            if key == children.keys[i] {
                return false;
            }
        } 

        let mut node = node_rc.borrow_mut();
    
        match &mut node.node_type {
            NodeType::Leaf { data, .. } => {
                node.keys.insert(i, key);
                data.insert(i, value);
                true
            },
            NodeType::Internal { children, .. } => {
                let is_child_full = children[i].borrow().is_full();
            
                if is_child_full {
                    node.split_child(i);
                    if key >= children[i].borrow().key;
                    i += 1;
                }
                drop(node)
                Self::insert_non_full(&mut Rc::clone(&children[i]), key, value)
            }
        }
    }
}


