use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let path: Vec<char> = input.lines().next().unwrap().chars().collect();
    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect();
    let mut step: u64 = 0;

    let mut current_pos: &str = "AAA";

    loop {
        for c in &path {
            step += 1;

            if *c == 'R' {
                current_pos = map.get(current_pos).unwrap().1;
            } else if *c == 'L' {
                current_pos = map.get(current_pos).unwrap().0;
            }

            if current_pos == "ZZZ" {
                return Some(step);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let path: Vec<char> = input.lines().next().unwrap().chars().collect();
    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect();

    let ends_with_a: Vec<&str> = map
        .clone()
        .iter()
        .filter(|x| x.0.chars().nth(2).unwrap() == 'A')
        .map(|x| *x.0)
        .collect();

    let mut steps_vec: Vec<u64> = Vec::new();

    for pos in ends_with_a {
        let mut current_pos = pos;
        let mut step: u64 = 0;
        let mut found_dest = false;
        while !found_dest {
            for c in &path {
                step += 1;

                if *c == 'R' {
                    current_pos = map.get(current_pos).unwrap().1;
                } else if *c == 'L' {
                    current_pos = map.get(current_pos).unwrap().0;
                }

                if current_pos.chars().nth(2).unwrap() == 'Z' {
                    steps_vec.push(step);
                    found_dest = true;
                    break;
                }
            }
        }
    }
    Some(steps_vec.iter().fold(1, |acc, x| lcm(acc, *x)))
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
