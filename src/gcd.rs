fn gcd(a: i32, b:i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
