fn main() {
    let mut heap = std::collections::BinaryHeap::new();
    let mut p = 10;
    let l = 25;
    let A = vec![10, 14, 20, 21, l];
    let B = vec![10, 5, 2, 4, 0];
    let mut cnt = 0;

    let mut a = vec![0; 5];
    a[0] = A[0];
    for i in 1..A.len() {
        a[i] = A[i] - A[i - 1];
    }

    for i in 0..A.len() {
        p -= a[i];
        while p < 0 {
            match heap.pop() {
                Some(a) => {
                    println!("use {:?}", a);
                    p += a;
                    cnt += 1;
                },
                None => {
                    println!("-1");
                    return;
                }
            }
        }
        heap.push(B[i]);
    }

    println!("{:?}", cnt);
}
