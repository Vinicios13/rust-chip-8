// mod memory;

// use memory::Memory;

pub struct Chip8 {
  // memory: Memory,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
      // memory: Memory::new(),
    }
  }

  pub fn load_instructions_from_file(&mut self, file_path: &str) {
    println!("{}", file_path)
  }
}
