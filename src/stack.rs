#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

fn main() {
    // let h = vec![2, 1, 4, 5, 1, 3, 3];
    // let h = vec![1, 2, 3, 4, 5, 6];
    let h = vec![6, 5, 4, 3, 2, 1];
    let n = h.len();

    let mut t = 0;
    let mut st = vec![0; n];
    let mut l = vec![0; n];
    let mut r = vec![0; n];
    for i in 0..n {
        while t > 0 && h[st[t - 1]] >= h[i] {
            t -= 1;
        }
        l[i] = if t == 0 { 0 } else { st[t - 1] + 1 };
        st[t] = i;
        t += 1;
    }
    t = 0;
    for i in (0..n).rev() {
        while t > 0 && h[st[t - 1]] >= h[i] {
            t -= 1;
        }
        r[i] = if t == 0 { n } else { st[t - 1] };
        st[t] = i;
        t += 1;
    }
    debug!(l);
    debug!(r);

    let mut ans = (0..n).map(|x| h[x] * (r[x] - l[x])).max().unwrap();
    println!("{}", ans);
}
