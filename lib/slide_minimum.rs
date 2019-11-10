#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;

fn slide_minimum(a: &Vec<i64>, k: usize) -> Vec<i64> {
    assert!(k <= a.len());
    let mut s = 0;
    let mut t = 0;
    let mut deq = vec![0; a.len()];
    let mut ans = vec![0; a.len() - k + 1];
    for i in 0..a.len() {
        while s < t && a[deq[t - 1]] >= a[i] {
            t -= 1;
        }
        deq[t] = i;
        t += 1;
        // println!("{:?}", &deq[s..t]);

        if i + 1 >= k {
            ans[i + 1 - k] = a[deq[s]];
            if deq[s] == i + 1 - k {
                s += 1;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(slide_minimum(&vec![1, 3, 5, 4, 2], 3), vec![1, 3, 2]);
}
