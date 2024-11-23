use std::ptr;

// ノードの定義
struct Node<T> {
    value: T,
    prev: *mut Node<T>, // 前のノードへの生ポインタ
    next: *mut Node<T>, // 次のノードへの生ポインタ
}

// 双方向連結リストの定義
pub struct DoublyLinkedList<T> {
    head: *mut Node<T>, // リストの先頭
    tail: *mut Node<T>, // リストの末尾
    len: usize,         // リストの長さ
}

impl<T> DoublyLinkedList<T> {
    // 新しい空のリストを作成
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    // リストの先頭に要素を追加
    pub fn push_front(&mut self, value: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                value,
                prev: ptr::null_mut(),
                next: self.head,
            }));

            if !self.head.is_null() {
                (*self.head).prev = new_node;
            } else {
                // リストが空の場合、末尾も新しいノードを指す
                self.tail = new_node;
            }

            self.head = new_node;
            self.len += 1;
        }
    }

    // リストの末尾に要素を追加
    pub fn push_back(&mut self, value: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                value,
                prev: self.tail,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_node;
            } else {
                // リストが空の場合、先頭も新しいノードを指す
                self.head = new_node;
            }

            self.tail = new_node;
            self.len += 1;
        }
    }

    // リストの先頭から要素を削除
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None;
            }

            let old_head = self.head;
            self.head = (*old_head).next;

            if !self.head.is_null() {
                (*self.head).prev = ptr::null_mut();
            } else {
                // リストが空になった場合、末尾もnullにする
                self.tail = ptr::null_mut();
            }

            self.len -= 1;

            Some(Box::from_raw(old_head).value)
        }
    }

    // リストの末尾から要素を削除
    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                return None;
            }

            let old_tail = self.tail;
            self.tail = (*old_tail).prev;

            if !self.tail.is_null() {
                (*self.tail).next = ptr::null_mut();
            } else {
                // リストが空になった場合、先頭もnullにする
                self.head = ptr::null_mut();
            }

            self.len -= 1;

            Some(Box::from_raw(old_tail).value)
        }
    }

    // リストの内容を先頭から表示
    pub fn display_from_head(&self) {
        unsafe {
            let mut current = self.head;
            while !current.is_null() {
                // print!("{:?} -> ", (*current).value);
                current = (*current).next;
            }
            println!("None");
        }
    }

    // リストの内容を末尾から表示
    pub fn display_from_tail(&self) {
        unsafe {
            let mut current = self.tail;
            while !current.is_null() {
                // print!("{:?} -> ", (*current).value);
                current = (*current).prev;
            }
            println!("None");
        }
    }

    // リストの長さを取得
    pub fn len(&self) -> usize {
        self.len
    }

    // リストが空かどうかを確認
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// 使用例
fn main() {
    let mut list = DoublyLinkedList::new();

    // 要素を追加
    list.push_front(1);
    list.push_front(2);
    list.push_back(0);
    list.push_back(-1);

    // リストを表示
    println!("From head:");
    list.display_from_head();

    println!("From tail:");
    list.display_from_tail();

    // 要素を削除
    println!("Popped from front: {:?}", list.pop_front());
    println!("Popped from back: {:?}", list.pop_back());

    // 再度リストを表示
    println!("After popping:");
    list.display_from_head();
}
