#[derive(Clone, Debug)]
struct PriorityQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone + std::marker::Copy,
{
    heap: Vec<T>,
}

impl<T> PriorityQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone + std::marker::Copy,
{
    fn new() -> PriorityQueue<T> {
        PriorityQueue {
            heap: Vec::<T>::new(),
        }
    }

    fn push(&mut self, x: T) {
        let mut i = self.heap.len() as i64;
        self.heap.push(x);
        while i > 0 {
            let p = (i as usize - 1) / 2;
            if self.heap[p] <= x {
                break;
            }
            let temp = self.heap[i as usize];
            self.heap[i as usize] = self.heap[p];
            self.heap[p] = temp;
            i = p as i64;
        }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let ret = self.heap[0];
        let x = self.heap[self.heap.len() - 1];
        let mut i = 0;
        let new_len = self.heap.len() - 1;
        while i * 2 + 1 < new_len {
            let mut a = i * 2 + 1;
            let b = i * 2 + 2;
            if b < new_len && self.heap[b] < self.heap[a] {
                a = b;
            }
            if self.heap[a] >= x {
                break;
            }
            self.heap[i] = self.heap[a];
            i = a;
        }
        self.heap[i] = x;
        self.heap.swap_remove(new_len);
        Some(ret)
    }

    fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.heap[0])
    }
}

impl<T> std::iter::FromIterator<T> for PriorityQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone + std::marker::Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = PriorityQueue::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

fn main() {
    let mut v = vec![4, 3, 0, 7, 2];
    let mut h = v.into_iter().collect::<PriorityQueue<_>>();
    println!("{:?}", h);
    assert_eq!(h.peek(), Some(0));
    assert_eq!(h.pop(), Some(0));
    println!("{:?}", h);
    assert_eq!(h.peek(), Some(2));
    assert_eq!(h.pop(), Some(2));
    println!("{:?}", h);
    assert_eq!(h.peek(), Some(3));
    assert_eq!(h.pop(), Some(3));
    h.push(1);
    assert_eq!(h.peek(), Some(1));
    assert_eq!(h.pop(), Some(1));
}
