fn main() {
    let total_timer = std::time::Instant::now();
    let path = match std::env::args().nth(1) {
        Some(s) => s,
        None => { 
            println!("Missing required argument: <input path>");
            std::process::exit(1)
        }
    };
    let file = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading file: {e}");
            std::process::exit(1)
        }
    };
    println!("Advent of Code | Day 01");


    let mut left_column: Vec<usize> = Vec::new();
    let mut right_column: Vec<usize> = Vec::new();
    for line in file.lines() {
        if line.is_empty() { continue }
        let nums: Vec<&str> = line.split_whitespace().collect();
        let left = nums[0].parse::<usize>().unwrap();
        let right = nums[1].parse::<usize>().unwrap();
        left_column.push(left);
        right_column.push(right);
    }
    
    let timer = std::time::Instant::now();
    let part1 = part1(&left_column, &right_column);
    let part1_time = timer.elapsed();
    println!("Part 1: {part1} in {}ns", part1_time.as_nanos());
    let timer = std::time::Instant::now();
    let part2 = part2(&left_column, &right_column);
    let part2_time = timer.elapsed();
    println!("Part 2: {part2} in {}µs", part2_time.as_micros());
    let total_time = total_timer.elapsed();
    println!("Total runtime: {}µs", total_time.as_micros())
}

fn part1(left_column: &[usize], right_column: &[usize]) -> usize {
    let l_col = left_column;
    let r_col = right_column;
    debug_assert!(left_column.is_sorted());
    debug_assert!(right_column.is_sorted());
    
    let mut sum = 0;
    for i in 0..l_col.len() {
        if l_col[i] > r_col[i] {
            sum += l_col[i] - r_col[i];
        } else {
            sum += r_col[i] - l_col[i];
        }
    }
    sum
}

fn part2(left_column: &[usize], right_column: &[usize]) -> usize {
    
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