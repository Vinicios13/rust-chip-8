static FIRST_INSTRUCTION_ADDRESS: u16 = 0x200;

use super::Memory;

pub struct Cpu {
  previously_instruction: u16,
  program_counter: u16,
  stack: Vec<u16>,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      previously_instruction: 0,
      program_counter: FIRST_INSTRUCTION_ADDRESS,
      stack: vec![],
    }
  }

  pub fn run_instruction(&mut self, memory: &mut Memory) {
    let instruction = memory.get_instruction(self.program_counter);

    print!("{:#x}", instruction)
  }

  // fn next_instruction(&mut self) {
  //   self.program_counter += 2;
  // }

  // fn jump_next_instruction(&mut self) {
  //   self.program_counter += 4;
  // }
}
