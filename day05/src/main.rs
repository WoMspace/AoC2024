use std::cmp::Ordering;
use shared::PrettyPrint;
use std::collections::HashMap;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 05");

    let timer = std::time::Instant::now();
    let (part1, page_rules, unordered_updates) = part1(&input);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    let part2 = part2(unordered_updates, page_rules);
    let part2_time = timer.elapsed();

    
    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
    debug_assert!(part2 < 6489);
}

fn part1(input: &String) -> (usize, HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut phase = 1;
    let mut page_rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut unordered_updates: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        if phase == 1 {
            if line.is_empty() { phase += 1; continue; }
            let (first, second) = line.split_once('|').unwrap();
            let first: usize = first.parse().unwrap();
            let second: usize = second.parse().unwrap();
            
            match page_rules.get_mut(&first) {
                Some(v) => v.push(second),
                None => { let _ = page_rules.insert(first, vec![second]); }
            }
        } else if phase == 2 {
            if line.is_empty() { break }
            updates.push(line.split(',').map(|s| s.parse().unwrap()).collect())
        }
    }
    let mut sum = 0;
    'updates: for update in updates {
        let mut pages_before = update.clone();
        'pages: for page in update.iter().rev() {
            pages_before.pop();
            match page_rules.get(&page) {
                Some(rules) => {
                    for page_before in &pages_before {
                        // if true, `update` is not correctly ordered
                        if rules.contains(page_before) {
                            // println!("page {} of {:?} not ordered by rule {} of rules {:?}", page, update, page_before, rules);
                            unordered_updates.push(update);
                            continue 'updates 
                        }
                    }
                },
                None => continue 'pages
            }
        }
        // if we get here, `update` is correctly ordered
        let middle = update.len() / 2;
        sum += update[middle];
        // println!("Adding {} of {:?} to sum", update[middle], update);
    }

    (sum, page_rules, unordered_updates)
}

fn part2(unordered_updates: Vec<Vec<usize>>, page_rules: HashMap<usize, Vec<usize>>) -> usize {
    let mut sum = 0;
    for mut update in unordered_updates {
        update.sort_by(|a, b| {
            match page_rules.get(a) {
                Some(v) => {
                    if v.contains(b) {
                        Ordering::Less
                    }
                    else if let Some(v2) = page_rules.get(b) {
                        if v2.contains(a) {
                            Ordering::Greater
                        } else { Ordering::Less }
                    } else { Ordering::Equal }
                },
                None => Ordering::Equal
            }
        });
        let middle = update.len() / 2;
        sum += update[middle];
    }
    sum
}