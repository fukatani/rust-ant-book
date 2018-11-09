use std::collections::*;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let v: Vec<usize> = read_vec();
    let (n, d) = (v[0], v[1]);
    let mut d = d;

    let mut factors: HashMap<usize, usize> = HashMap::new();
    factors.insert(2, 0);
    factors.insert(3, 0);
    factors.insert(5, 0);

    let primes = [2, 3, 5];
    let mut p_idx: usize = 0;
    while p_idx < 3 {
        let p = primes[p_idx];
        if d % p == 0 {
            d /= p;
            let a = factors.get_mut(&p).unwrap();
            *a += 1;
        } else {
            p_idx += 1;
        }
    }

    // let prod = factors.values().product();

    if d != 1 {
        println!("0.0");
        return;
    } else if factors.values().sum::<usize>() == 0 {
        println!("1.0");
        return;
    }

    let mut factor_list = vec![1];
    for p in factors.keys() {
        let mut list_clone = factor_list.clone();
        for i in 0..factors[p] {
            let cur_num = p.pow(i as u32 + 1);
            for elem in factor_list.iter() {
                list_clone.push(elem * cur_num);
            }
        }
        factor_list = list_clone;
    }
    factor_list.sort();
    println!("{:?}", factor_list);


    let factor_table = std::collections::HashMap::new();
    for (i, f) in factor_list.iter() {

    }
    let dp = vec![vec![0; n + 1]; factor_list.len()];
    for i in 0.. n + 1 {
        for num in (2, 3, 6) {
            dp[i];
        }
    }
}
