fn binary_search<F>(lb: usize, ub: usize, criterion: F) -> (usize, usize)
where
    F: Fn(usize) -> bool,
{
    let mut ok = lb;
    let mut ng = ub;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if criterion(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ok, ng)
}

fn main() {
    let v = vec![2, 3, 3, 5, 6];
    assert_eq!(2, binary_search(0, v.len(), |a| v[a] <= 4).0);
    assert_eq!(2, binary_search(0, v.len(), |a| v[a] <= 3).0);
    assert_eq!(0, binary_search(0, v.len(), |a| v[a] < 3).0);
}
