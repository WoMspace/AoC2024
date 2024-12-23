use shared::PrettyPrint;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 22");

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
    let sum = input.lines().filter_map(|s| s.parse::<usize>().ok()).fold(0,|acc: usize, mut secret| {
        for _ in 0..2000 {
            secret = predict_next_secret(secret);
        }
        acc + secret
    });
    
    sum
}

fn predict_next_secret(secret: usize) -> usize {
    let mut secret = secret ^ (secret * 64);
    secret = secret % 16777216;

    secret = secret ^ (secret / 32);
    secret = secret % 16777216;

    secret = secret ^ (secret * 2048);
    secret = secret % 16777216;
    
    secret
}