fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn accum_sum(inputs: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![0; inputs.len()];
    ret[0] = inputs[0];
    for i in 1..inputs.len() {
        ret[i] = ret[i - 1] + inputs[i];
    }
    return ret;
}

fn sum_order(input: i32) -> i32 {
    let mut ret = 0;
    let numstr: String = input.to_string();
    for i in 0..numstr.len() {
        match numstr.chars().nth(i) {
            None => break,
            Some(c) => {
                ret += c as i32 - 48;
            }
        }
    }
    return ret;
}

fn imos() {
    let H = 30;
    let W = 20;
    let mut tiles = [[0; 20]; 30];
    let A = [0, 5, 7];
    let B = [2 , 6 ,17];
    let C = [0, 15, 7];
    let D = [20, 16 ,27];

    for i in 0..A.len() {
        tiles[C[i]][A[i]] = 1;
        tiles[D[i]][B[i]] = 1;
        tiles[C[i]][B[i]] = -1;
        tiles[D[i]][A[i]] = -1;
    }

    for y in 0..H {
        for x in 1..W {
            tiles[y][x] += tiles[y][x - 1];
        }
    }

    for y in 1..H {
        for x in 0..W {
            tiles[y][x] += tiles[y - 1][x];
        }
    }

    println!("{:?}", tiles);
}

fn syakutori() {
    let (n, k) = (4, 10);
    let a = vec![6, 1, 2, 7];
    let mut r = 0;
    let mut cur = 0;
    let mut ans = 0;

    let check = |cond| cond >= k; // define continue condition

    for i in 0..n {
        while r < n && !check(cur) {
            cur += a[r]; // update end
            r += 1;
        }
        if !check(cur) {
            break;
        }
        ans += n + 1 - r;
        cur -= a[i]; // update start
    }
    println!("{}", ans);
}

fn kadane(points: &Vec<i64>) -> (i64, usize, usize) {
    let mut res = std::i64::MIN;
    let mut sub = 0;

    // range of subarray
    let mut start = 0;
    let mut end = 0;

    for (i, &n) in points.iter().enumerate() {
        let mut s = start;

        // At the i-th time, `sub` denotes the sum of the maximum subarray
        // that ends at element i.
        if sub + n <= n {
            sub = n;
            s = i;
        } else {
            sub = sub + n;
        }

        if res < sub {
            res = sub;
            start = s;
            end = i + 1;
        }
    }

    (res, start, end)
}

// n ^ p mod m
fn pow_m(n: i64, mut p: i64, m: i64) -> i64 {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % m;
            p /= 2;
        } else {
            ret = ret * r % m;
            p -= 1;
        }
    }
    ret
}

fn argmin<T>(u: &[T]) -> (usize)
    where T: Copy + PartialOrd
{
    assert!(u.len() != 0);

    let mut min_index = 0;
    let mut min = u[min_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if min > *v {
            min_index = i;
            min = *v;
        }
    }
    min_index
}

fn argmax<T>(u: &[T]) -> (usize)
    where T: Copy + PartialOrd
{
    assert!(u.len() != 0);

    let mut max_index = 0;
    let mut max = u[max_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if max < *v {
            max_index = i;
            max = *v;
        }
    }
    max_index
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() {
    let nums: Vec<i32> = read_vec();
    let ans = accum_sum(nums);
    println!("{:?}", ans);
    imos();
}

fn get_adjacents_and_diagonal(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut adjacents = vec![];
    if x > 0 {
        if y > 0 {
            adjacents.push((x - 1, y - 1));
        }
        adjacents.push((x - 1, y));
        if y < h - 1 {
            adjacents.push((x - 1, y + 1));
        }
    }
    if y > 0 {
        adjacents.push((x, y - 1));
    }
    if y < h - 1 {
        adjacents.push((x, y + 1));
    }
    if x < w - 1 {
        if y > 0 {
            adjacents.push((x + 1, y - 1));
        }
        adjacents.push((x + 1, y));
        if y < h - 1 {
            adjacents.push((x + 1, y + 1));
        }
    }
    adjacents
}

fn get_adjacents(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut adjacents = vec![];
    if x > 0 {
        adjacents.push((x - 1, y));
    }
    if y > 0 {
        adjacents.push((x, y - 1));
    }
    if y < h - 1 {
        adjacents.push((x, y + 1));
    }
    if x < w - 1 {
        adjacents.push((x + 1, y));
    }
    adjacents
}

#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn loop_for_subset(s: i64) {
    let mut t = s;
    loop {
        println!("{}", t);
        if t == 0 {
            break;
        }
        t = (t - 1) & s;
    }
}

pub fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = a.rem_euclid(b);
    if a == 0 {
        return (b, 0);
    }
    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;

    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut m0, &mut m1);
    }
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}

// return y (mod z)
pub fn crt(r: &[i64], m: &[i64]) -> Option<(i64, i64)> {
    assert_eq!(r.len(), m.len());
    // Contracts: 0 <= r0 < m0
    let (mut r0, mut m0) = (0, 1);
    for (&(mut ri), &(mut mi)) in r.iter().zip(m.iter()) {
        assert!(1 < mi);
        ri = ri.rem_euclid(mi);
        if m0 < mi {
            std::mem::swap(&mut r0, &mut ri);
            std::mem::swap(&mut m0, &mut mi);
        }
        if m0 % mi == 0 {
            if r0 % mi != ri {
                return None;
            }
            continue;
        }
        let (g, im) = inv_gcd(m0, mi);
        let u1 = mi / g;
        // |ri - r0| < (m0 + mi) <= lcm(m0, mi)
        if (ri - r0) % g != 0 {
            return None;
        }
        let x = (ri - r0) / g % u1 * im % u1;

        r0 += x * m0;
        m0 *= u1; // -> lcm(m0, mi)
        if r0 < 0 {
            r0 += m0
        };
    }

    Some((r0, m0))
}

fn rotate<T: Clone>(a: &[Vec<T>]) -> Vec<Vec<T>> {
    if a.is_empty() {
        return vec![];
    }
    let h = a.len();
    let w = a[0].len();
    assert!(a.iter().all(|a| a.len() == w));
    let mut b = (0..w).map(|_| Vec::with_capacity(h)).collect::<Vec<_>>();
    for a in a.iter().rev() {
        for (b, a) in b.iter_mut().zip(a.iter()) {
            b.push(a.clone());
        }
    }
    b
}

fn diagonal_reverse<T>(grid: &mut Vec<Vec<T>>) {
    grid.reverse();
    for line in grid.iter_mut() {
        line.reverse();
    }
}

fn calc_sq(x: i64) -> i64 {
    let sq = (x as f64).sqrt() as i64;
    for ret in sq - 2.. {
        if (ret + 1) * (ret + 1) > x {
            return ret;
        }
    }
    unreachable!();
}
