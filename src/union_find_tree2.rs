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

    fn find(&self, index: usize) -> usize {
        let mut index = index;
        while self.parent[index] >= 0 {
            index = self.parent[index] as usize;
        }
        index
    }

    fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn get_size(&self, x: usize) -> usize {
        self.size[self.find(x)]
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
