advent_of_code::solution!(2);

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            process_line_1(line)
        })
        .sum();

    Some(result)
}

fn process_line_1(line: &str) -> u32 {
    let mut it = line
        .split_whitespace()
        .map(|s| {
            s.trim_matches(|c| c == ',' || c == ':' || c == ';')
        });

    let _ = it.next();
    let mut result = it.next().expect("should be the game number").parse::<u32>().unwrap();

    while let Some(s) = it.next() {
        let num = s.parse::<u32>().unwrap();
        if let Some(color) = it.next() {
            if color == "red" && num > RED {
                result = 0;
            } else if color == "green" && num > GREEN{
                result = 0;
            } else if color == "blue" && num > BLUE{
                result = 0;
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            process_line_2(line)
        })
        .sum();

    Some(result)
}

fn process_line_2(line: &str) -> u32 {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let mut parts = line.split(": ");
    let _ = parts.next();

    if let Some(sets) = parts.next() {
        let mut it = sets.split(";").map(|set| set.trim());
        while let Some(set) = it.next() {
            let mut colors = set.split(", ");
            while let Some(color) = colors.next() {
                let mut parts = color.split_whitespace();
                if let Some(num_str) = parts.next() {
                    if let Ok(num) = num_str.parse::<u32>() {
                        if let Some(col) = parts.next() {
                            if col == "red" && num > red {
                                red = num;
                            } else if col == "green" && num > green {
                                green = num;
                            } else if col == "blue" && num > blue {
                                blue = num;
                            }
                        }
                    }
                }
            }
        }
    }
    red * green * blue
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}