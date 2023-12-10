advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = pad_grid(input);

    let s_pos = find_s(&grid);
    let mut prev_pos = s_pos;
    let mut curr_pos = s_pos;

    let mut next_pos = find_next(prev_pos, curr_pos, &grid);
    let mut dist: u32 = 0;

    while next_pos != s_pos {
        prev_pos = curr_pos;
        curr_pos = next_pos;
        next_pos = find_next(prev_pos, curr_pos, &grid);
        dist += 1;
    }

    Some((dist + 1) / 2)
}

fn pad_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let mut row = Vec::new();

        row.push('.');
        row.extend(line.chars());
        row.push('.');

        if i == 0 {
            grid.push((0..line.len() + 2).map(|_| '.').collect());
        }

        grid.push(row);

        if i == lines.len() - 1 {
            grid.push((0..line.len() + 2).map(|_| '.').collect());
        }
    }
    grid
}

fn find_s(grid: &[Vec<char>]) -> (usize, usize) {
    let mut s_pos = (0usize, 0usize);
    let mut found_s = false;
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c == &'S' {
                s_pos = (row, col);
                found_s = true;
                break;
            }
        }
        if found_s {
            break;
        }
    }
    s_pos
}

fn find_next(
    prev_pos: (usize, usize),
    curr_pos: (usize, usize),
    grid: &[Vec<char>],
) -> (usize, usize) {
    let c = grid[curr_pos.0][curr_pos.1];

    let connect_top = ['|', '7', 'F'];
    let connect_right = ['-', 'J', '7'];
    let connect_left = ['-', 'F', 'L'];

    match c {
        '|' => (curr_pos.0 * 2 - prev_pos.0, curr_pos.1),
        '-' => (curr_pos.0, curr_pos.1 * 2 - prev_pos.1),
        'L' => {
            if curr_pos.0 == prev_pos.0 {
                return (curr_pos.0 - 1, curr_pos.1);
            }
            (curr_pos.0, curr_pos.1 + 1)
        }
        'J' => {
            if curr_pos.0 == prev_pos.0 {
                return (curr_pos.0 - 1, curr_pos.1);
            }
            (curr_pos.0, curr_pos.1 - 1)
        }
        '7' => {
            if curr_pos.0 == prev_pos.0 {
                return (curr_pos.0 + 1, curr_pos.1);
            }
            (curr_pos.0, curr_pos.1 - 1)
        }
        'F' => {
            if curr_pos.0 == prev_pos.0 {
                return (curr_pos.0 + 1, curr_pos.1);
            }
            (curr_pos.0, curr_pos.1 + 1)
        }
        'S' => {
            if connect_top.contains(&grid[curr_pos.0 - 1][curr_pos.1]) {
                (curr_pos.0 - 1, curr_pos.1)
            } else if connect_right.contains(&grid[curr_pos.0][curr_pos.1 + 1]) {
                return (curr_pos.0, curr_pos.1 + 1);
            } else if connect_left.contains(&grid[curr_pos.0][curr_pos.1 - 1]) {
                return (curr_pos.0, curr_pos.1 - 1);
            } else {
                return (curr_pos.0 + 1, curr_pos.1);
            }
        }
        _ => (0usize, 0usize),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = pad_grid(input);
    let s_pos = find_s(&grid);

    let mut path: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    let mut prev_pos = s_pos;
    let mut curr_pos = s_pos;
    let mut next_pos = find_next(prev_pos, curr_pos, &grid);
    let first_step = next_pos;

    while next_pos != s_pos {
        path[curr_pos.0][curr_pos.1] = true;
        prev_pos = curr_pos;
        curr_pos = next_pos;
        next_pos = find_next(prev_pos, curr_pos, &grid);
    }
    path[curr_pos.0][curr_pos.1] = true;

    let last_step = curr_pos;

    grid[s_pos.0][s_pos.1] = define_s(&s_pos, &first_step, &last_step);

    let mut total = 0;

    let verts: Vec<char> = vec!['L', '|', 'J'];

    for (row, line) in path.iter().enumerate() {
        for col in 1..line.len() {
            if !path[row][col] {
                let mut parity = 0;
                for i in 0..col {
                    if path[row][i] && verts.contains(&grid[row][i]) {
                        parity += 1;
                    }
                }

                if parity % 2 == 1 {
                    total += 1;
                }
            }
        }
    }
    Some(total)
}

fn define_s(
    s_pos: &(usize, usize),
    first_step: &(usize, usize),
    last_step: &(usize, usize),
) -> char {
    let top: bool = first_step.0 < s_pos.0 || last_step.0 < s_pos.0;
    let bottom: bool = first_step.0 > s_pos.0 || last_step.0 > s_pos.0;
    let right: bool = first_step.1 > s_pos.1 || last_step.1 > s_pos.1;
    let left: bool = first_step.1 < s_pos.1 || last_step.1 < s_pos.1;

    if top && bottom {
        '|'
    } else if left && right {
        return '-';
    } else if top && right {
        return 'L';
    } else if top && left {
        return 'J';
    } else if left && bottom {
        return '7';
    } else if right && bottom {
        return 'F';
    } else {
        return '.';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }
}
