const INF: i64 = std::i64::MAX;

fn shortest_path(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> (Vec<i64>, Vec<bool>) {
    let n = graph.len();
    let mut dist = vec![INF; n];
    dist[start] = 0;
    for _ in 0..n {
        for v in 0..n {
            for &(to, cost) in &graph[v] {
                if dist[v] == INF || dist[to] <= dist[v] + cost {
                    continue;
                }
                dist[to] = dist[v] + cost;
            }
        }
    }

    let mut negative = vec![false; n];
    for _ in 0..n {
        for v in 0..n {
            for &(to, cost) in &graph[v] {
                if dist[v] == INF {
                    continue;
                }
                if dist[to] > dist[v] + cost {
                    dist[to] = dist[v] + cost;
                    negative[to] = true;
                }
                if negative[v] {
                    negative[to] = true;
                }
            }
        }
    }

    return (dist, negative);
}

fn main() {
    let mut edges = vec![Vec::new(); 6];
    edges[0].push((1, 5));
    edges[0].push((2, 4));
    edges[1].push((2, -2));
    edges[2].push((3, 2));
    edges[2].push((4, 1));
    edges[2].push((5, 4));
    edges[4].push((5, 4));
    let result = shortest_path(&edges, 0);
    println!("{:?}", result);
}
