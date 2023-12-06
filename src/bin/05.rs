use core::str::Lines;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines: Lines = input.lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    lines.next();
    lines.next();

    let seed_to_soil = get_bounds(&mut lines);
    let soil_to_fert = get_bounds(&mut lines);
    let fert_to_water = get_bounds(&mut lines);
    let water_to_light = get_bounds(&mut lines);
    let light_to_temp = get_bounds(&mut lines);
    let temp_to_humid = get_bounds(&mut lines);
    let humid_to_location = get_bounds(&mut lines);

    let mut min_location = u64::MAX;

    for seed in seeds {
        let soil = get_dest(seed, &seed_to_soil);
        let fert = get_dest(soil, &soil_to_fert);
        let water = get_dest(fert, &fert_to_water);
        let light = get_dest(water, &water_to_light);
        let temp = get_dest(light, &light_to_temp);
        let humid = get_dest(temp, &temp_to_humid);
        let location = get_dest(humid, &humid_to_location);

        if location < min_location {
            min_location = location;
        }
    }

    Some(min_location)
}

fn get_bounds(lines: &mut Lines) -> Vec<(u64, u64, u64)> {
    let mut result: Vec<(u64, u64, u64)> = Vec::new();
    let lines_nonempty: Vec<&str> = lines.take_while(|line| !line.is_empty()).collect();

    for line in lines_nonempty {
        if !line.is_empty() {
            let line_num: Vec<u64> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            result.push((line_num[1], line_num[0], line_num[2]));
        } else {
            break;
        }
    }

    let _ = lines.next();

    result.sort_by(|a, b| (a.0).cmp(&b.0));

    result
}

fn get_dest(src: u64, src_to_dst: &Vec<(u64, u64, u64)>) -> u64 {
    for bound in src_to_dst {
        if bound.0 <= src && src < bound.0 + bound.2 {
            return bound.1 + src - bound.0;
        }
    }
    src
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines: Lines = input.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap());

    let mut seed_bounds: Vec<(u64, u64)> = Vec::new();

    while let Some(base) = seeds.next() {
        seed_bounds.push((base, seeds.next().unwrap()));
    }

    seed_bounds.sort_by(|a, b| (a.0).cmp(&b.0));

    lines.next();
    lines.next();

    let seed_to_soil = get_bounds(&mut lines);
    let soil_to_fert = get_bounds(&mut lines);
    let fert_to_water = get_bounds(&mut lines);
    let water_to_light = get_bounds(&mut lines);
    let light_to_temp = get_bounds(&mut lines);
    let temp_to_humid = get_bounds(&mut lines);
    let humid_to_location = get_bounds(&mut lines);

    let mut location = 0;

    loop {
        let humid = get_src(location, &humid_to_location);
        let temp = get_src(humid, &temp_to_humid);
        let light = get_src(temp, &light_to_temp);
        let water = get_src(light, &water_to_light);
        let fert = get_src(water, &fert_to_water);
        let soil = get_src(fert, &soil_to_fert);
        let seed = get_src(soil, &seed_to_soil);

        for bound in &seed_bounds {
            if bound.0 <= seed && seed < bound.0 + bound.1 {
                return Some(location);
            }
        }

        location += 1;
    }
}

fn get_src(dst: u64, src_to_dst: &Vec<(u64, u64, u64)>) -> u64 {
    for bound in src_to_dst {
        if bound.1 <= dst && dst < bound.1 + bound.2 {
            return bound.0 + dst - bound.1;
        }
    }
    dst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
