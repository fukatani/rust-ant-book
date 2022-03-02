#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

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
    let n = read::<usize>();
    let a = read_vec::<i64>();
    let mut a_digits = vec![vec![0; 6]; n];
    for i in 0..n {
        let mut aa = a[i];
        for j in 0..6 {
            a_digits[i][j] = (aa % 10) as usize;
            aa /= 10;
        }
    }

    let mut imos = vec![vec![vec![vec![vec![vec![0; 11]; 11]; 11]; 11]; 11]; 11];
    for i in 0..n {
        for state in 0i32..1 << 6 {
            let is = (0..6)
                .map(|x| {
                    if ((state >> x) & 1) == 0 {
                        0
                    } else {
                        9 - a_digits[i][x] + 1
                    }
                })
                .collect::<Vec<_>>();
            if state.count_ones() % 2 == 0 {
                imos[is[0]][is[1]][is[2]][is[3]][is[4]][is[5]] += 1;
            } else {
                imos[is[0]][is[1]][is[2]][is[3]][is[4]][is[5]] -= 1;
            }
        }
    }
    for i0 in 1..11 {
        for i1 in 0..11 {
            for i2 in 0..11 {
                for i3 in 0..11 {
                    for i4 in 0..11 {
                        for i5 in 0..11 {
                            imos[i0][i1][i2][i3][i4][i5] += imos[i0 - 1][i1][i2][i3][i4][i5];
                        }
                    }
                }
            }
        }
    }
    for i1 in 1..11 {
        for i0 in 0..11 {
            for i2 in 0..11 {
                for i3 in 0..11 {
                    for i4 in 0..11 {
                        for i5 in 0..11 {
                            imos[i0][i1][i2][i3][i4][i5] += imos[i0][i1 - 1][i2][i3][i4][i5];
                        }
                    }
                }
            }
        }
    }
    for i2 in 1..11 {
        for i0 in 0..11 {
            for i1 in 0..11 {
                for i3 in 0..11 {
                    for i4 in 0..11 {
                        for i5 in 0..11 {
                            imos[i0][i1][i2][i3][i4][i5] += imos[i0][i1][i2 - 1][i3][i4][i5];
                        }
                    }
                }
            }
        }
    }
    for i3 in 1..11 {
        for i0 in 0..11 {
            for i1 in 0..11 {
                for i2 in 0..11 {
                    for i4 in 0..11 {
                        for i5 in 0..11 {
                            imos[i0][i1][i2][i3][i4][i5] += imos[i0][i1][i2][i3 - 1][i4][i5];
                        }
                    }
                }
            }
        }
    }
    for i4 in 1..11 {
        for i0 in 0..11 {
            for i1 in 0..11 {
                for i2 in 0..11 {
                    for i3 in 0..11 {
                        for i5 in 0..11 {
                            imos[i0][i1][i2][i3][i4][i5] += imos[i0][i1][i2][i3][i4 - 1][i5];
                        }
                    }
                }
            }
        }
    }
    for i5 in 1..11 {
        for i0 in 0..11 {
            for i1 in 0..11 {
                for i2 in 0..11 {
                    for i3 in 0..11 {
                        for i4 in 0..11 {
                            imos[i0][i1][i2][i3][i4][i5] += imos[i0][i1][i2][i3][i4][i5 - 1];
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0i64;
    for i in 0..n {
        ans += imos[a_digits[i][0]][a_digits[i][1]][a_digits[i][2]][a_digits[i][3]][a_digits[i][4]]
            [a_digits[i][5]];
        if (0..6).all(|j| a_digits[i][j] * 2 <= 9) {
            ans -= 1;
        }
    }
    println!("{}", ans / 2);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
