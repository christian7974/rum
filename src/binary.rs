use std::{convert::TryInto, vec};

use crate::{memory, registers::execute_instruction};

/// Our UM struct that has all of the architecture for our universal machine
/// * `memory`: 2D vector of integers that holds a memory segment (a vector of u32s)
/// * `registers`: Vector of u32s that represent the 8 registers in our UM
/// * `program_counter`: u32 that is the program counter for our UM
/// * `instructions_vector`: Vector of u32s that represent all of the instructions in our program
/// * `queue`: Vector of u32s that represents the queue we will use to aid us in making sure that we unmap and map our segments properly
pub struct UM {
    pub memory: Vec<Vec<u32>>,
    pub registers: Vec<u32>,
    pub program_counter: u32,
    pub queue: Vec<u32>
}

impl UM {
    /// Function that will create a new instance of our UM (returns a type UM)
    pub fn new() -> UM {
        let rum: UM = UM {
            memory: vec![vec![]],
            registers: vec![],
            program_counter: 0,
            queue: vec![]
        };
        return rum;
    }

    /// Boot function that will be called when the machine is started up before the program is ran; inititalizes the architecture 
    /// of the machine.
    /// * `self`: The instance of the universal machine struct
    pub fn boot(&mut self, instructions_from_binary: Vec<u32>) {
        self.registers = vec![0; 8];
        self.memory[0] = instructions_from_binary;
        self.program_counter = 0;
    }

    pub fn fetch(&mut self) -> u32 {
        return self.memory[0][self.program_counter as usize];
    }

    /// Function that will run the machine with the instructions from the binary; will be in charge of ending the program
    /// as well.
    /// * `self`: The instance of the universal machine struct
    pub fn run (&mut self) {
        // decode and execute are in the execute_instruction function
        // fetch instr, decode, execute
        loop {
            // fetch is getting insturction
            let individual_instruction = self.fetch();
            execute_instruction(self, individual_instruction);
        }
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