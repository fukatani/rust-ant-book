fn main() {
    let n = 20000;
    let mut is_prime = vec![true; n];
    let mut primes = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n {
        if is_prime[i] {
            primes.push(i);
            let mut j = 2 * i;
            while j < n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    println!("{:?}", primes);
}
