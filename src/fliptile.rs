#[derive(Clone, Debug)]
pub struct BitSet {
    buf: Vec<bool>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize, value: u32) -> BitSet {
        let mut buf = vec![false; size];
        let mut value = value;
        for i in 0..size {
            if value % 2 == 1 {
                buf[i] = true;
            }
            value = value >> 1;
        }
        BitSet {
            buf: buf,
            size: size,
        }
    }

    pub fn as_integer(&self) -> u32 {
        let mut value = 0;
        for i in (0..self.size).rev() {
            value = value << 1;
            if self.buf[i] {
                value += 1;
            }
        }
        value
    }
}

impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        &self.buf[index]
    }
}

#[derive(Clone, Debug)]
struct Tiles {
    tiles: Vec<Vec<bool>>
}

impl Tiles {
    fn reverse(&mut self, i: usize, j: usize) {
        self.tiles[i][j] = !self.tiles[i][j];
        if i > 0 {
            self.tiles[i - 1][j] = !self.tiles[i - 1][j];
        }
        if j > 0 {
            self.tiles[i][j - 1] = !self.tiles[i][j - 1];
        }
        if i < self.tiles.len() - 1 {
            self.tiles[i + 1][j] = !self.tiles[i + 1][j];
        }
        if j < self.tiles[0].len() - 1 {
            self.tiles[i][j + 1] = !self.tiles[i][j + 1];
        }
    }
}

fn main() {
    let m = 4usize;
    let n = 4usize;
    let mut tiles = vec![Vec::new(); n];

    tiles[0] = vec![true, false, false, true];
    tiles[1] = vec![false, true, true, false];
    tiles[2] = vec![false, true, true, false];
    tiles[3] = vec![true, false, false, true];

    let tiles = Tiles{ tiles: tiles };

    let mut ans = std::u32::MAX;
    for p in 0..1 << n {
        let mut tiles_clone = tiles.clone();
        let mut count = 0;
        let bs = BitSet::new(n, p);

        // i = 0
        for j in 0..m {
            if bs[j] {
                tiles_clone.reverse(0, j);
                count += 1;
            }
        }
        for i in 0..n - 1 {
            for j in 0..m {
                if !tiles_clone.tiles[i][j] {
                    tiles_clone.reverse(i + 1, j);
                    count += 1
                }
            }
        }

        // i = n - 1
        if !tiles_clone.tiles[n - 1].iter().all(|x| *x) {
            continue;
        }
        ans = std::cmp::min(ans, count);
    }
    println!("{:?}", ans);
}
