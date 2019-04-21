use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
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

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    let vec = vec![1, 2, 2, 4, 6, 7, 12, 54, 60];
    println!("{}", vec.lower_bound(&0));
    println!("{}", vec.lower_bound(&1));
    println!("{}", vec.lower_bound(&2));
    println!("{}", vec.lower_bound(&3));
    let vec = vec![1];
    println!("{}", vec.lower_bound(&0));
    println!("{}", vec.lower_bound(&1));
    println!("{}", vec.lower_bound(&2));
}
