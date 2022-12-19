use std::{convert::TryInto, vec};
use crate::operations;

/// Our UM struct that has all of the architecture for our universal machine
/// * `memory`: 2D vector of integers that holds a memory segment (a vector of u32s)
/// * `registers`: Vector of u32s that represent the 8 registers in our UM
/// * `program_counter`: u32 that is the program counter for our UM
/// * `queue`: Vector of u32s that represents the queue we will use to aid us in making sure that we unmap and map our segments properly
pub struct UM {
    pub memory: Vec<Vec<u32>>,
    pub registers: Vec<u32>,
    pub program_counter: u32,
    pub queue: Vec<u32>
}

impl UM {
    /// Function that will create a new instance of our UM (returns a type UM)
    #[inline(always)]
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
    /// * `instructions_from_binary`: The list of instructions from our binary as a vector of u32 integers that are loaded into
    pub fn boot(&mut self, instructions_from_binary: &[u32]) {
        self.registers = vec![0; 8];
        self.memory[0] = instructions_from_binary.to_vec();
        self.program_counter = 0;
    }

    /// Method that will fetch the instruction so that the machine can continue running (returning the instruction at memory segment 0
    /// at the program counter) 
    /// * `self`: The instance of the universal machine struct
    /// * `flag`: Flag entered in command line (used for debugging purposes)
    #[inline(always)]
    pub fn fetch(&mut self, flag: Option<String>) -> u32 {
        if flag.clone() == Some(("-d").to_string()) {
            println!("length of seg 0 is {} and the program counter is at {}", self.memory[0].len(), self.program_counter);
        }
        return self.memory[0][self.program_counter as usize];
    }

    /// Function that will run the machine with the instructions from the binary; will be in charge of ending the program
    /// as well.
    /// * `self`: The instance of the universal machine struct
    /// * `flag`: Flag entered in command line (used for debugging purposes)
    #[inline(always)]
    pub fn run (&mut self, flag: Option<String>) {
        type Umi = u32;
        pub struct Field {
            width: u32,
            lsb: u32,
        }
        static RA: Field = Field {width: 3, lsb: 6};
        static RB: Field = Field {width: 3, lsb: 3};
        static RC: Field = Field {width: 3, lsb: 0};
        static RL: Field = Field {width: 3, lsb: 25};
        static VL: Field = Field {width: 25, lsb: 0};
        pub static OP: Field = Field {width: 4, lsb: 28};
        
        /// Helper function to extract the proper field from an instruction
        /// * `bits`: u32 which represents the amount to shift left by
        fn mask(bits: u32) -> u32 { 
            (1 << bits) - 1 
        }
        
        /// Helper function that can extract something from an instruction
        /// * `field`: a reference to a type field which represents the information that you want to extract from an instruction
        /// * `instruction`: a type Umi (Universal machine instruction) which represents the instruction that you want to extract from
        pub fn get(field: &Field, instruction: Umi) -> u32 {
            (instruction >> field.lsb) & mask(field.width)
        }
        
        /// Helper function that can extract something from an instruction
        /// * `field`: a reference to a type field which represents the information that you want to extract from an instruction
        /// * `instruction`: a type Umi (Universal machine instruction) which represents the instruction that you want to extract from
        pub fn op(instruction: Umi) -> u32 {
            (instruction >> OP.lsb) & mask(OP.width)
        }
        
        enum Opcode {CMov, SegLoad, SegStore, ADD, MUL, DIV, BitNAND, HALT, MapSeg, UnmapSeg, Output, Input, LoadProgram, LoadValue}


        let mut num_inst = 1;
        loop {
            // self.fetch(flag.clone())
            let individual_instruction = self.memory[0][self.program_counter as usize];
            match get(&OP, individual_instruction) {
                o if o == Opcode::CMov as u32 => {
                    operations::conditional_move(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                },
                o if o == Opcode::SegLoad as u32 => {
                    operations::segmented_load(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                    },
        
                o if o == Opcode::SegStore as u32 => {
                    operations::segmented_store(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                   },
        
                o if o == Opcode::ADD as u32 => {
                    operations::addition(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                    },
        
                o if o == Opcode::MUL as u32 => {
                    operations::multiplication(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                    },
        
                o if o == Opcode::DIV as u32 => {
                    operations::division(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                   },
                    
                o if o == Opcode::BitNAND as u32 => {
                    operations::bit_nand(self, 
                        get(&RA, individual_instruction), 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                   },
        
                o if o == Opcode::HALT as u32 => {
                    operations::halt()
                   },
        
                o if o == Opcode::MapSeg as u32 => {
                    operations::map_segment(self, 
                        get(&RB, individual_instruction),
                        get(&RC, individual_instruction))
                    },
        
                o if o == Opcode::UnmapSeg as u32 => {
                    operations::unmap_segment(self, 
                        get(&RC, individual_instruction))
                },
        
                o if o == Opcode::Output as u32 => {
                    operations::output(self, 
                        get(&RC, individual_instruction))
                },
                   
                o if o == Opcode::Input as u32 => {
                    operations::input(self,
                        get(&RC, individual_instruction))
                },
        
                o if o == Opcode::LoadProgram as u32 => {
                    operations::load_program(self, 
                        get(&RB, individual_instruction), 
                        get(&RC, individual_instruction))
                },
        
                o if o == Opcode::LoadValue as u32 => {
                    operations::load_value(self, 
                        get(&RL, individual_instruction),
                    get(&VL, individual_instruction))
                },
        
                _ => {
                    panic!()
                },
            }
            if get(&OP, individual_instruction) != 12 {
                self.program_counter += 1;
            }
            if flag.clone() == Some(("-d").to_string()) {
                self.output_archs(individual_instruction, num_inst);
                num_inst += 1;
            }
        }
        
    }

    /// Helper function that prints out all of the architecture of our UM (the registers, what instruction we are holding, etc.); only called
    /// when the flag "-d" is passed in the command line
    /// * `self`: The instance of the universal machine struct
    /// * `individual_instruction`: The individual instruction being passed into the uM
    /// * `num_inst`: The numbered instruction in the binary we are on
    pub fn output_archs (&mut self, individual_instruction: u32, num_inst: u32) {
        println!("the current instruction is which is instruction {}", num_inst);
        for i in 0..8 {
            println!("register {} is holding {}", i, self.registers[i]);
        }
        println!();
        for i in 0..self.memory.len() {
            if self.memory[i] != [] {
                println!("the memory segment {} is holding {:?}", i, self.memory[i]);
            }
        }
    }
}
/// Load function that returns a vector of u32 integers representing the instructions from the binary that we read
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