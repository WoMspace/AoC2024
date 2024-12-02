use shared::PrettyPrint;

fn main() {
    let total_timer = std::time::Instant::now();
    let input = shared::get_input();
    println!("Advent of Code | Day 01");
    
    let timer = std::time::Instant::now();
    let (left_column, right_column) = parse(input);
    let parse_time = timer.elapsed();
    
    let timer = std::time::Instant::now();
    let part1 = part1(&left_column, &right_column);
    let part1_time = timer.elapsed();
    
    let timer = std::time::Instant::now();
    let part2 = part2(&left_column, &right_column);
    let part2_time = timer.elapsed();
    
    let total_time = total_timer.elapsed();
    println!("Parsed input in {}", parse_time.fmt_pretty());
    println!("Part 1: {part1} in {}", part1_time.fmt_pretty());
    println!("Part 2: {part2} in {}", part2_time.fmt_pretty());
    println!("Total runtime: {}", total_time.fmt_pretty());
}

fn parse(input: String) -> (Vec<isize>, Vec<isize>) {
    let mut left_column: Vec<isize> = Vec::new();
    let mut right_column: Vec<isize> = Vec::new();
    for line in input.lines() {
        if line.is_empty() { continue }
        let nums: Vec<&str> = line.split_whitespace().collect();
        let left = nums[0].parse::<isize>().unwrap();
        let right = nums[1].parse::<isize>().unwrap();
        left_column.push(left);
        right_column.push(right);
    }
    (left_column, right_column)
}

fn part1(left_column: &Vec<isize>, right_column: &Vec<isize>) -> usize {
    let mut l_col = left_column.clone();
    let mut r_col = right_column.clone();
    l_col.sort();
    r_col.sort();
    debug_assert!(l_col.is_sorted());
    debug_assert!(r_col.is_sorted());
    
    let mut sum = 0;
    for i in 0..l_col.len() {
        sum += l_col[i].abs_diff(r_col[i]);
    }
    sum
}

fn part2(left_column: &[isize], right_column: &[isize]) -> isize {    
    let mut similarity = 0;
    for left in left_column {
        let mut count = 0;
        for right in right_column {
            if *right == *left { count += 1 }
        }
        similarity += left * count;
    }
    similarity
}