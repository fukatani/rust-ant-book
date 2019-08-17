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
    adjacents.push((x, y));
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
    adjacents.push((x, y));
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
