use std::collections::{HashMap, HashSet};
advent_of_code::solution!(3);

const POSITIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let lines: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        let mut num: u32 = 0;
        let mut is_engine: bool = false;

        for (j, c) in line.iter().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                num = num * 10 + digit;
                for (row_offset, col_offset) in POSITIONS.iter() {
                    let new_row = (i as i32 + row_offset).clamp(0, lines.len() as i32 - 1);
                    let new_col = (j as i32 + col_offset).clamp(0, line.len() as i32 - 1);

                    let sym = lines[new_row as usize][new_col as usize];

                    if !sym.is_digit(10) && sym != '.' {
                        is_engine = true;
                    }
                }
            } else {
                if num > 0 && is_engine {
                    result += num;
                    is_engine = false;
                }
                num = 0;
            }
        }
        if is_engine {
            result += num;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| (line.to_string() + ".").chars().collect())
        .collect();

    let row_num = lines.len();
    let col_num = lines[0].len();

    for r in 0..row_num {
        let mut num: u32 = 0;
        let mut gears_around_num: HashSet<(usize, usize)> = HashSet::new();

        for c in 0..col_num {
            if let Some(digit) = lines[r][c].to_digit(10) {
                num = num * 10 + digit;
                for (row_offset, col_offset) in POSITIONS.iter() {
                    let row = (r as i32 + row_offset).clamp(0, row_num as i32 - 1) as usize;
                    let col = (c as i32 + col_offset).clamp(0, col_num as i32 - 1) as usize;
                    if lines[row][col] == '*' {
                        gears_around_num.insert((row, col));
                        gears.entry((row, col)).or_insert_with(Vec::new);
                    }
                }
            } else {
                if num > 0 && !gears_around_num.is_empty() {
                    gears_around_num.iter().for_each(|&gear| {
                        gears.entry(gear).or_insert_with(Vec::new).push(num);
                    });
                    gears_around_num.clear();
                }
                num = 0;
            }
        }
    }

    Some(
        gears
            .into_values()
            .filter(|set| set.len() == 2)
            .map(|set| set.iter().product::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
