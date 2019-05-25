fn get_primes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n + 1 {
        if is_prime[i] {
            primes.push(i);
            let mut j = 2 * i;
            while j < n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn factorize(mut num: usize, primes: &Vec<usize>) -> std::collections::HashMap<usize, usize>  {
    // max_primes >= (num)^(1/2)
    let mut dict = std::collections::HashMap::new();
    for &p in primes.iter() {
        while num % p == 0 {
            *dict.entry(p).or_insert(0) += 1usize;
            num /= p;
        }
        if num == 1 {
            break;
        }
    }
    if num != 1 {
        dict.insert(num, 1);
    }
    dict
}

fn pow(n: usize, mut p: usize) -> usize {
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

fn collect_all_factors(dict: &std::collections::HashMap<usize, usize>) -> Vec<usize> {
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
