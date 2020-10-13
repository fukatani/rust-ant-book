struct Permutation {
    indexes: Vec<usize>,
}

impl Permutation {
    fn new(n: usize) -> Permutation {
        Permutation {
            indexes: (0..n).collect::<Vec<_>>(),
        }
    }

    fn next_permutation(&mut self) -> bool {
        if self.indexes.len() < 2 {
            return false;
        }

        let mut i = self.indexes.len() - 1;
        while i > 0 && self.indexes[i - 1] >= self.indexes[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        let mut j = self.indexes.len() - 1;
        while j >= i && self.indexes[j] <= self.indexes[i - 1] {
            j -= 1;
        }

        self.indexes.swap(j, i - 1);
        self.indexes[i..].reverse();
        true
    }
}

fn main() {
    let mut p = Permutation::new(3);
    assert_eq!(p.indexes, vec![0, 1, 2]);
    p.next_permutation();
    assert_eq!(p.indexes, vec![0, 2, 1]);
    p.next_permutation();
    assert_eq!(p.indexes, vec![1, 0, 2]);
    p.next_permutation();
    assert_eq!(p.indexes, vec![1, 2, 0]);
    p.next_permutation();
    assert_eq!(p.indexes, vec![2, 0, 1]);
    assert_eq!(p.next_permutation(), true);
    assert_eq!(p.indexes, vec![2, 1, 0]);
    assert_eq!(p.next_permutation(), false);
}
