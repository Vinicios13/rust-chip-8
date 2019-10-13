mod memory;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use memory::Memory;

pub struct Chip8 {
  memory: Option<Memory>,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 { memory: None }
  }

  pub fn load_instructions_from_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    self.memory = Some(Memory::new(buffer));

    Ok(())
  }
}
