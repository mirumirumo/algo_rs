use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
struct Node<T> {
    element: T,
    prev: Option<Arc<Mutex<Node<T>>>>,
    next: Option<Arc<Mutex<Node<T>>>>,
}

#[derive(Debug)]
pub struct List<T: Debug> {
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
    size: usize,
}

impl<T: Debug> Default for List<T> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: Default::default(),
            size: Default::default(),
        }
    }
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn push(&mut self, element: T) {
        let new_node = Arc::new(Mutex::new(Node {
            element,
            prev: None,
            next: self.head.take(),
        }));
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
            }
            Some(old_head) => {
                old_head.lock().unwrap().prev = Some(new_node.clone());
            }
        }
        self.head = Some(new_node.clone());
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 1 {
            self.tail = None;
        }
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.lock().unwrap().next.clone();
                if let Some(ref head) = self.head {
                    head.lock().unwrap().prev = None;
                }
                self.size -= 1;
                match Arc::try_unwrap(old_head) {
                    Ok(node) => Some(node.into_inner().unwrap().element),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn add(&mut self, element: T) {
        let new_node = Arc::new(Mutex::new(Node {
            element,
            prev: self.tail.take(),
            next: None,
        }));
        match new_node.lock().unwrap().prev.clone() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            Some(old_tail) => {
                old_tail.lock().unwrap().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        self.size += 1;
    }
}
