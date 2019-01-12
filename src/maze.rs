fn main() {
    let mut maze: Vec<&str> = Vec::new();

    maze.push("#S######.#");
    maze.push("......#..#");
    maze.push(".#.##.##.#");
    maze.push(".#........");
    maze.push("##.##.####");
    maze.push("....#....#");
    maze.push(".#######.#");
    maze.push("....#....#");
    maze.push(".####.###.");
    maze.push("....#....G");

    let mut start_x = 0;
    let mut start_y = 0;
    let mut goal_x = 0;
    let mut goal_y = 0;

    let maze = (0..maze.len())
        .map(|i| maze[i].chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                start_x = i;
                start_y = j;
            } else if maze[i][j] == 'G' {
                goal_x = i;
                goal_y = j;
            }
        }
    }

    let mut que = std::collections::VecDeque::new();
    que.push_back((start_x, start_y));
    let mut d = vec![vec![std::u64::MAX; maze[0].len()]; maze.len()];
    d[start_x][start_y] = 0;

    while let Some((cur_x, cur_y)) = que.pop_front() {
        if cur_x == goal_x && cur_y == goal_y {
            break;
        }
        if cur_x > 0 && d[cur_x - 1][cur_y] == std::u64::MAX {
            d[cur_x - 1][cur_y] = d[cur_x][cur_y] + 1;
            que.push_back((cur_x - 1, cur_y));
        }
        if cur_x < maze[0].len() - 1 && d[cur_x + 1][cur_y] == std::u64::MAX {
            d[cur_x + 1][cur_y] = d[cur_x][cur_y] + 1;
            que.push_back((cur_x + 1, cur_y));
        }
        if cur_y > 0 && d[cur_x][cur_y - 1] == std::u64::MAX {
            d[cur_x][cur_y - 1] = d[cur_x][cur_y] + 1;
            que.push_back((cur_x, cur_y - 1));
        }
        if cur_y < maze.len() - 1 && d[cur_x][cur_y + 1] == std::u64::MAX {
            d[cur_x][cur_y + 1] = d[cur_x][cur_y] + 1;
            que.push_back((cur_x, cur_y + 1));
        }
    }
    println!("{:?}", d[goal_x][goal_y]);
}
