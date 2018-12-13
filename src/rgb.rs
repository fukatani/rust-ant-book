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

const big_int: usize = 998244353;

fn main() {
    let v = read_vec::<usize>();
    let (n, a, b, k) = (v[0], v[1], v[2], v[3]);

    let mut comb = vec![1; n + 1];
    for i in 1..n + 1 {
        comb[i] = comb[i - 1] * (n - i) / i;
    }
    println!("{:?}", comb);

    let mut ans = 0;

    let mut a_coef = 0;
    while a * a_coef <= k && a_coef <= n {
        let cur_a_sum = a * a_coef;
        if (k - cur_a_sum) % b == 0 {
            let b_coef = (k - cur_a_sum) / b;
            ans += comb[a_coef] * comb[b_coef];
            ans %= big_int;
        }
        a_coef += 1;
    }
    println!("{:?}", ans);
}
