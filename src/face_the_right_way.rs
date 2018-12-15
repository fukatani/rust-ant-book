fn main() {
    let init_cows = vec![false, false, true, false, true, false, false];

    let mut ans_m = 5000usize;
    let mut ans_k = 0usize;
    for k in 1..init_cows.len() {
        let mut m = 0usize;
        let mut used = vec![false; init_cows.len()];
        let mut current_poler = false;
        for cow_idx in 0..init_cows.len() - k + 1 {
            if cow_idx >= k && used[cow_idx - k] {
                current_poler = !current_poler;
            }
            if (!current_poler && !init_cows[cow_idx]) || (current_poler && init_cows[cow_idx]) {
                m += 1;
                used[cow_idx] = true;
                current_poler = !current_poler;
            }
        }
        let mut succeeded = true;
        for cow_idx in init_cows.len() + 1 - k..init_cows.len() {
            if cow_idx >= k && used[cow_idx - k] {
                current_poler = !current_poler;
            }
            if (!current_poler && !init_cows[cow_idx]) || (current_poler && init_cows[cow_idx]) {
                succeeded = false;
                break;
            }
        }
        if !succeeded {
            continue;
        }
        if m < ans_m {
            ans_m = m;
            ans_k = k;
        }
    }
    println!("{:?}", (ans_m, ans_k));
}
