fn dfs(i: usize, j: usize, field: &mut Vec<Vec<char>>) {
    let mut stack = std::collections::VecDeque::new();
    stack.push_back((i, j));
    while let Some((cur_x, cur_y)) = stack.pop_back() {
        field[cur_x][cur_y] = '.';
        if cur_x > 0 && field[cur_x - 1][cur_y] == 'W' {
            stack.push_back((cur_x - 1, cur_y));
        }
        if cur_x < field.len() - 1 && field[cur_x + 1][cur_y] == 'W' {
            stack.push_back((cur_x + 1, cur_y));
        }
        if cur_y > 0 && field[cur_x][cur_y - 1] == 'W' {
            stack.push_back((cur_x, cur_y - 1));
        }
        if cur_y < field[0].len() - 1 && field[cur_x][cur_y + 1] == 'W' {
            stack.push_back((cur_x, cur_y + 1));
        }

        if cur_x > 0 && cur_y > 0 && field[cur_x - 1][cur_y - 1] == 'W' {
            stack.push_back((cur_x - 1, cur_y - 1));
        }
        if cur_x > 0 && cur_y < field[0].len() - 1 && field[cur_x - 1][cur_y + 1] == 'W' {
            stack.push_back((cur_x - 1, cur_y + 1));
        }
        if cur_x < field.len() - 1 && cur_y > 0 && field[cur_x + 1][cur_y - 1] == 'W' {
            stack.push_back((cur_x + 1, cur_y - 1));
        }
        if cur_x < field.len() - 1 && cur_y < field[0].len() - 1 && field[cur_x + 1][cur_y + 1] == 'W' {
            stack.push_back((cur_x + 1, cur_y + 1));
        }
    }
}

fn main() {
    let mut field: Vec<&str> = Vec::new();

    field.push("W........WW.");
    field.push(".WWW.....WWW");
    field.push("....WW...WW.");
    field.push(".........WW.");
    field.push(".........W..");
    field.push("..W......W..");
    field.push(".W.W.....WW.");
    field.push("W.W.W.....W.");
    field.push(".W.W......W.");
    field.push("..W......W..");

    let mut field = (0..field.len())
        .map(|i| field[i].chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] == 'W' {
                dfs(i, j, &mut field);
                ans += 1;
            }
        }
    }
    println!("{:?}", ans);
}
