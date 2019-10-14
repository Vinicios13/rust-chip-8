use super::Memory;

static FIRST_INSTRUCTION_ADDRESS: u16 = 0x200;

pub struct Cpu {
  previously_instruction: u16,
  program_counter: u16,
  stack: Vec<u16>,
}

trait IntoInstructionValue {
  fn into_instruction_value(self) -> u16;
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
    let formated_instruction = instruction.format_instruction();

    match formated_instruction {
      (1, x, y, z) => {
        self.set_program_counter((x, y, z).into_instruction_value());
      }
      _ => panic!("undefined instruction {:#x}", instruction.get_value()),
    }
  }

  fn set_program_counter(&mut self, next_instruction: u16) {
    self.program_counter = next_instruction
  }

  // fn next_instruction(&mut self) {
  //   self.program_counter += 2;
  // }

  // fn jump_next_instruction(&mut self) {
  //   self.program_counter += 4;
  // }
}

impl IntoInstructionValue for (u16, u16, u16) {
  fn into_instruction_value(self) -> u16 {
    let (x, y, z) = self;
    ((x << 8) | (y << 4)) | z
  }
}
