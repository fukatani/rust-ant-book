use std::cmp::*;

fn main() {
    let n = 5;
    let a = vec![4, 2, 3, 1 ,5];
    let mut dp = vec![std::i64::MAX; n];
    for i in 0..n {
        let pointer = dp.lower_bound(&a[i]);
        dp[pointer] = a[i];
    }
    println!("{:?}", dp.lower_bound(&std::i64::MAX));
}
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
