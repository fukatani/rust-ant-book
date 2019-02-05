fn grundy(w: usize, h:usize, mem: &mut Vec<Vec<i32>>) -> i32 {
    if mem[w][h] != -1 {
        return mem[w][h];
    }
    let mut s = std::collections::HashSet::new();
    for i in 2..w - 1 {
        s.insert(grundy(i, h, mem) ^ grundy(w - 1, h, mem));
    }
    for i in 2..h - 1 {
        s.insert(grundy(w, i, mem) ^ grundy(w, h - i, mem));
    }
    let mut res = 0;
    while s.contains(&res) {
        res += 1;
    }
    mem[w][h] = res;
    res
}

fn main() {
    let w = 4;
    let h = 2;
    let mut mem = vec![vec![-1; h + 1]; w + 1];
    if grundy(w, h, &mut mem) != 0 {
        println!("WIN");
    } else {
        println!("LOSE");
    }
}
