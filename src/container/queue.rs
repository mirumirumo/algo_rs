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
        self.end += 1;
        if self.end == self.buffer.len() {
            self.end = 0;
        }
        return true;
    }
}
