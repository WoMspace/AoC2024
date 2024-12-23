use shared::PrettyPrint;

type Coord = (isize, isize);

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 10");

    let timer = std::time::Instant::now();
    let part1 = part1(&input);
    let part1_time = timer.elapsed();
    // let timer = std::time::Instant::now();
    // let part2 = part2(limits, antennas);
    // let part2_time = timer.elapsed();

    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    // println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

fn part1(input: &str) -> usize {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut map: Vec<Vec<u8>> = vec![vec![0; height]; width];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[x][y] = c.to_digit(10).unwrap() as u8;
        }
    }
    
    for (x, column) in map.iter().enumerate() {
        for (y, height) in column.iter().enumerate() {
            if *height == 0 {
                
            }
        }
    }
    
    0
}

// fn recursive_