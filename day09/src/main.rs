use shared::PrettyPrint;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 09");

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
    // index is block id, contents is Some(file_id) or None for free space
    let mut disk: Vec<Option<usize>> = Vec::new();
    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            // file
            let file_id = i / 2;
            let file_size = c.to_digit(10).unwrap() as usize;
            disk.append(&mut vec![Some(file_id); file_size]);
        } else {
            // free space
            let empty_space = c.to_digit(10).unwrap() as usize;
            disk.append(&mut vec![None; empty_space]);
        }
    }
    
    // move file blocks from end to free space at beginning
    let range = 0..disk.len();
    for block_id in range.rev() {
        if let Some(file_id) = disk[block_id] {
            // find first free space at beginning of disk
            if let Some(free_space) = disk.iter().position(|b| b.is_none()) {
                if free_space > block_id {
                    break;
                }
                disk[block_id] = None;
                disk[free_space] = Some(file_id);
            }
        }
    }
    
    // calculate checksum
    let mut checksum = 0;
    for (block_id, file_id) in disk.iter().enumerate() {
        if let Some(file_id) = file_id {
            checksum += block_id * file_id;
        }
    }

    checksum
}