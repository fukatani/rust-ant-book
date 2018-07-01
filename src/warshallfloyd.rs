const INF:i32 = 1000;

fn warshall_floyd(mut edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for k in 0..edges.len() {
        for i in 0..edges.len() {
            for j in 0..edges.len() {
                edges[i][j] = std::cmp::min(edges[i][j], edges[i][k] + edges[k][j]);
            }
        }
    }
    edges
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
    println!("{:?}", warshall_floyd(edges));
}
