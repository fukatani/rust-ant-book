fn main() {
    // left top is (0, 0)
    // right bottom is (maze[0].len() - 1, maze.len() - 1)
    // (x, y) is maze[y][x]
    let mut maze: Vec<&str> = Vec::new();

    maze.push("#S######.#");
    maze.push("......#..#");
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
                start_x = j;
                start_y = i;
            } else if maze[i][j] == 'G' {
                goal_x = j;
                goal_y = i;
            }
        }
    }

    let mut que = std::collections::VecDeque::new();
    que.push_back((start_x, start_y));
    let mut d = vec![vec![std::u64::MAX; maze[0].len()]; maze.len()];
    d[start_y][start_x] = 0;

    while let Some((cur_x, cur_y)) = que.pop_front() {
        if cur_x == goal_x && cur_y == goal_y {
            break;
        }
        if cur_x > 0 && maze[cur_y][cur_x - 1] != '#' && d[cur_y][cur_x - 1] == std::u64::MAX {
            d[cur_y][cur_x - 1] = d[cur_y][cur_x] + 1;
            que.push_back((cur_x - 1, cur_y));
        }
        if cur_x < maze[0].len() - 1 && maze[cur_y][cur_x + 1] != '#' && d[cur_y][cur_x + 1] == std::u64::MAX {
            d[cur_y][cur_x + 1] = d[cur_y][cur_x] + 1;
            que.push_back((cur_x + 1, cur_y));
        }
        if cur_y > 0 && maze[cur_y - 1][cur_x] != '#' && d[cur_y - 1][cur_x] == std::u64::MAX {
            d[cur_y - 1][cur_x] = d[cur_y][cur_x] + 1;
            que.push_back((cur_x, cur_y - 1));
        }
        if cur_y < maze.len() - 1 && maze[cur_y + 1][cur_x] != '#' && d[cur_y + 1][cur_x] == std::u64::MAX {
            d[cur_y + 1][cur_x] = d[cur_y][cur_x] + 1;
            que.push_back((cur_x, cur_y + 1));
        }
    }
    println!("{:?}", d[goal_y][goal_x]);
}
