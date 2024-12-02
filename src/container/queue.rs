use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

//define Deque
pub struct Deque<T> {
    buffer: Vec<Option<T>>,
    front: usize,
    end: usize,
    size: usize,
}

impl<T> Deque<T> {
    pub fn new(n: usize) -> Deque<T> {
        let mut b = Vec::new();
        for _ in 0..n {
            b.push(None);
        }
        Deque {
            buffer: b,
            front: 0,
            end: 0,
            size: 0,
        }
    }
    pub fn push_back(&mut self, x: T) -> bool {
        if self.buffer.len() >= self.size {
            return false;
        }
        self.buffer[self.end] = Some(x);
        self.size += 1;
        self.end = (self.end + 1) % self.buffer.len();
        return true;
    }
    pub fn push_front(&mut self, x: T) -> bool {
        if self.buffer.len() >= self.size {
            return false;
        }
        self.front = (self.front - 1) % self.buffer.len();
        self.buffer[self.front] = Some(x);
        self.size += 1;
        return true;
    }
    pub fn pop_back(&mut self) -> Option<&T> {
        todo!()
    }
    pub fn pop_front(&mut self) -> Option<&T> {
        todo!()
    }
    pub fn front(&self) -> Option<&T> {
        todo!()
    }
    pub fn back(&self) -> Option<&T> {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
    pub fn clear(&mut self) {
        todo!()
    }
}

//Deque for linked list
struct Node<T> {
    item: T,
    prev: RefCell<Link<T>>,
    next: RefCell<Link<T>>,
}
type Link<T> = Option<Rc<Node<T>>>;

struct LinkedDeq<T> {
    front: Link<T>,
    end: Link<T>,
    size: usize,
}

impl<T> LinkedDeq<T> {
    fn new() -> Self {
        LinkedDeq {
            front: None,
            end: None,
            size: 0,
        }
    }
    fn push_back(&mut self, x: T) {
        let new_node = Rc::new(Node {
            item: x,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        });
        if self.size == 0 {
            self.front = Some(new_node.clone());
        } else {
            let node = self.end.take().unwrap();
            let mut ptr1 = node.next.borrow_mut();
            *ptr1 = Some(new_node.clone());
            let mut ptr2 = new_node.prev.borrow_mut();
            *ptr2 = Some(node.clone());
        }
        self.end = Some(new_node);
        self.size += 1;
    }

    fn push_front(&mut self, x: T) {
        let new_node = Rc::new(Node {
            item: x,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        });
        if self.size == 0 {
            self.end = Some(new_node.clone());
        } else {
            let node = self.front.take().unwrap();
            let mut pt1 = node.prev.borrow_mut();
            *pt1 = Some(new_node.clone());
            let mut pt2 = new_node.next.borrow_mut();
            *pt2 = Some(node.clone());
        }
        self.end = Some(new_node);
        self.size += 1;
    }
}
