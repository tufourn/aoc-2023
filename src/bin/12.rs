use std::collections::HashMap;
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(process_line).sum())
}

fn process_line(line: &str) -> u64 {
    let blocks: Vec<u64> = line
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let dots = line.split_ascii_whitespace().next().unwrap().as_bytes();

    let mut dp = HashMap::new();

    fn f(dots: &[u8], blocks: &Vec<u64>, i: usize, bi: usize, current: u64, dp: &mut HashMap<(usize, usize, u64), u64>) -> u64 {
        let key = (i, bi, current);
        if dp.contains_key(&key) {
            return dp[&key];
        }
        if i == dots.len() {
            if (bi == blocks.len() && current == 0) || (bi == blocks.len() - 1 && blocks[bi] == current){
                return 1;
            } else {
                return 0;
            }
        }

        let mut ans: u64 = 0;
        for c in [b'.', b'#'] {
            if dots[i] == c || dots[i] == b'?' {
                if c == b'.' && current == 0 {
                    // continue with dot
                    ans += f(dots, blocks, i + 1, bi, 0, dp);
                } else if c == b'.' && current > 0 && bi < blocks.len() && blocks[bi] == current {
                    // end a block
                    ans += f(dots, blocks, i + 1, bi + 1, 0, dp)
                } else if c == b'#' {
                    // continue a block
                    ans += f(dots, blocks, i + 1, bi, current + 1, dp);
                }
            }
        }
        dp.entry(key).or_insert(ans);
        ans
    }

    f(dots, &blocks, 0, 0, 0, &mut dp)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| {
        let dots = line.split_ascii_whitespace().next().unwrap();
        let new_dots = std::iter::repeat(dots).take(5).collect::<Vec<_>>().join("?");
        let blocks = line.split_ascii_whitespace().nth(1).unwrap();
        let new_blocks = std::iter::repeat(blocks).take(5).collect::<Vec<_>>().join(",");
        let new_line = new_dots + " " + &new_blocks;
        process_line(&new_line)

    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
