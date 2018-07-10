fn main() {
    let mut rows:Vec<Vec<i8>> = Vec::new();
    rows.push(vec![1, 1, 1, 0]);
    rows.push(vec![1, 1, 0, 0]);
    rows.push(vec![1, 1, 0, 0]);
    rows.push(vec![1, 0, 0, 0]);

    let mut a:Vec<i32> = vec![0; rows.len()];
    for (i, row) in rows.into_iter().enumerate() {
        a[i] = -1;
        for (j, num) in row.into_iter().enumerate() {
            if num == 1 {
                a[i] = j as i32;
            }
        }
    }
    let mut res = 0;
    for i in 0..rows.len() {
        let mut pos:usize = 0;
        for j in i..rows.len() {
            let i = i as i32;
            if a[j] <= i {
                pos = j;
                break;
            }
        }
        for j in pos..i {
            let j = j as usize;
            std::mem::swap(&mut a[j], &mut a[j-1]);
            res += 1;
        }
    }
    println!("{:?}", res);
}
