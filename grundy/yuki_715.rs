#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn grundy(n: usize, mem: &mut Vec<i32>) -> i32 {
    if mem[n] != -1 {
        return mem[n];
    }
    let mut s = HashSet::new();
    s.insert(grundy(n / 2, mem) ^ grundy(n - n / 2, mem));
    s.insert(grundy(n / 3, mem) ^ grundy((1 + n) / 3, mem) ^ grundy((2 + n) / 3, mem));

    let mut res = 0;
    while s.contains(&res) {
        res += 1;
    }
    mem[n] = res;
    res
}

fn main() {
    let n: usize = read();
    let mut mem = vec![-1; n + 1];
    mem[0] = 0;
    mem[1] = 0;

    if grundy(n, &mut mem) == 0 {
        println!("B");
    } else {
        println!("A");
    }
}

