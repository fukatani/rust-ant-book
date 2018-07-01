use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Debug)]
struct MinInt(i32);

impl Eq for MinInt {}

impl PartialOrd for MinInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinInt {
    fn cmp(&self, other: &MinInt) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

fn make_pair(x: i32, y:i32) -> (MinInt, MinInt) {
    (MinInt(x), MinInt(y))
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
