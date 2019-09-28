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

fn construct_lcp(s: &str, sa: &[usize]) -> Vec<usize> {
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
    lcp
}

fn main() {
    let s = "abracadabra";
    let a = construct_sa(s);
    assert_eq!(a, vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
    let lcp = construct_lcp(s, &a);
    assert_eq!(lcp, [0, 1, 4, 1, 1, 0, 3, 0, 0, 0, 2, 0]);
}
