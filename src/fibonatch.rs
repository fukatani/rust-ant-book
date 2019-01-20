type mat = Vec<Vec<i64>>;

fn mat_zeros(n: usize, m: usize) -> mat {
    vec![vec![0i64; m]; n]
}

fn matmul(a: &mat, b: &mat) -> mat {
    let mut c = mat_zeros(a.len(), b[0].len());
    for i in 0..a.len() {
        for k in 0..b.len() {
            for j in 0..b[0].len() {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn matpow(a: &mat, n: u64) -> mat {
    let mut b = mat_zeros(a.len(), a.len());
    for i in 0..a.len() {
        b[i][i] = 1;
    }
    let mut n = n;
    let mut a: mat = a.clone();
    while n > 0 {
        if n & 1 == 1 {
            b = matmul(&a, &b);
        }
        a = matmul(&a, &a);
        n >>= 1;
    }
    b
}

fn main() {
    let mut a = mat_zeros(2, 2);
    a[0][0] = 1;
    a[0][1] = 1;
    a[1][0] = 1;
    a[1][1] = 0;
    a = matpow(&a, 10);
    println!("{}", a[1][0]);
}
