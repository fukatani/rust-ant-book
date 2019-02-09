#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

fn grundy(i: usize, j: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
    if mem[i][j] != -1 {
        return mem[i][j];
    }
    let mut s = HashSet::new();

    for ii in 1..i / 2 + 1 {
        s.insert(grundy(i - ii * 2, j + ii, mem));
    }
    for jj in 1..j / 2 + 1 {
        s.insert(grundy(i + jj, j - jj * 2, mem));
    }

    let mut res = 0;
    while s.contains(&res) {
        res += 1;
    }
    mem[i][j] = res;
    res
}

fn main() {
    let n = 10usize;
    let mut mem = vec![vec![-1; 10 * n]; 10 * n];
    mem[0][0] = 0;
    mem[1][0] = 0;
    mem[0][1] = 0;
    mem[1][1] = 0;

    for i in 0..n {
        for j in 0..n {
            grundy(i, j, &mut mem);
        }
    }
    let mut partial_mem = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            partial_mem[i][j] = mem[i][j];
        }
    }
    println!("{:?}", partial_mem);
}

