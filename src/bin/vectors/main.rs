use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(0);
    println!("{:?}", stack);
    println!("{:?}", stack.pop());

    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(0);
    queue.push_front(1);
    println!("{:?}", queue);
    println!("{:?}", queue.pop_front());
    println!("{:?}", queue.pop_back());

    let mut prior_que = BinaryHeap::new();
    prior_que.push(1);
    prior_que.push(2);
    prior_que.push(0);
    println!("{:?}", prior_que);
    println!("{:?}", prior_que.pop());

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(0);
    println!("{:?}", set.contains(&1));
    if set.contains(&2) {
        println!("{:?}", set.get(&2));
        set.remove(&2);
    }

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    println!("{:?}", map.get("one"));
    map.remove("one");
    println!("{:?}", map);
}
