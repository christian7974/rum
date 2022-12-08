use rum::binary;
use rum::binary::UM;
use std::env;
use std::time::Instant;

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
    let start = Instant::now();
    let mut machine = UM::new();
    machine.boot(instructions.clone());
    machine.run();
    if flag.clone() == Some(("-t").to_string()) {
        eprintln!("it took {:2?}", start.elapsed());
    }
    println!("finished running");
}
