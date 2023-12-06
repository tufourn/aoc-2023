advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let times = process_line(lines.next().unwrap());
    let dists = process_line(lines.next().unwrap());

    let mut index: usize = 0;
    let mut result = 1;

    while index < times.len() {
        result *= calc_num_options(times[index], dists[index]);
        index += 1;
    }

    Some(result)
}

fn calc_num_options(time: u64, dist: u64) -> u64 {
    let time: f64 = time as f64;
    let dist: f64 = dist as f64;

    let mut lower_bound: u64 = f64::ceil((time - f64::sqrt(time * time - 4.0 * dist)) / 2.0) as u64;
    if lower_bound * (time as u64 - lower_bound) == dist as u64 {
        lower_bound += 1;
    }

    let mut upper_bound: u64 =
        f64::floor((time + f64::sqrt(time * time - 4.0 * dist)) / 2.0) as u64;
    if upper_bound * (time as u64 - upper_bound) == dist as u64 {
        upper_bound -= 1;
    }

    upper_bound - lower_bound + 1
}

fn process_line(line: &str) -> Vec<u64> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let time: u64 = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let dist: u64 = input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    Some(calc_num_options(time, dist))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
