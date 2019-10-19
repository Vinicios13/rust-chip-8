extern crate rand;
use rand::prelude::*;

use super::Display;
use super::Keyboard;
use super::Memory;

static FIRST_INSTRUCTION_ADDRESS: u16 = 0x200;

pub struct Cpu {
  program_counter: u16,
  stack: Vec<u16>,
  vx_register: [u8; 16],
  i_register: u16,
  delay_timer: u8,
  sound_timer: u8,
}

trait IntoInstructionValue {
  fn into_instruction_value(self) -> u16;
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      program_counter: FIRST_INSTRUCTION_ADDRESS,
      stack: vec![],
      vx_register: [0; 16],
      i_register: 0,
      delay_timer: 0,
      sound_timer: 0,
    }
  }

  pub fn run_instruction(
    &mut self,
    memory: &mut Memory,
    display: &mut Display,
    keyboard: &Keyboard,
  ) {
    let instruction = memory.get_instruction(self.program_counter);
    let formated_instruction = instruction.format_instruction();

    match formated_instruction {
      // 00E0
      (0, 0, 0xE, 0) => {
        display.clear_gfx();
        self.next_instruction();
      }
      // 00EE
      (0, 0, 0xE, 0xE) => {
        self.program_counter = self.stack.pop().unwrap();
      }
      // 1nnn
      (1, n1, n2, n3) => {
        self.set_program_counter((n1, n2, n3).into_instruction_value());
      }
      // 2nnn
      (2, n1, n2, n3) => {
        self.stack.push(self.program_counter + 2);
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
      // 4xkk
      (4, x, k1, k2) => {
        let index = usize::from(x);
        let vx = self.vx_register[index];

        if u16::from(vx) != (k1, k2).into_instruction_value() {
          self.skip_next_instruction()
        } else {
          self.next_instruction()
        }
      }
      // 5xy0
      (5, x, y, 0) => {
        let vx = self.vx_register[usize::from(x)];
        let vy = self.vx_register[usize::from(y)];

        if vx == vy {
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
      // 7xkk
      (7, x, k1, k2) => {
        let index = usize::from(x);

        self.vx_register[index] =
          self.vx_register[index].wrapping_add((k1, k2).into_instruction_value() as u8);

        self.next_instruction();
      }
      //8xy0
      (8, x, y, 0) => {
        self.vx_register[usize::from(x)] = self.vx_register[usize::from(y)];
        self.next_instruction();
      }
      //8xy1
      (8, x, y, 1) => {
        self.vx_register[usize::from(x)] |= self.vx_register[usize::from(y)];
        self.next_instruction();
      }
      //8xy2
      (8, x, y, 2) => {
        self.vx_register[usize::from(x)] &= self.vx_register[usize::from(y)];
        self.next_instruction();
      }
      //8xy3
      (8, x, y, 3) => {
        self.vx_register[usize::from(x)] ^= self.vx_register[usize::from(y)];
        self.next_instruction();
      }
      //8xy4
      (8, x, y, 4) => {
        let x_index = usize::from(x);

        let (sum, overflow) =
          self.vx_register[x_index].overflowing_add(self.vx_register[usize::from(y)]);

        self.vx_register[x_index] = sum;
        self.vx_register[0xF] = u8::from(overflow);

        self.next_instruction();
      }
      // 8xy5
      (8, x, y, 5) => {
        let x_index = usize::from(x);

        let (diff, overflow) =
          self.vx_register[x_index].overflowing_sub(self.vx_register[usize::from(y)]);

        self.vx_register[x_index] = diff;
        self.vx_register[0xF] = u8::from(!overflow);
        self.next_instruction();
      }
      // 8xy6
      (8, x, _, 6) => {
        let x_index = usize::from(x);

        self.vx_register[0xF] = self.vx_register[usize::from(x)] & 0x1;
        self.vx_register[x_index] >>= 1;
        self.next_instruction();
      }
      // 8xy7
      (8, x, y, 7) => {
        let x_index = usize::from(x);

        let (diff, overflow) =
          self.vx_register[usize::from(y)].overflowing_sub(self.vx_register[x_index]);

        self.vx_register[x_index] = diff;
        self.vx_register[0xF] = u8::from(!overflow);
        self.next_instruction();
      }
      // 8xyE
      (8, x, _, 0xE) => {
        let x_index = usize::from(x);

        self.vx_register[0xF] = self.vx_register[usize::from(x)] & 0x1;
        self.vx_register[x_index] <<= 1;

        self.next_instruction();
      }
      // 9xy0
      (9, x, y, 0) => {
        if self.vx_register[usize::from(x)] != self.vx_register[usize::from(y)] {
          self.skip_next_instruction();
        } else {
          self.next_instruction();
        }
      }
      // Annn
      (0xA, n1, n2, n3) => {
        self.i_register = (n1, n2, n3).into_instruction_value();
        self.next_instruction();
      }
      // Bnnn
      (0xB, n1, n2, n3) => {
        self.i_register = (n1, n2, n3)
          .into_instruction_value()
          .wrapping_add(u16::from(self.vx_register[0]));
      }
      // Cxkk
      (0xC, x, k1, k2) => {
        self.vx_register[usize::from(x)] = random::<u8>() & (k1, k2).into_instruction_value() as u8;
        self.next_instruction();
      }
      // Dxyn
      (0xD, x, y, n) => {
        self.vx_register[0xF] = 0;

        let vx = usize::from(self.vx_register[usize::from(x)]);
        let vy = usize::from(self.vx_register[usize::from(y)]);
        let height = usize::from(n);

        let has_collided = display.draw(vx, vy, height, self.i_register, memory);

        self.vx_register[0xF] = if has_collided { 1 } else { 0 };
        self.next_instruction();
      }
      // Ex9E
      (0xE, x, 9, 0xE) => {
        let vx = usize::from(self.vx_register[usize::from(x)]);

        if keyboard.get_key_state(vx) {
          self.skip_next_instruction();
        } else {
          self.next_instruction();
        }
      }
      // ExA1
      (0xE, x, 0xA, 1) => {
        let vx = usize::from(self.vx_register[usize::from(x)]);

        if keyboard.get_key_state(vx) {
          self.next_instruction();
        } else {
          self.skip_next_instruction();
        }
      }
      // Fx07
      (0xF, x, 0, 7) => {
        self.vx_register[usize::from(x)] = self.delay_timer;
        self.next_instruction();
      }
      // Fx0A
      (0xF, x, 0, 0xA) => {
        self.vx_register[usize::from(x)] = display.await_for_key(&keyboard);
        self.next_instruction();
      }

      // Fx15
      (0xF, x, 1, 5) => {
        self.delay_timer = self.vx_register[usize::from(x)];
        self.next_instruction();
      }
      // Fx18
      (0xF, x, 1, 8) => {
        self.sound_timer = self.vx_register[usize::from(x)];
        self.next_instruction();
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
      // Fx29
      (0xF, x, 2, 9) => {
        self.i_register = u16::from(self.vx_register[usize::from(x)]) * 5;

        self.next_instruction();
      }

      // Fx33
      (0xF, x, 3, 3) => {
        let vx = self.vx_register[usize::from(x)];
        let i_index = usize::from(self.i_register);

        memory.set_byte(i_index, vx / 100);
        memory.set_byte(i_index + 1, (vx % 100) / 10);
        memory.set_byte(i_index + 2, vx % 10);

        self.next_instruction();
      }
      // Fx55
      (0xF, x, 5, 5) => {
        let i_register = usize::from(self.i_register);
        for i in 0..=usize::from(x) {
          let vx = self.vx_register[i];

          memory.set_byte(i_register + i, vx);
        }

        self.i_register += x + 1;
        self.next_instruction();
      }
      // Fx65
      (0xF, x, 6, 5) => {
        let i_register = usize::from(self.i_register);
        for i in 0..=usize::from(x) {
          self.vx_register[i] = memory.get_byte(i_register + i);
        }

        self.i_register += x + 1;

        self.next_instruction();
      }
      _ => panic!("undefined instruction {:#X}", instruction.get_value()),
    }

    if (self.delay_timer) > 0 {
      self.delay_timer -= 1
    }

    if (self.sound_timer) > 0 {
      self.sound_timer -= 1
    }
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
