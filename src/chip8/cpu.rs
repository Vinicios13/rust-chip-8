use super::Display;
use super::Memory;

static FIRST_INSTRUCTION_ADDRESS: u16 = 0x200;

pub struct Cpu {
  previously_instruction: u16,
  program_counter: u16,
  stack: Vec<u16>,
  vx_register: [u8; 16],
  i_register: u16,
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
      vx_register: [0; 16],
      i_register: 0,
    }
  }

  pub fn run_instruction(&mut self, memory: &mut Memory, display: &mut Display) {
    let instruction = memory.get_instruction(self.program_counter);
    let formated_instruction = instruction.format_instruction();

    match formated_instruction {
      // 1nnn
      (1, n1, n2, n3) => {
        self.set_program_counter((n1, n2, n3).into_instruction_value());
      }
      // 6xkk
      (6, x, k1, k2) => {
        self.set_vx_value(usize::from(x), (k1, k2).into_instruction_value() as u8);
        self.next_instruction();
      }
      // Annn
      (0xA, n1, n2, n3) => {
        self.set_i_value((n1, n2, n3).into_instruction_value());
        self.next_instruction();
      }
      // Dxyn
      (0xD, x, y, n) => {
        self.set_vx_value(0xF, 0);

        let vx = usize::from(self.get_vx_value(usize::from(x)));
        let vy = usize::from(self.get_vx_value(usize::from(y)));
        let height = usize::from(n);
        let i = usize::from(self.get_i_value());

        display.draw(vx, vy, height, i, &mut memory);

        //self.next_instruction();
      }
      _ => panic!("undefined instruction {:#x}", instruction.get_value()),
    }

    // self.set_previously_instruction();
  }

  fn set_program_counter(&mut self, next_instruction: u16) {
    self.program_counter = next_instruction
  }

  fn set_vx_value(&mut self, index: usize, value: u8) {
    self.vx_register[index] = value;
  }

  fn get_vx_value(&mut self, index: usize) -> u8 {
    self.vx_register[index]
  }

  fn set_i_value(&mut self, value: u16) {
    self.i_register = value;
  }

  fn get_i_value(&self) -> u16 {
    self.i_register
  }

  fn next_instruction(&mut self) {
    self.program_counter += 2;
  }

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

impl IntoInstructionValue for (u16, u16) {
  fn into_instruction_value(self) -> u16 {
    let (x, y) = self;
    (x << 4) | y
  }
}