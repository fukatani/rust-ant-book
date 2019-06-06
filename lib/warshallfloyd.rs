const INF:i64 = 1000000;

fn warshall_floyd(n: usize, mut edges: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                edges[i][j] = std::cmp::min(edges[i][j], edges[i][k] + edges[k][j]);
            }
        }
    }
    edges
}

fn warshall_floyd_recover(n: usize, mut edges: Vec<Vec<i64>>) {
    let mut prev = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            prev[i][j] = i;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if edges[i][k] + edges[k][j] < edges[i][j] {
                    edges[i][j] = edges[i][k] + edges[k][j];
                    prev[i][j] = prev[k][j];
                }
            }
        }
    }
    let mut path = vec![vec![vec![]; n]; n];
    for start in 0..n {
        for goal in 0..n {
            let mut cur = goal;
            while cur != start {
                path[start][goal].push(cur);
                cur = prev[start][cur];
            }
        }
    }
    println!("{:?}", path);
}

fn main() {
    let v = 6;
    let mut edges: Vec<Vec<i64>> = vec![vec![INF; v]; v];
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
    println!("{:?}", warshall_floyd(v, edges));
}
