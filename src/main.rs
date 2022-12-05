use rum::bitpack;
use rum::binary;
use rum::memory;
use rum::registers;
use std::env;
fn main() {
    let input = env::args().nth(1);
    let instructions = binary::load(input.as_deref());
    println!("{} instructions", instructions.len());
    let rum_registers = registers::initialize_registers();
    let rum_memory = memory::initialize_memory(instructions);
}
