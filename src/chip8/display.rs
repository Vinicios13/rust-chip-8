const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
  gfx: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
  pub fn new() -> Display {
    Display {
      gfx: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    }
  }
}
