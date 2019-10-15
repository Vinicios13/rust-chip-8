use super::Memory;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
  gfx: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
  should_render: bool,
}

impl Display {
  pub fn new() -> Display {
    Display {
      gfx: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
      should_render: false,
    }
  }

  pub fn draw(
    &mut self,
    x: usize,
    y: usize,
    height: usize,
    i_register: u16,
    memory: &mut Memory,
  ) -> bool {
    let mut has_collided = false;

    for i in 0..height {
      let pixel = memory.get_instruction(i_register + i as u16).get_value();

      for j in 0..8 {
        if (pixel & (0x80 >> j)) != 0 {
          let index = ((x + j) + ((y + i) * DISPLAY_WIDTH)) % 2048;

          if self.gfx[index] == 1 {
            has_collided = true;
          }

          self.gfx[index] ^= 1;
        }
      }
    }

    self.should_render = true;
    has_collided
  }

  pub fn render(&self) {}

  pub fn set_should_render(&mut self, should_render: bool) {
    self.should_render = should_render;
  }

  pub fn get_should_render(&mut self) -> bool {
    self.should_render
  }
}
