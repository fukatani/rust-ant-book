fn binary_search<F>(lb: usize, ub: usize, criterion: F) -> usize
    where F : Fn(usize) -> bool {
    let mut lb = lb;
    let mut ub = ub;
    while ub - lb > 1 {
        let mid = (ub + lb) / 2;
        if criterion(mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    lb
}

fn main() {
    let v = vec![2, 3, 3, 5, 6];
    let k = 4;
    let ans = binary_search(0, v.len(), |a| v[a] >= k);
    println!("{:?}", ans);
}
