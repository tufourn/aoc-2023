advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().map(|line| process_line_1(line)).sum())
}

fn process_line_1(line: &str) -> i32 {
    let original: Vec<i32> = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let mut lists = vec![original];
    let mut index = 0;

    let mut all_zeroes = false;

    while !all_zeroes {
        all_zeroes = true;
        let mut new_list: Vec<i32> = Vec::new();
        for (i, num) in lists[index].iter().enumerate() {
            if &i + 1 < lists[index].len() {
                let dist = lists[index][i + 1] - num;
                if dist != 0 {
                    all_zeroes = false;
                }
                new_list.push(dist);
            }
        }
        lists.push(new_list);
        index += 1;
    }

    let mut to_push: i32 = 0;

    for i in (0..lists.len()).rev() {
        to_push += lists[i].last().unwrap();
    }

    to_push
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(input.lines().map(|line| process_line_2(line)).sum())
}

fn process_line_2(line: &str) -> i32 {
    let original: Vec<i32> = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let mut lists = vec![original];
    let mut index = 0;

    let mut all_zeroes = false;

    while !all_zeroes {
        all_zeroes = true;
        let mut new_list: Vec<i32> = Vec::new();
        for (i, num) in lists[index].iter().enumerate() {
            if &i + 1 < lists[index].len() {
                let dist = lists[index][i + 1] - num;
                if dist != 0 {
                    all_zeroes = false;
                }
                new_list.push(dist);
            }
        }
        lists.push(new_list);
        index += 1;
    }

    let mut to_push: i32 = 0;

    for i in (0..lists.len()).rev() {
        to_push = lists[i].first().unwrap() - to_push;
    }

    to_push
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
