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
    let a = vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8];
    let S = 20;
    let mut sum = 0;

    let mut s: usize = 0;
    let mut t: usize = 0;
    let mut res = 0;
    'outer: loop {
        while t < a.len() && sum < S {
            t += 1;
            if t == a.len() {
                break 'outer;
            }
            sum += a[t];
        }
        res = std::cmp::min(res, t - s);
        s += 1;
        sum -= a[s];
    }
    println!("{:?}", res);
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


fn main() {
    let nums: Vec<i32> = read_vec();
    let ans = accum_sum(nums);
    println!("{:?}", ans);
    imos();
}
