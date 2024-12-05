use shared::PrettyPrint;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 04");

    let timer = std::time::Instant::now();
    let part1 = part1(&input);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    // let part2 = part2(&input);
    let part2_time = timer.elapsed();

    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    // println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

fn part1(input: &String) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut word_search = vec![vec!['0'; width]; height];
    let mut found_words = vec![vec![' '; width]; height];
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        // find forwards and backwards instances
        // sum += line.matches("XMAS").count();
        // sum += line.matches("SAMX").count();
        
        // make into 2D array
        word_search[i] = line.chars().collect();
    }
    
    // search horizontally (which could be done earlier but for visualisation's sake)
    for y in 0..height {
        for x in 0..(width - 3) {
            let sample = [word_search[x][y], word_search[x+1][y], word_search[x+2][y], word_search[x+3][y]];
            if sample == ['X', 'M', 'A', 'S'] || sample == ['S', 'A', 'M', 'X'] {
                sum += 1;
                found_words[x][y] = sample[0];
                found_words[x+1][y] = sample[1];
                found_words[x+2][y] = sample[2];
                found_words[x+3][y] = sample[3];
            }
        }
    }
    
    // search vertically
    for y in 0..(height - 3) {
        for x in 0..width {
            let sample = [word_search[x][y], word_search[x][y+1], word_search[x][y+2], word_search[x][y+3]];
            if sample == ['X', 'M', 'A', 'S'] || sample == ['S', 'A', 'M', 'X'] {
                sum += 1;
                found_words[x][y] = sample[0];
                found_words[x][y+1] = sample[1];
                found_words[x][y+2] = sample[2];
                found_words[x][y+3] = sample[3];
            }
        }
    }
    
    // search diagonally 💀
    for y in 0..(height - 3) {
        for x in 0..(width - 3) {
            let sample = [word_search[x][y], word_search[x+1][y+1], word_search[x+2][y+2], word_search[x+3][y+3]];
            if sample == ['X', 'M', 'A', 'S'] || sample == ['S', 'A', 'M', 'X'] {
                sum += 1;
                found_words[x][y] = sample[0];
                found_words[x+1][y+1] = sample[1];
                found_words[x+2][y+2] = sample[2];
                found_words[x+3][y+3] = sample[3];
            }
            
            let sample = [word_search[x+3][y], word_search[x+2][y+1], word_search[x+1][y+2], word_search[x][y+3]];
            if sample == ['X', 'M', 'A', 'S'] || sample == ['S', 'A', 'M', 'X'] {
                sum += 1;
                found_words[x+3][y] = sample[0];
                found_words[x+2][y+1] = sample[1];
                found_words[x+1][y+2] = sample[2];
                found_words[x][y+3] = sample[3];
            }
            
        }
    }
    
    // print_2d_array(&found_words);
    
    sum
}

fn print_2d_array(array: &Vec<Vec<char>>) {
    for row in array {
        for c in row {
            print!("{c} ");
        }
        println!();
    }
}