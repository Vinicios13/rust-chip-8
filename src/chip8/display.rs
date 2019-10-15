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

  pub fn draw(&mut self, x: usize, y: usize, height: usize, i: usize, memory: &mut Memory) -> bool {
    let mut has_collided = false;

    for y_line in 0..height {
      let pixel = memory.get_instruction((i + y_line) as u16).get_value();

      for x_line in 0..8 {
        if (pixel & (0x80 >> x)) != 0 {
          if self.gfx[x + x_line + ((y + y_line) * 64)] == 1 {
            has_collided = true;
          }

          self.gfx[x + x_line + ((y + y_line) * 64)] ^= 1;
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
