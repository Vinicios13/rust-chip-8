pub struct Memory {
  memory: [u8; 0xFFF], //4095 positions
}

pub struct Instruction {
  value: u16,
}

impl Memory {
  pub fn new(instructions: Vec<u8>) -> Memory {
    // 0x000 to 0x1FF Reserved for interpreter
    if instructions.len() > (0xFFF - (0x1FF)) {
      panic!("Max instructions length is 3583");
    }

    let mut memory = [0; 0xFFF];

    Memory::load_predefined_instructions(&mut memory);
    Memory::load_instructions(&mut memory, &instructions);

    Memory { memory }
  }

  pub fn get_instruction(&self, address: u16) -> Instruction {
    let beggin_instruction = u16::from(self.memory[usize::from(address)]);
    let end_instruction = u16::from(self.memory[usize::from(address + 1)]);

    Instruction::new((beggin_instruction << 8) | end_instruction)
  }

  fn load_instructions(memory: &mut [u8; 0xFFF], instructions: &[u8]) {
    for (index, instruction) in instructions.iter().enumerate() {
      memory[index + 0x200] = *instruction;
    }
  }

  fn load_predefined_instructions(memory: &mut [u8; 0xFFF]) {
    let sprites: [[u8; 5]; 16] = [
      [0xF0, 0x90, 0x90, 0x90, 0xF0],
      [0x20, 0x60, 0x20, 0x20, 0x70],
      [0xF0, 0x10, 0xF0, 0x80, 0xF0],
      [0xF0, 0x10, 0xF0, 0x10, 0xF0],
      [0x90, 0x90, 0xF0, 0x10, 0x10],
      [0xF0, 0x80, 0xF0, 0x10, 0xF0],
      [0xF0, 0x80, 0xF0, 0x90, 0xF0],
      [0xF0, 0x10, 0x20, 0x40, 0x40],
      [0xF0, 0x90, 0xF0, 0x90, 0xF0],
      [0xF0, 0x90, 0xF0, 0x10, 0xF0],
      [0xF0, 0x90, 0xF0, 0x90, 0x90],
      [0xE0, 0x90, 0xE0, 0x90, 0xE0],
      [0xF0, 0x80, 0x80, 0x80, 0xF0],
      [0xE0, 0x90, 0x90, 0x90, 0xE0],
      [0xF0, 0x80, 0xF0, 0x80, 0xF0],
      [0xF0, 0x80, 0xF0, 0x80, 0x80],
    ];

    let mut i = 0;
    for sprite in &sprites {
      for &ch in sprite {
        memory[i] = ch;
        i += 1;
      }
    }
  }
}

impl Instruction {
  pub fn new(value: u16) -> Instruction {
    Instruction { value }
  }

  pub fn format_instruction(&self) -> (u16, u16, u16, u16) {
    (
      self.value >> 12,
      (self.value << 4) >> 12,
      (self.value << 8) >> 12,
      (self.value << 12) >> 12,
    )
  }

  pub fn get_value(&self) -> u16 {
    self.value
  }
}
