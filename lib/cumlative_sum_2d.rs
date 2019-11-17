struct CumulativeSum2D {
    data: Vec<Vec<i64>>,
}

impl CumulativeSum2D {
    fn new(w: usize, h: usize) -> CumulativeSum2D {
        CumulativeSum2D {
            data: vec![vec![0; w + 1]; h + 1],
        }
    }

    fn add(&mut self, mut x: usize, mut y: usize, z: i64) {
        x += 1;
        y += 1;
        if x >= self.data[0].len() || y >= self.data.len() {
            return;
        }
        self.data[y][x] += z;
    }

    fn build(&mut self) {
        for i in 1..self.data.len() {
            for j in 1..self.data[i].len() {
                self.data[i][j] +=
                    self.data[i][j - 1] + self.data[i - 1][j] - self.data[i - 1][j - 1];
            }
        }
    }

    // sum of [sx, gx), [sy, gy)
    fn query(&self, sx: usize, sy: usize, gx: usize, gy: usize) -> i64 {
        self.data[gy][gx] + self.data[sy][sx] - self.data[gy][sx] - self.data[sy][gx]
    }
}

fn main() {
    let mut cum2d = CumulativeSum2D::new(3, 8);
    cum2d.add(2, 7, 1);
    cum2d.add(2, 6, 1);
    cum2d.add(2, 5, 4);
    cum2d.add(1, 4, -2);
    cum2d.add(1, 2, 3);
    cum2d.build();

    assert_eq!(cum2d.query(0, 0, 1, 4), 0);
    assert_eq!(cum2d.query(0, 0, 2, 4), 3);
    assert_eq!(cum2d.query(0, 0, 2, 6), 1);
    assert_eq!(cum2d.query(0, 0, 3, 6), 5);
    assert_eq!(cum2d.query(0, 0, 3, 8), 7);
    assert_eq!(cum2d.query(2, 6, 3, 8), 2);
}
