fn topological_sort(edges: &Vec<(usize, usize)>, v: usize) -> Option<Vec<usize>> {
    let mut h = vec![0; v];
    let mut g = vec![Vec::new(); v];
    for &(s, t) in edges {
        g[s].push(t);
        h[t] += 1;
    }

    let mut st: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    for i in 0..v {
        if h[i] == 0 {
            st.push_back(i);
        }
    }

    let mut sorted_indexes = Vec::new();
    while !st.is_empty() {
        let i = st.pop_back().unwrap();
        sorted_indexes.push(i);
        for &j in g[i].iter() {
            h[j] -= 1;
            if h[j] == 0 {
                st.push_back(j);
            }
        }
    }
    if sorted_indexes.len() == v {
        Some(sorted_indexes)
    } else {
        None
    }
}

fn main() {
    let mut edges = Vec::new();
    edges.push((3, 0));
    edges.push((3, 2));
    edges.push((0, 1));
    edges.push((2, 4));
    edges.push((5, 0));
    edges.push((2, 1));
    edges.push((1, 3));
    let v = 6;
    let sorted_indexes = topological_sort(&edges, v);
    println!("sorted_indexes {:?}", sorted_indexes);
}
