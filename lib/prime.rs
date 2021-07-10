fn get_primes(n: i64) -> Vec<i64> {
    let mut is_prime = vec![true; n as usize + 1];
    let mut primes = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n + 1 {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    primes
}

fn factorize(mut num: i64, primes: &Vec<i64>) -> std::collections::HashMap<i64, i64> {
    // max_primes >= (num)^(1/2)
    let num_org = num;
    let mut dict = std::collections::HashMap::new();
    for &p in primes.iter() {
        while num % p == 0 {
            *dict.entry(p).or_insert(0) += 1;
            num /= p;
        }
        if num == 1 {
            break;
        }
        if p * p > num_org {
            *dict.entry(num).or_insert(0) += 1;
            break;
        }
    }
    if num != 1 {
        dict.insert(num, 1);
    }
    dict
}

fn pow(n: i64, mut p: i64) -> i64 {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r;
            p /= 2;
        } else {
            ret = ret * r;
            p -= 1;
        }
    }
    ret
}

fn collect_all_factors(dict: &std::collections::HashMap<i64, i64>) -> Vec<i64> {
    let mut ret = vec![1];
    for &key in dict.keys() {
        let mut new = ret.clone();

        for coef in 1..dict[&key] + 1 {
            let coef_1 = pow(key, coef);
            for &num in ret.iter() {
                new.push(num * coef_1);
            }
        }
        ret = new;
    }
    ret
}

fn main() {
    println!("{:?}", get_primes(20));
    println!("{:?}", factorize(10, &get_primes(20)));
    println!("{:?}", factorize(29, &get_primes(30)));
}
