use std::fmt::Display;

struct QueueNode<T: Display> {
    val: T,
    next_node: Option<Box<QueueNode<T>>>,
}

struct Queue<T: Display> {
    start_node: Option<Box<QueueNode<T>>>,
}

impl<T: Display> Queue<T> {
    fn enqueue(&mut self, value: T) -> () {
        let new_node = Box::new(QueueNode{val: value, next_node: None});

        let mut current = &mut self.start_node;

        while let Some(node) = current {
            current = &mut node.next_node;
        }

        *current = Some(new_node);
    }
    fn dequeue(&mut self) -> Option<T>{
        if let Some(i) = self.start_node.take() {
            self.start_node = i.next_node;
            return Some(i.val)
        }

        println!("No elements to dequeue.");
        None
    }
    fn show_queue(&self) {
        let mut current = &self.start_node;
        while let Some(i) = current {
            print!("{}", i.val);
            current = &i.next_node;
        }
    }
}