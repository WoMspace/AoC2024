use std::collections::{HashMap, HashSet};
use shared::PrettyPrint;
type Point = (isize, isize);

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 08");

    let timer = std::time::Instant::now();
    let (part1, antennas, limits) = part1(&input);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    let part2 = part2(limits, antennas);
    let part2_time = timer.elapsed();

    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

fn part1(input: &str) -> (usize, HashMap<char, Vec<Point>>, Point) {
    let limits = (input.trim().lines().count() as isize, input.lines().next().unwrap().trim().len() as isize);
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() { break }
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                match antennas.get_mut(&c) {
                    Some(v) => v.push((x as isize, y as isize)),
                    None => _ = antennas.insert(c, vec![(x as isize, y as isize)])
                }
            }
        }
    }
    
    let mut antinodes: HashSet<Point> = HashSet::new();
    for matching_antennas in antennas.values() {
        for antenna_a in matching_antennas {
            for antenna_b in matching_antennas {
                if antenna_a == antenna_b { continue; }
                let offset = (antenna_a.0 - antenna_b.0, antenna_a.1 - antenna_b.1);
                let antinode = (antenna_a.0 + offset.0, antenna_a.1 + offset.1);
                if (0..limits.0).contains(&antinode.0) && (0..limits.1).contains(&antinode.1) {
                    _ = antinodes.insert(antinode);
                }
            }
        }
    }

    (antinodes.len(), antennas, limits)
}

fn part2(limits: Point, antennas: HashMap<char, Vec<Point>>) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for matching_antennas in antennas.values() {
        for antenna_a in matching_antennas {
            for antenna_b in matching_antennas {
                if antenna_a == antenna_b { continue; }
                let offset = (antenna_a.0 - antenna_b.0, antenna_a.1 - antenna_b.1);
                'raymarch: for i in 0.. {
                    let antinode = (antenna_a.0 + offset.0 * i, antenna_a.1 + offset.1 * i);
                    if (0..limits.0).contains(&antinode.0) && (0..limits.1).contains(&antinode.1) {
                        _ = antinodes.insert(antinode);
                    } else {
                        break 'raymarch;
                    }
                }
            }
        }
    }
    
    antinodes.len()
}