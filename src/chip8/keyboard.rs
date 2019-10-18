extern crate minifb;

use minifb::Key;

pub struct Keyboard {
  keys: [Key; 16],
  pub keys_state: [bool; 16],
}

impl Keyboard {
  pub fn new() -> Keyboard {
    Keyboard {
      keys: Keyboard::chip8_keys(),
      keys_state: [false; 16],
    }
  }

  pub fn get_key(&self, chip8_key: usize) -> Key {
    self.keys[chip8_key]
  }

  pub fn set_keys_state(&mut self, keys_state: [bool; 16]) {
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
      Key::X,    // 0
      Key::Key1, // 1
      Key::Key2, // 2
      Key::Key3, // 3
      Key::Q,    // 4
      Key::W,    // 5
      Key::E,    // 6
      Key::A,    // 7
      Key::S,    // 8
      Key::D,    // 9
      Key::Z,    // 10
      Key::C,    // 11
      Key::Key4, // 12
      Key::R,    // 13
      Key::F,    // 14
      Key::V,    // 15
    ]
  }
}
