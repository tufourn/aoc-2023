advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(process_card).sum::<u32>())
}

fn process_card(line: &str) -> u32 {
    let mut result: u32 = 0;

    let mut it = line
        .split(':')
        .nth(1)
        .expect("should be winning nums | your nums")
        .split('|');

    let winning_nums = it
        .next()
        .expect("should be the winning nums")
        .split(' ')
        .filter(|s| !s.is_empty())
        .flat_map(|n| n.parse::<u32>().ok());

    let your_nums: Vec<u32> = it
        .next()
        .expect("should be your nums")
        .split(' ')
        .filter(|s| !s.is_empty())
        .flat_map(|n| n.parse::<u32>().ok())
        .collect();

    winning_nums.for_each(|n| {
        if your_nums.contains(&n) {
            result = match result {
                0 => 1,
                _ => result * 2,
            }
        }
    });

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let it = input.lines();

    let mut counts = vec![1; it.clone().count()];

    for (index, line) in it.enumerate() {
        let mut matches = 0;

        let mut num_it = line
            .split(':')
            .nth(1)
            .expect("should be winning nums | your nums")
            .split('|');

        let winning_nums = num_it
            .next()
            .expect("should be the winning nums")
            .split(' ')
            .filter(|s| !s.is_empty())
            .flat_map(|n| n.parse::<u32>().ok());

        let your_nums: Vec<u32> = num_it
            .next()
            .expect("should be your nums")
            .split(' ')
            .filter(|s| !s.is_empty())
            .flat_map(|n| n.parse::<u32>().ok())
            .collect();

        winning_nums.for_each(|n| {
            if your_nums.contains(&n) {
                matches += 1;
            }
        });

        if matches > 0 {
            let mut next_index = index;
            for _ in 0..matches {
                next_index += 1;
                counts[next_index] += counts[index];
            }
        }
    }
    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
