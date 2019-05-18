#[derive(Debug, Clone)]
struct UnionFindTree {
    parent: Vec<isize>,
    size: Vec<usize>,
    height: Vec<u64>,
}

impl UnionFindTree {
    fn new(n: usize) -> UnionFindTree {
        UnionFindTree {
            parent: vec![-1; n],
            size: vec![1usize; n],
            height: vec![0u64; n],
        }
    }

    fn find(&mut self, index: usize) -> usize {
        if self.parent[index] == -1 {
            return index;
        }
        let idx = self.parent[index] as usize;
        let ret = self.find(idx);
        self.parent[index] = ret as isize;
        ret
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn get_size(&mut self, x: usize) -> usize {
        let idx = self.find(x);
        self.size[idx]
    }

    fn unite(&mut self, index0: usize, index1: usize) -> bool {
        let a = self.find(index0);
        let b = self.find(index1);
        if a == b {
            false
        } else {
            if self.height[a] > self.height[b] {
                self.parent[b] = a as isize;
                self.size[a] += self.size[b];
            } else if self.height[a] < self.height[b] {
                self.parent[a] = b as isize;
                self.size[b] += self.size[a];
            } else {
                self.parent[b] = a as isize;
                self.size[a] += self.size[b];
                self.height[a] += 1;
            }
            true
        }
    }
}

fn main() {
    let mut uft = UnionFindTree::new(4);
    println!("{:?}", uft);
    assert_eq!(false, uft.same(0, 1));
    uft.unite(0, 1);
    assert_eq!(true, uft.same(0, 1));
    assert_eq!(false, uft.same(0, 2));
    uft.unite(2, 1);
    println!("{:?}", uft);
    assert_eq!(true, uft.same(0, 2));
    assert_eq!(3, uft.get_size(2));
    assert_eq!(0, uft.find(2));
}
