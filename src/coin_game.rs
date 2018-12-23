fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let x = 9;
    let k = 2;
    let a = [1, 4];

    let mut win = vec![false; x + 1];
    for i in 1..x + 1 {
        for num in a.into_iter() {
            if i >= *num {
                win[i] |= !win[i - num];
            }
        }
    }

    if win[x] {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
