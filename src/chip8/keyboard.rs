extern crate minifb;

use minifb::Key;

pub struct Keyboard {
  keys: [Key; 16],
  pub keys_state: Vec<bool>,
}

impl Keyboard {
  pub fn new() -> Keyboard {
    Keyboard {
      keys: Keyboard::chip8_keys(),
      keys_state: vec![],
    }
  }

  pub fn get_key(&self, chip8_key: usize) -> Key {
    self.keys[chip8_key]
  }

  pub fn set_keys_state(&mut self, keys_state: Vec<bool>) {
    self.keys_state = keys_state
  }

  pub fn get_key_state(&self, index: usize) -> bool {
    self.keys_state[index]
  }

  fn chip8_keys() -> [Key; 16] {
    // Key map
    // 1 2 3 C to 1 2 3 4
    // 4 5 6 D to Q W E R
    // 7 8 9 E to A S D F
    // A 0 B F to Z X C V
    [
      Key::X,
      Key::Key1,
      Key::Key2,
      Key::Key3,
      Key::Q,
      Key::W,
      Key::E,
      Key::A,
      Key::S,
      Key::D,
      Key::Z,
      Key::C,
      Key::Key4,
      Key::R,
      Key::F,
      Key::V,
    ]
  }
}
