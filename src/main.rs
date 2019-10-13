use std::{env, process};
mod chip8;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = utils::Config::new(&args);

    let mut emulator = chip8::Chip8::new();
    if let Err(err) = emulator.load_instructions_from_file(config.get_filename()) {
        println!("{}", err);
        process::exit(1);
    }
}
