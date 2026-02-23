use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct BPLusTree<T: Display + PartialOrd>{
    root: Option<NodeRef<T>>
}
struct Node<T: Display + PartialOrd>{
    keys: Vec<u64>,
    node_type: NodeType<T>
}
enum NodeType<T: Display + PartialOrd>{
    Internal{children: Vec<NodeRef<T>>, t: u8},
    Leaf{data: Vec<T>, t: u8, next_leaf: Option<NodeRef<T>>}
}

impl<T: Display + PartialOrd> Node<T>{
    pub fn new_node(t: u8, is_leaf: bool) -> NodeRef<T>{
        let node_type = if is_leaf{
            NodeType::Leaf {t, data: Vec::new(), next_leaf: None}
        } else {
            NodeType::Internal {t, children: Vec::new()}
        };
        Rc::new(RefCell::new(Node{
            keys: Vec::new(),
            node_type
        }))
    }
    pub fn is_full(&self) -> bool {
        match &self.node_type {
            NodeType::Internal{t, ..} => {
                self.keys.len() >= (t * 2 - 1) as usize
            },
            NodeType::Leaf{t, ..} => {
                self.keys.len() >= (t * 2 - 1) as usize
            }
        }
    }
    pub fn split_child(&mut self, children_index: usize){
        if let NodeType::Internal {children, ..} = &mut self.node_type {
            let second_node;
            {
                let mut full_node = children[children_index].borrow_mut();
                let full_node = &mut *full_node;
                match &mut full_node.node_type {
                    NodeType::Leaf { data, t, next_leaf } => {
                        second_node = Self::new_node(*t, true);
                        let mut second_mut = second_node.borrow_mut();
                        let split_index = *t - 1;
                        second_mut.keys = full_node.keys.split_off((split_index) as usize);
                        if let NodeType::Leaf { data: data2, next_leaf: next_leaf2, .. } = &mut second_mut.node_type {
                            *data2 = data.split_off((*t - 1) as usize);
                            *next_leaf2 = next_leaf.take();
                            self.keys.insert(children_index, second_mut.keys[0].clone());
                            *next_leaf = Some(Rc::clone(&second_node));
                        }
                    },

                    NodeType::Internal { children, t } => {
                        second_node = Self::new_node(*t, false);
                        let mut second_mut = second_node.borrow_mut();
                        second_mut.keys = full_node.keys.split_off((*t - 1) as usize);
                        if let NodeType::Internal { children: children2, .. } = &mut second_mut.node_type {
                            *children2 = children.split_off((*t) as usize);
                            self.keys.insert(children_index, second_mut.keys.remove(0));
                        }
                    }
                }
            }
            children.insert(children_index + 1, second_node);
        }
    }
}


