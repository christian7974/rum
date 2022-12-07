use std::{convert::TryInto, vec};

use crate::memory;


pub struct UM {
    pub memory: Vec<Vec<u32>>,
    pub registers: Vec<u32>,
    pub program_counter: u32,
    pub instructions_vector: Vec<u32>,
    pub queue: Vec<u32>
}

impl UM {
    pub fn new() -> UM {
        let rum: UM = UM {
            memory: vec![vec![]],
            registers: vec![],
            program_counter: 0,
            instructions_vector: vec![],
            queue: vec![]
        };
        return rum;
    }


    /// Boot function that will be called when the machine is started up before the program is ran; inititalizes the architecture 
    /// of the machine. This is before we have started to read the inputted binary file.
    /// * `self`: Means that this would affect the current instance of the UM (there will only be one UM running in our program)
    pub fn boot(&mut self, instructions_from_binary: Vec<u32>) {
        self.registers = vec![0; 8];
        self.instructions_vector = instructions_from_binary.clone();
        self.memory.push(instructions_from_binary)
        
    }
}
/// Load function that returns a vector of u32 integers representing the instructions from the binary that we read it
/// * `input`: Option reference str that represents the name of the inputted binary file to run
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
        std::fs::File::open(filename).unwrap(),
        )),
    };

    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();

    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}