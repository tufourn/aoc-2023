use std::u32;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("should be a valid digit");

            match it.last() {
                Some(num) => format!("{first}{num}").parse::<u32>(),
                None => format!("{first}{first}").parse::<u32>(),
            }
            .expect("should be a valid number")
        })
        .sum::<u32>();

    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = input.lines().map(process_line).sum::<u32>();

    Some(output)
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };
        index += 1;
        result
    });

    let mut it = line_iter.filter_map(|character| character.to_digit(10));
    let first = it.next().expect("should be a valid digit");

    match it.last() {
        Some(num) => format!("{first}{num}").parse::<u32>(),
        None => format!("{first}{first}").parse::<u32>(),
    }
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}


