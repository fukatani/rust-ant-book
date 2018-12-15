#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn binary_search<F>(lb: usize, ub: usize, criterion: F) -> usize
where
    F: Fn(usize) -> bool,
{
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

fn is_ok(x: &Vec<usize>, m: usize, proposal_length: usize) -> bool {
    let mut last_placed_pos = x[0];
    let mut placed_count = 1;
    for pos in x.iter().skip(1) {
        if *pos - last_placed_pos >= proposal_length {
            last_placed_pos = *pos;
            placed_count += 1;
        }
    }
    placed_count >= m
}

fn main() {
    let mut x: Vec<usize> = vec![1, 2, 8, 4, 9];
    let m = 3;

    x.sort();
    let search_max = *x.iter().max().unwrap() - *x.iter().min().unwrap();
    let ans = binary_search(1, search_max, |i| is_ok(&x, m, i));
    println!("{:?}", ans);
}
