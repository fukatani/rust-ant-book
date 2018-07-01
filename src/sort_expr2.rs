use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Debug)]
struct MinInt {
    value:i32,
}

impl Eq for MinInt {}

impl PartialOrd for MinInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl Ord for MinInt {
    fn cmp(&self, other: &MinInt) -> Ordering {
        other.value.cmp(&self.value)
    }
}

fn make_pair(x: i32, y:i32) -> (MinInt, MinInt) {
    (MinInt{value: x}, MinInt{value: y})
}

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(make_pair(2, 5));
    heap.push(make_pair(0, 2));
    heap.push(make_pair(0, 1));
    heap.push(make_pair(3, 5));

    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
}
