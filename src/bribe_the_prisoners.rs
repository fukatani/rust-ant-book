fn split(start_idx: usize, end_idx: usize, a: &Vec<usize>) -> usize {
    let half = (end_idx - 1 + start_idx) as f32 / 2.0;
    let mut min_diff = std::f32::MAX;
    let mut min_index:usize = std::usize::MAX;
    for (i, num) in a.iter().enumerate() {
        if *num < start_idx {
            continue;
        } else if *num >= end_idx {
            break;
        }
        let num = *num as f32;
        if (num - half).abs() < min_diff {
            min_diff = (num - half).abs();
            min_index = i;
        }
    }
    if min_index == std::usize::MAX {
        0
    } else {
        let mut ret = end_idx - start_idx - 1;
        if start_idx < a[min_index] {
            ret += split(start_idx, a[min_index], &a);
        }
        if a[min_index] + 1 < end_idx {
            ret += split(a[min_index] + 1, end_idx, &a);
        }
        ret
    }
}


fn main() {
    // let p:usize = 8;
    // let _q:usize = 1;
    // let a:Vec<usize> = vec![3,];
    let p:usize = 20;
    let a: Vec<usize> = vec![3, 6, 14];

    let p:usize = 10000;
    let a: Vec<usize> = vec![65, 69, 296, 401, 495, 713, 754, 767, 899, 1158, 1431, 1465, 1473, 1725, 1769, 1797, 1985, 2005, 2036, 2113, 2291, 2434, 2449, 2575, 2589, 2653, 2665, 3022, 3031, 3238, 3430, 3500, 3628, 3676, 3803, 3815, 3848, 3944, 4137, 4222, 4511, 4618, 4758, 4778, 4933, 5018, 5106, 5212, 5259, 5321, 5342, 5401, 5516, 5625, 5766, 5966, 6133, 6220, 6484, 6487, 6488, 6544, 6707, 6802, 6923, 7028, 7092, 7223, 7279, 7513, 7514, 7560, 7569, 7644, 7732, 8057, 8117, 8161, 8171, 8185, 8257, 8338, 8358, 8482, 8503, 8507, 8670, 8709, 8753, 8771, 8778, 8975, 9206, 9406, 9487, 9529, 9568, 9717, 9785, 9909];

    println!("{:?}", split(1, p + 1, &a));
}
