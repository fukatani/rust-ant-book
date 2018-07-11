fn main() {
    let mut rows:Vec<Vec<i8>> = Vec::new();
    rows.push(vec![1, 1, 1, 0]);
    rows.push(vec![1, 1, 0, 0]);
    rows.push(vec![1, 1, 0, 0]);
    rows.push(vec![1, 0, 0, 0]);

    let mut a:Vec<i32> = vec![0; rows.len()];
    for (i, row) in rows.iter().enumerate() {
        a[i] = -1;
        for (j, num) in row.iter().enumerate() {
            if *num == 1 {
                a[i] = j as i32;
            }
        }
    }
    let mut res = 0;

    for i in 0..rows.len() {
        let mut pos:usize = 0;
        for j in i..rows.len() {
            if a[j] <= i as i32 {
                pos = j;
                break;
            }
        }
        for j in (i+1..pos+1).rev() {
            a.swap(j, j - 1);
            res += 1;
        }
    }
    println!("{:?}", res);
}
