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
    
    println!("Advent of Code | Day 02");
    
    let timer = std::time::Instant::now();
    let (part1, reports) = part1(&file);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    let part2 = part2(reports);
    let part2_time = timer.elapsed();
    let total_time = total_timer.elapsed();
    println!("Part 1: {} in {}µs", part1, part1_time.as_micros());
    println!("Part 2: {} in {}µs", part2, part2_time.as_micros());
    println!("Total runtime: {}µs", total_time.as_micros())
}

fn part1(input: &String) -> (usize, Vec<Vec<isize>>) {
    let mut reports: Vec<Vec<isize>> = Vec::new();
    let mut safe_reports = 0;
    'report: for line in input.lines() {
        if line.is_empty() { continue; }
        let levels: Vec<isize> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        reports.push(levels.clone());
        
        let mut lastlevel = levels[0];
        let increasing = levels[0] - levels[1] < 0;
        for level in &levels[1..] {
            let change = if increasing {
                level - lastlevel
            } else {
                lastlevel - level
            };
            if !(1..=3).contains(&change) {
                continue 'report
            }
            lastlevel = *level;
        }
        safe_reports += 1;
    }

    (safe_reports, reports)
}

fn part2(reports: Vec<Vec<isize>>) -> usize {
    let mut safe_reports = 0;
    
    'reports: for report in reports {
        if test_report(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if test_report(&new_report) {
                    safe_reports += 1;
                    continue 'reports
                }
            }
        }
    }
    
    safe_reports
}

fn test_report(report: &Vec<isize>) -> bool {
    let mut lastlevel = report[0];
    let increasing = report[0] - report.last().unwrap() < 0;
    for (i, level) in report[1..].iter().enumerate() {
        let change = if increasing { level - lastlevel } else { lastlevel - level };
        if !(1..=3).contains(&change) {
            return false;
        }
        lastlevel = *level;
    }
    true
}