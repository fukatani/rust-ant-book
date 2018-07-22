#[derive(Debug, Clone)]
struct UnionFindTree {
    parent_or_size: Vec<isize>,
}

impl UnionFindTree {
    fn new(size: usize) -> UnionFindTree {
        UnionFindTree { parent_or_size: vec![-1; size] }
    }

    fn find(&self, index: usize) -> usize {
        let mut index = index;
        while self.parent_or_size[index] >= 0 {
            index = self.parent_or_size[index] as usize;
        }
        index
    }

    fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, index0: usize, index1: usize) -> bool {
        let a = self.find(index0);
        let b = self.find(index1);
        if a == b {
            false
        } else {
            if self.parent_or_size[a] < self.parent_or_size[b] {
                self.parent_or_size[a] += self.parent_or_size[b];
                self.parent_or_size[b] = a as isize;
            } else {
                self.parent_or_size[b] += self.parent_or_size[a];
                self.parent_or_size[a] = b as isize;
            }
            true
        }
    }
}


fn main() {
    let mut uft = UnionFindTree::new(4);
    println!("{:?}", uft);
    println!("{:?}", uft.same(0, 1));
    uft.unite(0, 1);
    println!("{:?}", uft.same(0, 1));
    println!("{:?}", uft.same(0, 2));
    uft.unite(2, 1);
    println!("{:?}", uft);
    println!("{:?}", uft.same(0, 2));
}
