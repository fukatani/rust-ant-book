fn karatsuba<T>(a: &[T], b: &[T], c: &mut [T], zero: T, buf: &mut [T])
where
    T: std::marker::Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    assert!(a.len() == b.len());
    let n = a.len();
    if n <= 16 {
        for (i, a) in a.iter().enumerate() {
            for (c, b) in c[i..].iter_mut().zip(b.iter()) {
                *c = *c + *a * *b;
            }
        }
        return;
    }
    if n & 1 == 1 {
        karatsuba(&a[1..], &b[1..], &mut c[2..], zero, buf);
        let x = a[0];
        let y = b[0];
        c[0] = c[0] + x * y;
        for (c, (a, b)) in c[1..].iter_mut().zip(a[1..].iter().zip(b[1..].iter())) {
            *c = *c + x * *b + *a * y;
        }
        return;
    }
    let m = n / 2;
    let (fa, ta) = a.split_at(m);
    let (fb, tb) = b.split_at(m);
    karatsuba(fa, fb, &mut c[..n], zero, buf);
    karatsuba(ta, tb, &mut c[n..], zero, buf);
    let (x, buf) = buf.split_at_mut(m);
    let (y, buf) = buf.split_at_mut(m);
    let (z, buf) = buf.split_at_mut(n);
    for z in z.iter_mut() {
        *z = zero;
    }
    for (x, (p, q)) in x.iter_mut().zip(fa.iter().zip(ta.iter())) {
        *x = *p + *q;
    }
    for (y, (p, q)) in y.iter_mut().zip(fb.iter().zip(tb.iter())) {
        *y = *p + *q;
    }
    karatsuba(x, y, z, zero, buf);
    for (z, (p, q)) in z.iter_mut().zip(c[..n].iter().zip(c[n..].iter())) {
        *z = *z - (*p + *q);
    }
    for (c, z) in c[m..].iter_mut().zip(z.iter()) {
        *c = *c + *z;
    }
}

fn multiply<T>(a: &[T], b: &[T], zero: T) -> Vec<T>
where
    T: std::marker::Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    let mut i = 0;
    let mut j = 0;
    let mut ans = vec![zero; a.len() + b.len()];
    let mut buf = vec![zero; 4 * std::cmp::min(a.len(), b.len())];
    let mut c = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        let x = a.len() - i;
        let y = b.len() - j;
        let z = std::cmp::min(x, y);
        c.clear();
        c.resize(2 * z, zero);
        karatsuba(&a[i..(i + z)], &b[j..(j + z)], &mut c, zero, &mut buf);
        for (ans, c) in ans[(i + j)..].iter_mut().zip(c.iter()) {
            *ans = *ans + *c;
        }
        if x <= y {
            j += x;
        } else {
            i += y;
        }
    }
    ans.truncate(a.len() + b.len() - 1);
    ans
}

fn main() {
    let a = vec![1; 3];
    let b = vec![1; 3];
    let c = multiply(&a, &b, 0);
    assert_eq!(c, vec![1, 2, 3, 2, 1]);
}
