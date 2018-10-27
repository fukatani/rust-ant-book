use std::collections::*;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn accum_prod(val: u32) -> u32 {
    return (1..val + 1).product();
}

fn calc_prob_coef(indexes: &Vec<usize>) -> f32 {
    let mut index_dict = HashMap::new();
    for idx in indexes {
        *index_dict.entry(idx).or_insert(0) += 1;
    }
    let mut prob = 1.0;
    for value in index_dict.values() {
        prob *= accum_prod(*value) as f32;
    }
    return prob;
}

fn main() {
    let v: Vec<usize> = read_vec();
    let (n, m, q) = (v[0], v[1], v[2]);
    let q = q as f32;

    let mut xs = vec![0u32; m];
    let mut ps = vec![0u32; m];

    for i in 0..m {
        let v: Vec<u32> = read_vec();
        xs[i] = v[0];
        ps[i] = v[1];
    }

    let mut values = vec![vec![0; n]; m];
    let mut probs = vec![vec![0.0; n]; m];
    let mut indexes = vec![0usize; n];
    for i in 0..m {
        let mut denom = 1.0;
        for j in 0..n {
            denom *= ps[indexes[j]] as f32 / q;
        }
        for j in 0..n {
            for k in 0..n {
                values[i][j] = xs[indexes[k]];
            }
            probs[i][j] = calc_prob_coef(&indexes) * denom;
            println!("{0} {1} {2} {3} {4}", i, j, calc_prob_coef(&indexes), denom, probs[i][j]);
            indexes[j] += 1;
        }
    }
    println!("{:?}", values);
    println!("{:?}", probs);

    let mut expect = vec![0f32; n];
    for i in 0..n {
        for j in 0..m {
            expect[i] += probs[j][i] * values[j][i] as f32;
        }
    }
    println!("{:?}", expect);
}
