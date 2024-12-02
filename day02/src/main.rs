use shared::{PrettyPrint};

fn main() {
    let total_timer = std::time::Instant::now();
    let input = shared::get_input();
    
    println!("Advent of Code | Day 02");
    
    let timer = std::time::Instant::now();
    let (part1, reports) = part1(&input);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    let part2 = part2(reports);
    let part2_time = timer.elapsed();
    let total_time = total_timer.elapsed();
    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
    println!("Total runtime: {}", total_time.fmt_pretty())
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

fn test_report(report: &[isize]) -> bool {
    let mut lastlevel = report[0];
    let increasing = report[0] - report.last().unwrap() < 0;
    for level in &report[1..] {
        let change = if increasing { level - lastlevel } else { lastlevel - level };
        if !(1..=3).contains(&change) {
            return false;
        }
        lastlevel = *level;
    }
    true
}