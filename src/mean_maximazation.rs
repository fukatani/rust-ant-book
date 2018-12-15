#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn binary_search<F>(lb: f32, ub: f32, criterion: F) -> f32
where
    F: Fn(f32) -> bool,
{
    let mut lb = lb;
    let mut ub = ub;
    for _ in 0..100 {
        let mid = (ub + lb) / 2.0;
        if criterion(mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    lb
}

fn is_ok(ws: &Vec<usize>, vs: &Vec<usize>, k: usize, proposal_weight: f32) -> bool {
    let mut points = vec![0f32; ws.len()];
    for i in 0..ws.len() {
        points[i] = vs[i] as f32 - proposal_weight * ws[i] as f32;
    }
    points.sort_by_key(|&x| Total(x));
    let sum = points.iter().rev().take(k).sum::<f32>();
    // println!("{:?}", (proposal_weight, sum, points));
    sum >= 0.0
}

fn main() {
    let ws: Vec<usize> = vec![2, 5, 2];
    let vs: Vec<usize> = vec![2, 3, 1];
    let k = 2;

    let search_max = *vs.iter().max().unwrap() as f32 / *ws.iter().min().unwrap() as f32;
    let ans = binary_search(0.0, search_max, |i| is_ok(&ws, &vs, k, i));
    println!("{:?}", ans);
}
