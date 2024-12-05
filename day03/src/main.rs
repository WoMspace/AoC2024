use shared::PrettyPrint;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 03");
    
    let timer = std::time::Instant::now();
    let part1 = part1(&input);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    let part2 = part2(&input);
    let part2_time = timer.elapsed();
    
    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

fn part1(input: &String) -> isize {
    sum_muls(input.as_str())
}

fn part2(input: &String) -> isize {
    let mut sum = 0;
    for code in input.split("do()") {
        let good_code = code.split("don't()").next().unwrap_or("");
        sum += sum_muls(good_code);
    }
    
    sum
}

fn sum_muls(input: &str) -> isize {
    let exp = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let sum = exp.find_iter(input)
        .fold(0, |acc, r| {
            acc + r.as_str().strip_prefix("mul(").unwrap()
                .strip_suffix(')').unwrap()
                .split(',')
                .map(|x| x.parse::<isize>().unwrap())
                .reduce(|acc, i| acc * i).unwrap()
        });
    sum
}