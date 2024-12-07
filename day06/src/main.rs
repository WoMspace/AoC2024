use shared::PrettyPrint;
use crate::Direction::*;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 03");

    let timer = std::time::Instant::now();
    let part1 = part1(&input);
    let part1_time = timer.elapsed();
    // let timer = std::time::Instant::now();
    // let part2 = part2(&input);
    // let part2_time = timer.elapsed();

    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    // println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

enum Direction {
    Up, Right, Down, Left
}

struct Guard {
    x: isize,
    y: isize,
    direction: Direction
}

fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    let mut guard = Guard {
        x: 0,
        y: 0,
        direction: Up
    };
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() { continue }
        
        let mut chars: Vec<char> = line.chars().collect();
        if chars.contains(&'^') {
            let guard_pos = chars.iter().position(|c| c == &'^').unwrap() as isize;
            guard.x = guard_pos;
            guard.y = y as isize;
            guard.direction = Up;
            chars[guard_pos as usize] = 'X';
        }
        grid[y] = chars;
    }
    
    'move_guard: loop {
        let check_cell = match guard.direction {
            Up => (guard.x, guard.y - 1),
            Down => (guard.x, guard.y + 1),
            Left => (guard.x - 1, guard.y),
            Right => (guard.x + 1, guard.y),
        };
        if check_cell.0 >= 0 && check_cell.0 < width as isize && check_cell.1 >= 0 && check_cell.1 < height as isize {
            let cell = grid[check_cell.1 as usize][check_cell.0 as usize];
            if cell == '#' {
                // turn right
                guard.direction = match guard.direction {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up
                }
            } else {
                // move forward
                match guard.direction {
                    Up => guard.y -= 1,
                    Down => guard.y += 1,
                    Left => guard.x -= 1,
                    Right => guard.x += 1
                }
                grid[guard.y as usize][guard.x as usize] = 'X';
            }
        } else {
            // still move forward
            match guard.direction {
                Up => guard.y -= 1,
                Down => guard.y += 1,
                Left => guard.x -= 1,
                Right => guard.x += 1
            }
            if guard.x < 0 || guard.x >= width as isize || guard.y < 0 || guard.y >= height as isize {
                break 'move_guard;
            }
            grid[guard.y as usize][guard.x as usize] = 'X';
        }
    }
    
    let mut sum = 0;
    for row in &grid {
        for c in row {
            if *c == 'X' { sum += 1; }
        }
    }
    
    sum    
}