fn edit_distance(s1: Vec<char>, s2: Vec<char>) -> usize {
    if s1.is_empty() || s2.is_empty() {
        return std::cmp::max(s1.len(), s2.len());
    }

    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 0..s1.len() + 1 {
        dp[i][0] = i
    }
    for j in 0..s2.len() + 1 {
        dp[0][j] = j
    }
    for i in 1..s1.len() + 1 {
        for j in 1..s2.len() + 1 {
            let insert_cost = dp[i - 1][j] + 1;
            let remove_cost = dp[i][j - 1] + 1;
            let replace_cost = if s1[i - 1] == s2[j - 1] {
                dp[i - 1][j - 1]
            } else {
                dp[i - 1][j - 1] + 1
            };
            dp[i][j] = *[insert_cost, remove_cost, replace_cost]
                .iter()
                .min()
                .unwrap();
        }
    }
    dp[s1.len()][s2.len()]
}

fn wrap_edit_distance(s1: String, s2: String) -> usize {
    edit_distance(
        s1.chars().collect::<Vec<_>>(),
        s2.chars().collect::<Vec<_>>(),
    )
}

fn main() {
    assert_eq!(
        wrap_edit_distance("yahoo".to_string(), "yahoo".to_string()),
        0
    );
    assert_eq!(wrap_edit_distance("yahoo".to_string(), "".to_string()), 5);
    assert_eq!(
        wrap_edit_distance("yho".to_string(), "yahoo".to_string()),
        2
    );
    assert_eq!(
        wrap_edit_distance("yahooooo".to_string(), "yahoo".to_string()),
        3
    );
    assert_eq!(
        wrap_edit_distance("yphpoz".to_string(), "yahoo".to_string()),
        3
    );
    assert_eq!(
        wrap_edit_distance("yfoo".to_string(), "yahoo".to_string()),
        2
    );
    assert_eq!(
        wrap_edit_distance("yahyahoo".to_string(), "yahooyahoo".to_string()),
        2
    )
}
