pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }
        self.swap(j, i-1);
        self[i..].reverse();
        true
    }
}

fn main() {
    let mut data = vec![0, 1, 2 ,3];
    while data.next_permutation() {
        println!("{:?}", data);
    }
}
