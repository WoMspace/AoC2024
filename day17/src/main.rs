use shared::PrettyPrint;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 17");

    let timer = std::time::Instant::now();
    let part1 = part1(&input);
    let part1_time = timer.elapsed();
    // let timer = std::time::Instant::now();
    // let part2 = part2(limits, antennas);
    // let part2_time = timer.elapsed();

    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    // println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

fn part1(input: &str) -> String {
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut memory: Vec<u8> = Vec::new();
    for line in input.lines() {
        if line.contains("Register A") {
            let (_, value) = line.split_once(": ").unwrap();
            register_a = value.parse().unwrap()
        } else if line.contains("Register B") {
            let (_, value) = line.split_once(": ").unwrap();
            register_b = value.parse().unwrap()
        } else if line.contains("Register C") {
            let (_, value) = line.split_once(": ").unwrap();
            register_c = value.parse().unwrap()
        } else if line.contains("Program") {
            let (_, program) = line.split_once(": ").unwrap();
            memory = program.split(',').map(|s| s.parse().unwrap()).collect();
        }
    }
    let mut program_counter = 0;
    let mut output: String = String::new();
    
    'cycle: loop {
        let instruction = memory[program_counter];
        let literal_operand = memory[program_counter + 1];
        let combo_operand = match literal_operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => literal_operand as usize
        };
        match instruction {
            0 => { // adv: reg[A] = reg[A] / (2^combo_op)
                let numerator = register_a;
                let denominator = 2usize.pow(combo_operand as u32);
                register_a = numerator / denominator
            }
            1 => { // bxl: reg[B] XOR lit_op
                register_b = register_b ^ literal_operand as usize;
            }
            2 => { // bst: reg[B] % 8
                register_b = combo_operand % 8
            }
            3 => { // jnz: jump to lit_op if reg[A] not zero
                if register_a != 0 {
                    program_counter = literal_operand as usize;
                    // don't increment PC
                    continue 'cycle
                }
            }
            4 => { // bxc: reg[B] = reg[B] XOR reg[C]
                register_b ^= register_c
            }
            5 => { // out: print combo_op % 8
                // print!("{},", combo_operand % 8);
                output.push(char::from_digit((combo_operand % 8) as u32, 10).unwrap());
                output.push(',');
            }
            6 => { // bdv: reg[B] = reg[A] / (2^combo_op)
                register_b = register_a / 2usize.pow(combo_operand as  u32)
            }
            7 => { // cdv: reg[C] = reg[A] / (2^combo_op)
                register_c = register_a / 2usize.pow(combo_operand as  u32)
            }
            _ => {}
        }
        
        program_counter += 2;
        if program_counter + 1 > memory.len() { break 'cycle }
    }    
    
    output.trim_end_matches(',').to_string()
}