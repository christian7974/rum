use rum::bitpack;
use rum::binary;
use rum::binary::{UM};
use rum::memory;
use rum::registers;
use std::env;
use std::vec;

/// Main function that initializes our rum
fn main() {
    let input = env::args().nth(1);
    let instructions = binary::load(input.as_deref());
    println!("{} instructions in the binary", instructions.clone().len());
    let mut machine = UM::new();
    machine.boot(instructions.clone());
    machine.run();
}
