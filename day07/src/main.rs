use shared::PrettyPrint;

fn main() {
    let input = shared::get_input();
    println!("Advent of Code | Day 07");

    let timer = std::time::Instant::now();
    let (part1, equations) = part1(&input);
    let part1_time = timer.elapsed();
    let timer = std::time::Instant::now();
    let part2 = part2(equations);
    let part2_time = timer.elapsed();

    println!("Part 1: {} in {}", part1, part1_time.fmt_pretty());
    println!("Part 2: {} in {}", part2, part2_time.fmt_pretty());
}

fn part1(input: &str) -> (usize, Vec<(usize, Vec<usize>)>) {
    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() { continue }
        let (solution, inputs) = line.split_once(':').unwrap();
        let inputs: Vec<usize> = inputs.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let solution: usize = solution.parse().unwrap();
    
        
        let max_permutations =  usize::pow(2, inputs.len() as u32);
        'solve: for permutation in 0..=max_permutations {
            let mut acc = inputs[0];
            for (i, operand) in inputs[1..].iter().enumerate() {
                acc = match extract_bit(permutation, i) {
                    false => acc + operand,
                    true => acc * operand
                }
            }
            if acc == solution {
                sum += acc;
                break 'solve
            }
        }
        equations.push((solution, inputs));
    }
    (sum, equations)
}

fn part2(equations: Vec<(usize, Vec<usize>)>) -> usize {
    let mut sum = 0;
    for (solution, equation) in equations {
        let mut ops = vec![Op::Add; equation.len() - 1];
        let max_permutations = 3usize.pow(ops.len() as u32);
        'solve: for _ in 0..max_permutations {
            let mut acc = equation[0];
            for (op, rhs) in ops.iter().zip(&equation[1..]) {
                acc = match op {
                    Op::Add => acc + *rhs,
                    Op::Mul => acc * *rhs,
                    Op::Concat => acc * 10usize.pow((*rhs).ilog10() + 1) + *rhs
                }
            }
            if acc == solution {
                sum += solution;
                break 'solve
            }
            increment_ops(&mut ops);
        }
    }
    sum
}

fn extract_bit(num: usize, i: usize) -> bool {
    (num >> i) & 1 == 1
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Op {
    Add, Mul, Concat
}

fn increment_ops(ops: &mut [Op]) {
    for i in (0..ops.len()).rev() {
        let op = &ops[i];
        match op {
            Op::Add => { ops[i] = Op::Mul; break; }
            Op::Mul => { ops[i] = Op::Concat; break; }
            Op::Concat => { ops[i] = Op::Add; continue; }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Op::*;
    use super::*;
    
    #[test]
    fn test_increment_op() {
        let mut ops = [Add, Add, Add];
        increment_ops(&mut ops);
        assert_eq!(ops, [Add, Add, Mul]);
        
        ops = [Add, Mul, Mul];
        increment_ops(&mut ops);
        assert_eq!(ops, [Add, Mul, Concat]);

        ops = [Add, Mul, Concat];
        increment_ops(&mut ops);
        assert_eq!(ops, [Add, Concat, Add]);

        ops = [Add, Concat, Concat];
        increment_ops(&mut ops);
        assert_eq!(ops, [Mul, Add, Add]);
    }
}