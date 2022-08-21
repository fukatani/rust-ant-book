// g[T] = sum(f[S]) where T is subset of S
fn subset_zeta_transform<T, F>(f: &mut Vec<T>, op: F)
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    let n = f.len();
    let mut i = 1;
    while i < n {
        for j in 0..n {
            if j & i != 0 {
                f[j] = op(f[j], f[j ^ i]);
                // f[j] = op(f[j], f[j | i]); for where T is superset of S
            }
        }
        i <<= 1;
    }
}
