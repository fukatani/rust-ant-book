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

fn ternary_search<F, T: std::cmp::Ord>(lb: usize, ub: usize, criterion: F) -> usize
where
    F: Fn(usize) -> T,
{
    let mut lb = lb;
    let mut ub = ub;
    while ub - lb > 2 {
        let mid1 = (ub + lb * 2) / 3;
        let mid2 = (ub * 2 + lb) / 3;
        if criterion(mid1) < criterion(mid2) {
            lb = mid1;
        } else {
            ub = mid2;
        }
    }
    (lb + ub) / 2
}

fn main() {
    let v = vec![2, 3, 3, 5, 6];
<<<<<<< HEAD
    assert_eq!(2, binary_search(0, v.len(), |a| v[a] <= 4).0);
    assert_eq!(2, binary_search(0, v.len(), |a| v[a] <= 3).0);
    assert_eq!(0, binary_search(0, v.len(), |a| v[a] < 3).0);
=======
    let k = 4;
    let ans = binary_search(0, v.len(), |a| v[a] >= k);
    assert_eq!(5, v[ans]);

    let v = vec![1, 2, 3, 3, 5, 6, 4, 3, 2, 1];
    let ans = ternary_search(0, v.len(), |a| v[a]);
    assert_eq!(6, v[ans]);

    let v = vec![1, 2, 3, 3, 5, 6, 4, 3, 2];
    let ans = ternary_search(0, v.len(), |a| v[a]);
    assert_eq!(6, v[ans]);

    let v = vec![2, 3, 3, 5, 6, 4, 3, 2, 1];
    let ans = ternary_search(0, v.len(), |a| v[a]);
    assert_eq!(6, v[ans]);
>>>>>>> master
}
