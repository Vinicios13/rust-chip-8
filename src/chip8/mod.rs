mod cpu;
mod memory;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use cpu::Cpu;
use memory::Memory;

pub struct Chip8 {
  memory: Option<Memory>,
  cpu: Cpu,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
      memory: None,
      cpu: Cpu::new(),
    }
  }

  pub fn load_instructions_from_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    self.memory = Some(Memory::new(buffer));

    Ok(())
  }

  pub fn run(&mut self) {
    if let Some(memory) = &mut self.memory {
      self.cpu.run_instruction(memory);
    } else {
      panic!("memory was not set")
    }
  }
}
