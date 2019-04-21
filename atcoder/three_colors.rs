#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let v = read_vec::<usize>();
    let (n, m) = (v[0], v[1]);
    let mut info = vec![Vec::new(); n];
    for i in 0..m {
        let v = read_vec::<i64>();
        let l = v[0] as usize - 1;
        let r = v[1] as usize - 1;
        info[l].push((r, v[2]));
        info[r].push((l, -v[2]));
    }
    // println!("{:?}", info);

    let mut poses = vec![std::i64::MAX; n];
    for i in 0..n {
        if poses[i] != std::i64::MAX {
            continue;
        }
        poses[i] = 0;
        let mut que = VecDeque::new();
        que.push_back(i);

        while let Some(l) = que.pop_back() {
            let origin = poses[l];
            for &(r, dist) in info[l].iter() {
                if poses[r] == std::i64::MAX {
                    poses[r] = origin + dist;
                    que.push_back(r);
                }
                if poses[r] != origin + dist {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
