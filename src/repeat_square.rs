fn mod_pow(x: i64, n:i64, modula:i64) -> i64 {
    let mut res:i64 = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x % modula;
        }
        x = x * x % modula;
        n = n >> 1;
    }
    res
}

fn sieve(n:usize) -> (Vec<usize>, Vec<bool>) {
    let mut prime_numbers: Vec<usize> = Vec::new();
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n+1 {
        if is_prime[i] {
            prime_numbers.push(i);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    (prime_numbers, is_prime)
}

fn main() {
    let n:i64 = 561;
    let (_, is_prime) = sieve(n as usize);
    if is_prime[n as usize] {
        println!("No");
        return;
    }
    for i in 1..n {
        if (mod_pow(i, n, n)) != i {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
