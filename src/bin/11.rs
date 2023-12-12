advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let expanded_grid = expand(&grid);
    let mut stars: Vec<(usize, usize)> = Vec::new();
    for (row, line) in expanded_grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c == &'#' {
                stars.push((row, col));
            }
        }
    }
    let mut total = 0;
    for i in 0..(stars.len() - 1) {
        for j in i + 1..stars.len() {
            let dist = (i64::abs(stars[i].0 as i64 - stars[j].0 as i64)
                + i64::abs(stars[i].1 as i64 - stars[j].1 as i64)) as u64;
            total += dist
        }
    }
    Some(total)
}

fn expand(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    let mut row = 0;
    let mut col = 0;

    while row < grid.len() {
        let mut empty = true;
        for c in &grid[row] {
            if c != &'.' {
                empty = false;
                break;
            }
        }
        if empty {
            new_grid.push(vec!['.'; grid[row].len()]);
        }
        new_grid.push(grid[row].clone());
        row += 1;
    }

    while col < new_grid[0].len() {
        let mut empty = true;
        for row in &new_grid {
            if row[col] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            for row in &mut new_grid {
                row.insert(col, '.');
            }
            col += 1
        }
        col += 1;
    }
    new_grid
}

pub fn part_two(input: &str) -> Option<u64> {
    let multiplier = 1000000;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    let mut row = 0;
    let mut col = 0;

    while row < grid.len() {
        let mut empty = true;
        for c in &grid[row] {
            if c != &'.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_rows.push(row)
        }
        row += 1;
    }
    while col < grid[0].len() {
        let mut empty = true;
        for row in &grid {
            if row[col] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols.push(col);
        }
        col += 1;
    }

    let mut stars: Vec<(usize, usize)> = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if c == &'#' {
                stars.push((row, col));
            }
        }
    }

    let mut total = 0;

    for i in 0..(stars.len() - 1) {
        for j in i + 1..stars.len() {
            let mut to_add = 0;
            let mut lower;
            let mut higher;
            if stars[i].0 < stars[j].0 {
                lower = stars[i].0;
                higher = stars[j].0;
            } else {
                lower = stars[j].0;
                higher = stars[i].0;
            }

            for row in lower..higher {
                to_add += 1;
                if empty_rows.contains(&row) {
                    to_add += multiplier - 1
                }
            }

            if stars[i].1 < stars[j].1 {
                lower = stars[i].1;
                higher = stars[j].1;
            } else {
                lower = stars[j].1;
                higher = stars[i].1;
            }

            for col in lower..higher {
                to_add += 1;
                if empty_cols.contains(&col) {
                    to_add += multiplier - 1
                }
            }
            total += to_add;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
