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
      // 2nnn
      (2, n1, n2, n3) => {
        self.next_instruction();
        self.stack.push(self.program_counter);
        self.set_program_counter((n1, n2, n3).into_instruction_value())
      }
      // 3xkk
      (3, x, k1, k2) => {
        let index = usize::from(x);
        let vx = self.vx_register[index];

        if u16::from(vx) == (k1, k2).into_instruction_value() {
          self.skip_next_instruction()
        } else {
          self.next_instruction()
        }
      }
      // 6xkk
      (6, x, k1, k2) => {
        self.vx_register[usize::from(x)] = (k1, k2).into_instruction_value() as u8;
        self.next_instruction();
      }
      // Annn
      (0xA, n1, n2, n3) => {
        self.i_register = (n1, n2, n3).into_instruction_value();
        self.next_instruction();
      }
      // 7xkk
      (7, x, k1, k2) => {
        let index = usize::from(x);

        self.vx_register[index] += (k1, k2).into_instruction_value() as u8;

        self.next_instruction();
      }
      //8xy0
      (8, x, y, 0) => {
        self.vx_register[usize::from(x)] = self.vx_register[usize::from(y)];
        self.next_instruction();
      }
      // Dxyn
      (0xD, x, y, n) => {
        self.vx_register[0xF] = 0;

        let vx = usize::from(self.vx_register[usize::from(x)]);
        let vy = usize::from(self.vx_register[usize::from(y)]);
        let height = usize::from(n);
        let i = self.i_register;

        let has_collided = display.draw(vx, vy, height, i, memory);

        self.vx_register[0xF] = u8::from(has_collided);
        self.next_instruction();
      }
      // ExA1
      (0xE, x, 0xA, 1) => {
        //println!("Todo");
        println!("{:#X}", 12);
      }
      // Fx1E
      (0xF, x, 1, 0xE) => {
        let index = usize::from(x);

        let (sum, overflow) = self
          .i_register
          .overflowing_add(u16::from(self.vx_register[index]));

        self.i_register = sum;
        self.vx_register[0xF] = u8::from(overflow);

        self.next_instruction();
      }
      _ => panic!("undefined instruction {:#x}", instruction.get_value()),
    }

    // self.set_previously_instruction();
  }

  fn set_program_counter(&mut self, next_instruction: u16) {
    self.program_counter = next_instruction
  }

  fn next_instruction(&mut self) {
    self.program_counter += 2;
  }

  fn skip_next_instruction(&mut self) {
    self.program_counter += 4;
  }
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
