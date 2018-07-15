use std::hash::{Hash, Hasher};

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[derive(Hash, Clone)]
struct MyStr {
    value: Vec<char>,
}

impl Eq for MyStr {}

impl PartialEq for MyStr {
    fn eq(&self, other: &MyStr) -> bool {
        for (i, j) in self.value.iter().zip(other.value.iter()) {
            if i != j {
                return false;
            }
        }
        true
    }
}

impl Ord for MyStr {
    fn cmp(&self, other: &MyStr) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for MyStr {
    fn partial_cmp(&self, other: &MyStr) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for MyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyStr {{ s: {} }}", self.to_string())
    }
}

impl std::fmt::Display for MyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl MyStr {
    fn new() -> MyStr {
        return MyStr{value: Vec::new()}
    }

    fn from_string(value: String) -> MyStr {
        return MyStr{value: value.chars().collect()}
    }

    fn push(&mut self, ch: char) {
        self.value.push(ch)
    }

    fn reverse(&mut self) {
        self.value.reverse();
    }

    fn repeat(&self, n:usize) -> MyStr {
        let mut ret = MyStr::new();
        for _i in 0..n {
            ret = ret + self.clone();
        }
        ret
    }

    fn to_string(&self) -> String {
        let mut res: String = "".to_string();
        for ch in self.value.iter() {
            res.push(ch.clone());
        }
        res
        // self.value.iter().collect()
    }

    fn slice(&self, start_idx: usize, end_idx: usize) -> MyStr {
        MyStr{value: self.value[start_idx..end_idx].to_vec()}
    }
}

impl Iterator for MyStr {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        None
    }
}

impl ExactSizeIterator for MyStr {
    fn len(&self) -> usize {
        self.value.len()
    }
}

impl std::ops::Add for MyStr {
    type Output = MyStr;
    fn add(self, other: MyStr) -> MyStr {
        let mut cont = self.value.clone();
        cont.append(&mut other.value.clone());
        MyStr{value: cont}
    }
}


fn main() {
    let _n: usize = read();
    let s: String = read();
    let n = s.len() / 2;
    let s:Vec<char> = s.chars().collect();

    let left_word:Vec<&char> = s[..s.len()/2].iter().collect();
    let mut left_map = std::collections::HashMap::new();

    for i in 0..2 << (n - 1) {
        let mut i = i;
        let mut blue = MyStr::new();
        let mut red = MyStr::new();
        for j in 0..n {
            if i % 2 == 1 {
                blue.push(left_word[j].clone());
            } else {
                red.push(left_word[j].clone());
            }
            i = i >> 1;
        }
        red.reverse();
        blue.reverse();
        *left_map.entry((red, blue)).or_insert(0 as i64) += 1;
    }

    let right_word:Vec<&char> = s[s.len()/2..].iter().collect();
    let mut right_map = std::collections::HashMap::new();

    for i in 0..2 << (n - 1) {
        let mut i = i;
        let mut blue = MyStr::new();
        let mut red = MyStr::new();
        for j in 0..n {
            if i % 2 == 1 {
                blue.push(right_word[j].clone());
            } else {
                red.push(right_word[j].clone());
            }
            i = i >> 1;
        }
        *right_map.entry((blue, red)).or_insert(0 as i64) += 1;
    }

    let mut res:i64 = 0;
    /*
    println!("left\n{:?}", left_map);
    println!("\nright\n");
    println!("{:?}", right_map);
    */
    for (key, lvalue) in &left_map {
        if let Some(rvalue) = right_map.get(key) {
            res += lvalue * rvalue;
        }
    }
    println!("{:?}", res);
}
