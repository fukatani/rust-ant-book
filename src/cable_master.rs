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

fn is_ok(l: &Vec<f32>, k: usize, proposal_length: f32) -> bool {
    if proposal_length == 0.0 {
        return true;
    }
    let mut num = 0usize;
    for l_length in l {
        num += (l_length / proposal_length) as usize;
    }
    num >= k
}

fn main() {
    let l = vec![8.02, 7.43, 4.57, 5.39];
    let k = 11;
    let base_length = l.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() / 100.0;
    let ans = binary_search(0, 101, |i| is_ok(&l, k, base_length * i as f32));
    println!(
        "{:?}",
        (ans as f32 * base_length * 100.0) as u32 as f32 / 100.0
    );
}
