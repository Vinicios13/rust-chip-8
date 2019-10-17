extern crate minifb;
use super::Keyboard;
use super::Memory;
use minifb::{Window, WindowOptions};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
  pub gfx: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
  window: Window,
  should_render: bool,
}

impl Display {
  pub fn new() -> Display {
    Display {
      gfx: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
      should_render: false,
      window: Window::new(
        "CHIP-8 EMULATOR - ESC to exit",
        DISPLAY_WIDTH * 10,
        DISPLAY_HEIGHT * 10,
        WindowOptions::default(),
      )
      .unwrap_or_else(|e| {
        panic!(e);
      }),
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

  pub fn render(&mut self) {
    let (width, height) = self.window.get_size();
    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
      let y_coord = y / 10;
      let offset = y * width;
      for x in 0..width {
        let x_coord = x / 10;

        let pixel = self.gfx[y_coord * DISPLAY_WIDTH + x_coord];

        let color_pixel = match pixel {
          0 => 0x0,
          1 => 0xffffff,
          _ => unreachable!(),
        };
        buffer[offset + x] = color_pixel;
      }
    }

    if let Err(err) = self.window.update_with_buffer(&buffer) {
      panic!(err);
    }
  }

  pub fn set_should_render(&mut self, should_render: bool) {
    self.should_render = should_render;
  }

  pub fn get_should_render(&mut self) -> bool {
    self.should_render
  }

  pub fn set_keys_state(&mut self, keyboard: &mut Keyboard) {
    let mut keys_state = Vec::with_capacity(16);

    self.window.update();
    for i in 0..16 {
      let key = keyboard.get_key(i);

      let key_state = self.window.is_key_down(key);

      keys_state.push(key_state);
    }

    keyboard.set_keys_state(keys_state);
  }
}
