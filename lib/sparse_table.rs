struct SparseTable {
    st: Vec<Vec<i64>>,
    lookup: Vec<usize>,
}

impl SparseTable {
    fn new(v: &Vec<i64>) -> SparseTable {
        let mut b = 0;
        while (1 << b) <= v.len() {
            b += 1;
        }
        let mut st = vec![vec![0; 1 << b]; b];
        for i in 0..v.len() {
            st[0][i] = v[i];
        }
        for i in 1..b {
            let mut j = 0;
            while j + (1 << i) <= (1 << b) {
                st[i][j] = std::cmp::min(st[i - 1][j], st[i - 1][j + (1 << (i - 1))]);
                j += 1;
            }
        }
        let mut lookup = vec![0; v.len() + 1];
        for i in 2..lookup.len() {
            lookup[i] = lookup[i >> 1] + 1;
        }
        SparseTable {
            st: st,
            lookup: lookup,
        }
    }
    // [l, r)
    fn rmq(&self, l: usize, r: usize) -> i64 {
        let b = self.lookup[r - l];
        std::cmp::min(self.st[b][l], self.st[b][r - (1 << b)])
    }
}

fn main() {
    let v = vec![2,5,3,9,8,2,10,1,7,2,1,6];
    let st = SparseTable::new(&v);
    assert_eq!(st.rmq(1, 2), 5);
    assert_eq!(st.rmq(1, 3), 3);
    assert_eq!(st.rmq(0, 3), 2);
    assert_eq!(st.rmq(0, v.len()), 1);
}
