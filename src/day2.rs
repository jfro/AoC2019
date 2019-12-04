struct IntcodeState {
    memory: Vec<u32>,
    prog_index: usize,
}
impl IntcodeState {
    fn new(memory: &[u32]) -> Self {
        IntcodeState {
            memory: memory.to_vec(),
            prog_index: 0,
        }
    }
    fn add(&mut self, arg1: u32, arg2: u32, out: u32) {
        self.memory[out as usize] = self.memory[arg1 as usize] + self.memory[arg2 as usize];
    }
    fn mult(&mut self, arg1: u32, arg2: u32, out: u32) {
        self.memory[out as usize] = self.memory[arg1 as usize] * self.memory[arg2 as usize];
    }
    fn execute_cycle(&mut self) -> bool {
        let arg1 = self.memory[self.prog_index + 1];
        let arg2 = self.memory[self.prog_index + 2];
        let output = self.memory[self.prog_index + 3];
        match self.memory[self.prog_index] {
            1 => self.add(arg1, arg2, output),
            2 => self.mult(arg1, arg2, output),
            99 => return false,
            opcode @ _ => panic!("Invalid opcode: {}", opcode)
        }
        self.prog_index += 4;
        true
    }
    fn run_until_halt(&mut self) {
        while self.execute_cycle() {}
    }
    fn run_with_inputs(&mut self, arg1: u32, arg2: u32) -> u32 {
        self.memory[1] = arg1;
        self.memory[2] = arg2;
        self.run_until_halt();
        self.memory[0]
    }
}
#[aoc_generator(day2)]
fn gen_part1(input: &str) -> Vec<u32> {
    input.split(",").map(|d| d.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    let mut state = IntcodeState::new(input);
    state.memory[1] = 12;
    state.memory[2] = 2;
    state.run_until_halt();
    state.memory[0]
}

#[aoc(day2, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    let goal = 19690720;
    let input1_range = 0..=99;
    for noun in input1_range {
        let input2_range = 0..=99;
        for verb in input2_range {
            let mut state = IntcodeState::new(input);
            let answer = state.run_with_inputs(noun, verb);
            if answer == goal {
                return 100 * noun + verb;
            }
        }
    }
    0
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//}
