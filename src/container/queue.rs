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
