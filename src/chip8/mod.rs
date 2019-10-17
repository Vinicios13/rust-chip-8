extern crate fps_clock;

mod cpu;
mod display;
mod keyboard;
mod memory;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

pub struct Chip8 {
  memory: Option<Memory>,
  cpu: Cpu,
  display: Display,
  keyboard: Keyboard,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
      memory: None,
      cpu: Cpu::new(),
      display: Display::new(),
      keyboard: Keyboard::new(),
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
      let mut fps = fps_clock::FpsClock::new(60);

      loop {
        self.display.set_should_render(false);

        self
          .cpu
          .run_instruction(memory, &mut self.display, &self.keyboard);

        if self.display.get_should_render() {
          self.display.render();
        }

        self.display.set_keys_state(&mut self.keyboard);

        fps.tick();
      }
    } else {
      panic!("memory was not set")
    }
  }
}
