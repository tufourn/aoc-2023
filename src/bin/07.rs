use std::collections::HashMap;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<(Strength, Vec<u32>, u32)> = Vec::new();

    for line in input.lines() {
        hands.push(process_hand_1(line));
    }

    hands.sort_by(|a, b| {
        let cmp_first = a.0.cmp(&b.0);

        if cmp_first == std::cmp::Ordering::Equal {
            let mut index = 0;
            loop {
                let cmp_second = a.1[index].cmp(&b.1[index]);
                if cmp_second != std::cmp::Ordering::Equal {
                    return cmp_second;
                }
                index += 1;
            }
        } else {
            cmp_first
        }
    });

    let mut result = 0;

    for (index, hand) in hands.iter().enumerate() {
        result += hand.2 * (index + 1) as u32;
    }

    Some(result)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Strength {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn process_hand_1(hand: &str) -> (Strength, Vec<u32>, u32) {
    let bid = hand
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let cards: Vec<u32> = hand
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        })
        .collect();

    (get_strength_1(&cards), cards, bid)
}

fn get_strength_1(cards: &[u32]) -> Strength {
    let cards_map: HashMap<u32, u32> = cards.iter().fold(HashMap::new(), |mut map, card| {
        map.entry(*card).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });

    if cards_map.len() == 1 {
        return Strength::FiveOfAKind;
    }
    if cards_map.len() == 2 {
        let first_card_freq = cards_map.iter().next().unwrap().1;
        if *first_card_freq == 1 || *first_card_freq == 4 {
            return Strength::FourOfAKind;
        }
        return Strength::FullHouse;
    }
    if cards_map.len() == 3 {
        let max_freq = cards_map.values().max().unwrap();
        if *max_freq == 3 {
            return Strength::ThreeOfAKind;
        }
        return Strength::TwoPair;
    }
    if cards_map.len() == 4 {
        return Strength::OnePair;
    }
    Strength::High
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<(Strength, Vec<u32>, u32)> = Vec::new();

    for line in input.lines() {
        hands.push(process_hand_2(line));
    }

    hands.sort_by(|a, b| {
        let cmp_first = a.0.cmp(&b.0);

        if cmp_first == std::cmp::Ordering::Equal {
            let mut index = 0;
            loop {
                let cmp_second = a.1[index].cmp(&b.1[index]);
                if cmp_second != std::cmp::Ordering::Equal {
                    return cmp_second;
                }
                index += 1;
            }
        } else {
            cmp_first
        }
    });

    let mut result = 0;

    for (index, hand) in hands.iter().enumerate() {
        result += hand.2 * (index + 1) as u32;
    }

    Some(result)
}

fn process_hand_2(hand: &str) -> (Strength, Vec<u32>, u32) {
    let bid = hand
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let cards: Vec<u32> = hand
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        })
        .collect();

    (get_strength_2(&cards), cards, bid)
}

fn get_strength_2(cards: &[u32]) -> Strength {
    let mut cards_map: HashMap<u32, u32> = cards.iter().fold(HashMap::new(), |mut map, card| {
        map.entry(*card).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });

    if cards_map.contains_key(&0) && cards_map.len() > 1 {
        let joker_freq = cards_map.remove(&0).unwrap();
        if let Some((max_key, _)) = cards_map.clone().iter().max_by_key(|&(_, val)| val) {
            *cards_map.get_mut(max_key).unwrap() += joker_freq;
        }
    }

    if cards_map.len() == 1 {
        return Strength::FiveOfAKind;
    }
    if cards_map.len() == 2 {
        let first_card_freq = cards_map.iter().next().unwrap().1;
        if *first_card_freq == 1 || *first_card_freq == 4 {
            return Strength::FourOfAKind;
        }
        return Strength::FullHouse;
    }
    if cards_map.len() == 3 {
        let max_freq = cards_map.values().max().unwrap();
        if *max_freq == 3 {
            return Strength::ThreeOfAKind;
        }
        return Strength::TwoPair;
    }
    if cards_map.len() == 4 {
        return Strength::OnePair;
    }
    Strength::High
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
