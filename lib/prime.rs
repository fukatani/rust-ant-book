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
    dict
}

fn main() {
    println!("{:?}", get_primes(20));
    println!("{:?}", factorize(10, &get_primes(20)));
}
