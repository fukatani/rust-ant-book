const INF:i32 = std::i32::MAX;

fn argmin<T>(u: &[T]) -> (usize)
    where T: Copy + PartialOrd
{
    assert!(u.len() != 0);
    let mut min_index = 0;
    let mut min = u[min_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if min > *v {
            min_index = i;
            min = *v;
        }
    }
    min_index
}


fn prim(edges: &Vec<Vec<i32>>) -> i32 {
    let mut min_cost = vec![INF; edges.len()];
    let mut used = vec![false; edges.len()];
    let mut res = 0;
    min_cost[0] = 0;
    loop {
        let not_used_indexes:Vec<usize> = (0..edges.len()).filter(|&i| {!used[i]})
                                                          .collect();
        println!("not used {:?}", not_used_indexes);
        if not_used_indexes.is_empty() {
            break;
        }
        let not_used_costs:Vec<i32> = (not_used_indexes.clone()).into_iter()
                                                                .map(|i| {min_cost[i]})
                                                                .collect();
        let min_index = not_used_indexes[argmin(&not_used_costs)];
        used[min_index] = true;
        res += min_cost[min_index];
        for u in 0..edges.len(){
            min_cost[u] = std::cmp::min(min_cost[u], edges[min_index][u]);
        }
    }
    res
}


fn main() {
    let v = 6;
    let mut edges: Vec<Vec<i32>> = vec![vec![INF; v]; v];
    for i in 0..v {
        edges[i][i] = 0;
    }
    edges[0][1] = 5;
    edges[0][2] = 4;
    edges[1][2] = 2;
    edges[2][3] = 2;
    edges[2][4] = 1;
    edges[2][5] = 4;
    edges[4][5] = 4;
    println!("{:?}", edges);
    println!("{:?}", prim(&edges));
}
