use rum::binary;
use rum::binary::UM;
use std::env;
// TO ZIP: zip -r rum.zip rum -x "rum/Cargo.lock" -x "rum/target/**" -x "rum/.git/*"
// DELETE THE OLD .ZIP BEFORE RUNNING THIS
// TO RUN WITH TIME: \time ./target/debug/rum hello.um
// time for 87,070,522 instructions (midmark.um): 20.87 real, 20.23 user, .52 sys
// to get flamegraph to work: sudo -E cargo flamegraph -- cat.um 
// to decipher flamegraph: if there is a row with things that are abnormally larger than
    // the things in the row, it means that it is taking longer to run; look at the code in
    // that section and see if it should be taking that long.
fn main() {
    let input = env::args().nth(1);
    let flag = env::args().nth(2);
    if flag != None{
        flag.clone().unwrap();
    }
    let instructions = binary::load(input.as_deref());
    let mut machine = UM::new();
    machine.boot(instructions.as_slice());
    machine.run(flag.clone());
}
