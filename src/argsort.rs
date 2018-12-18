fn argsort<T: std::cmp::Ord + std::marker::Copy>(v: &Vec<T>) -> Vec<usize> {
    let mut sort_v = Vec::new();
    for i in 0..v.len() {
        sort_v.push((v[i], i));
    }
    sort_v.sort();
    sort_v.iter().map(|x| x.1).collect::<Vec<usize>>()
}

fn main() {
    let v = vec![3, 2, 1, 4, 5];
    println!("{:?}", argsort(&v));
}
