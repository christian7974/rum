use rum::binary;
use rum::binary::UM;
use std::env;
// TO ZIP: zip -r rum.zip rum -x "rum/Cargo.lock" -x "rum/target/**" -x "rum/.git/*"
// DELETE THE OLD .ZIP BEFORE RUNNING THIS
// TO RUN WITH TIME: \time ./target/debug/rum hello.um
fn main() {
    let input = env::args().nth(1);
    let flag = env::args().nth(2);
    if flag != None{
        flag.clone().unwrap();
    }
    let instructions = binary::load(input.as_deref());
    if flag.clone() == Some(("-d").to_string()) {
        println!("{} instructions in the binary", instructions.clone().len());
        println!("the instructions were:");
        for i in 0..instructions.len() {
            println!("{:b}", instructions[i]);
        }
    }
    let mut machine = UM::new();
    machine.boot(instructions.clone());
    machine.run();
}
