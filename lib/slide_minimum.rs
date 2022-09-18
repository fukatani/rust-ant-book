#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;

fn slide_minimum(a: &Vec<i64>, width: usize) -> Vec<i64> {
    assert!(width <= a.len());

    let mut deq = VecDeque::new();
    let mut ans = vec![0; a.len() - width + 1];
    for i in 0..a.len() {
        while !deq.is_empty() && a[deq[deq.len() - 1]] >= a[i] {
            deq.pop_back();
        }
        deq.push_back(i);

        if i + 1 >= width {
            ans[i + 1 - width] = a[deq[0]];
            if deq[0] == i + 1 - width {
                deq.pop_front();
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(slide_minimum(&vec![1, 3, 5, 4, 2], 3), vec![1, 3, 2]);
}
