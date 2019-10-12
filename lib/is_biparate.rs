fn dfs(start: usize, graph: &Vec<Vec<usize>>, cur_color: i32, color: &mut Vec<i32>) -> bool {
    if color[start] != 0 {
        return cur_color == color[start];
    }
    color[start] = cur_color;
    for &to in graph[start].iter() {
        if !dfs(to, &graph, -1 * cur_color, color) {
            return false;
        }
    }
    true
}

fn is_biparate(graph: &Vec<Vec<usize>>) -> bool {
    let mut color = vec![0; graph.len()];
    for i in 0..graph.len() {
        if color[i] == 0 {
            if !dfs(i, graph, 1, &mut color) {
                return false;
            }
        }
    }
    true
}

fn make_graph(n: usize, nodes: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    for &(i, j) in nodes.iter() {
        graph[i].push(j);
        graph[j].push(i);
    }
    graph
}

fn main() {
    let nodes = vec![(0, 1), (1, 2), (2, 0)];
    assert_eq!(is_biparate(&make_graph(3, &nodes)), false);
    let nodes = vec![(0, 1), (0, 3), (2, 1), (2, 3)];
    assert_eq!(is_biparate(&make_graph(4, &nodes)), true);
}
