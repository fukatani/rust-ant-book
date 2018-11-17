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
    // println!("factor list {:?}", factor_list);
    let mut factor_map: HashMap<usize, usize> = HashMap::new();
    for (j, f) in factor_list.iter().enumerate() {
        factor_map.insert(*f, j);
    }

    let mut dp = vec![vec![0f64; factor_list.len()]; n + 1];
    dp[0][0] = 1.0;

    for i in 0.. n {
        for (j, f) in factor_list.iter().enumerate() {
            let mut used_flags = [false; 7];
            if factor_map.contains_key(&(f * 6)) {
                let idx = factor_map[&(f * 6)];
                dp[i + 1][idx] += dp[i][j];
                used_flags[6] = true;
            }

            if factor_map.contains_key(&(f * 5)) {
                let idx = factor_map[&(f * 5)];
                dp[i + 1][idx] += dp[i][j];
                used_flags[5] = true;
            }

            if factor_map.contains_key(&(f * 4)) {
                let idx = factor_map[&(f * 4)];
                dp[i + 1][idx] += dp[i][j];
                used_flags[4] = true;
            }

            if factor_map.contains_key(&(f * 3)) {
                let idx = factor_map[&(f * 3)];
                if used_flags[6] {
                    dp[i + 1][idx] += dp[i][j];
                } else {
                    dp[i + 1][idx] += 2.0 * dp[i][j];
                }
                used_flags[3] = true;
            }

            if factor_map.contains_key(&(f * 2)) {
                let idx = factor_map[&(f * 2)];
                let c = used_flags[4] as u64 + used_flags[6] as u64;
                dp[i + 1][idx] += (3.0 - c as f64) * dp[i][j];
                used_flags[2] = true;
            }

            if true {
                let idx = factor_map[&(f * 1)];
                let mut c = 0;
                if used_flags[5] {
                    c += 1;
                }
                if used_flags[3] {
                    c += 2;
                    if used_flags[2] {
                        c += 2;
                    }
                } else if used_flags[2] {
                    c += 3;
                }
                dp[i + 1][idx] += (6.0 - c as f64) * dp[i][j];
                used_flags[1] = true;
            }
        }
        // println!("{:?}", dp[i + 1]);
        for j in 0..factor_list.len() {
            dp[i + 1][j] /= 6.0;
        }
    }
    println!("{:?}", dp[n][factor_list.len() - 1]);
}
