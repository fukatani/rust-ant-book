use std::cmp::*;

fn main() {
    let mut s = "mississipi".chars().collect::<Vec<_>>();
    let n = s.len();
    let mut t = s.clone();
    t.reverse();
    s.push('$');
    s.append(&mut t);
    let s: String = s.into_iter().collect();
    let sa = construct_sa(&s);
    let (lcp, rank) = construct_lcp(&s, &sa);
    let st = SparseTable::new(&lcp);
    let common = |i, j| st.rmq(min(rank[i], rank[j]), max(rank[i], rank[j]));

    // find odd length palindrome
    let mut ans = 0;
    for i in 0..n {
        let j = n * 2 - i;
        let l = common(i, j);
        ans = max(ans, 2 * l - 1);
    }

    // find even length palindrome
    for i in 1..n {
        let j = n * 2 + 1 - i;
        let l = common(i, j);
        ans = max(ans, 2 * l);
    }
    println!("{}", ans);
}

struct SparseTable {
    st: Vec<Vec<usize>>,
    lookup: Vec<usize>,
}

impl SparseTable {
    fn new(v: &Vec<usize>) -> SparseTable {
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
    fn rmq(&self, l: usize, r: usize) -> usize {
        let b = self.lookup[r - l];
        std::cmp::min(self.st[b][l], self.st[b][r - (1 << b)])
    }
}

fn compare_sa(i: usize, j: usize, k: usize, n: usize, rank: &Vec<i32>) -> bool {
    if rank[i] != rank[j] {
        return rank[i] < rank[j];
    }
    let ri = if i + k <= n { rank[i + k] } else { -1 };
    let rj = if i + k <= n { rank[j + k] } else { -1 };
    ri < rj
}

fn construct_sa(s: &str) -> Vec<usize> {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut sa = vec![0; n + 1];

    let mut rank = vec![0; n + 1];
    for i in 0..n + 1 {
        sa[i] = i;
        rank[i] = if i < n { s[i] as u64 as i32 } else { -1 };
    }

    let mut k: usize = 1;
    while k <= n {
        let mut tmp = vec![0; n + 1];
        sa.sort_by_key(|&i| (rank[i], *rank.get(i + k).unwrap_or(&-1)));
        tmp[sa[0]] = 0;
        for i in 1..n + 1 {
            tmp[sa[i]] = tmp[sa[i - 1]];
            if compare_sa(sa[i - 1], sa[i], k, n, &rank) {
                tmp[sa[i]] += 1;
            }
        }
        rank = tmp;
        k *= 2;
    }
    sa
}

fn construct_lcp(s: &str, sa: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let s = s.chars().collect::<Vec<_>>();
    let mut rank = vec![0; s.len() + 1];
    for i in 0..s.len() + 1 {
        rank[sa[i]] = i;
    }

    let mut h = 0;
    let mut lcp = vec![0; s.len() + 1];
    for i in 0..s.len() {
        let mut j = sa[rank[i] - 1];
        if h > 0 {
            h -= 1;
        }
        while j + h < s.len() && i + h < s.len() {
            if s[i + h] != s[j + h] {
                break;
            }
            h += 1;
        }

        lcp[rank[i] - 1] = h;
    }
    (lcp, rank)
}
