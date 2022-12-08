use core::{panic, num};
use std::process::{exit};
use std::io;
use crate::binary::{UM, load};
use crate::memory::map_memory_segment;

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
static OP: Field = Field {width: 4, lsb: 28};
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

/// Function that will execute a singular instruction (will parse what the instruction says and call the appropriate function
/// depending on the OPCODE)
/// * `machine`: the machine that will have the instrution executed (of type UM)
/// * `inst`: a type Umi (u32) that represents a singular instruction 
pub fn execute_instruction(machine: &mut UM , inst: Umi) {
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            conditional_move(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
        },
        o if o == Opcode::SegLoad as u32 => {
            segmented_load(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
            },

        o if o == Opcode::SegStore as u32 => {
            segmented_store(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
           },

        o if o == Opcode::ADD as u32 => {
            addition(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
            },

        o if o == Opcode::MUL as u32 => {
            multiplication(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
            },

        o if o == Opcode::DIV as u32 => {
            division(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
           },
            
        o if o == Opcode::BitNAND as u32 => {
            bit_nand(machine, 
                get(&RA, inst), 
                get(&RB, inst), 
                get(&RC, inst))
           },

        o if o == Opcode::HALT as u32 => {
            halt()
           },

        o if o == Opcode::MapSeg as u32 => {
            map_segment(machine, 
                get(&RB, inst),
                get(&RC, inst))
            },

        o if o == Opcode::UnmapSeg as u32 => {
            unmap_segment(machine, 
                get(&RC, inst))
        },

        o if o == Opcode::Output as u32 => {
            output(machine, 
                get(&RC, inst))
        },
           
        o if o == Opcode::Input as u32 => {
            input(machine,
                get(&RC, inst))
        },

        o if o == Opcode::LoadProgram as u32 => {
            load_program(machine, 
                get(&RA, inst), 
                get(&RB, inst))
        },

        o if o == Opcode::LoadValue as u32 => {
            load_value(machine, 
                get(&RL, inst),
            get(&VL, inst))
        },

        _ => {
            panic!()
        },
}}

/// Helper function that increments the program counter of the machine
/// * `machine`: the machine to operate on (of type UM)
/// * `amount_to_increment`: a u32 that is the amount to increment the program counter by (usually 1)
fn inc_program_counter(machine: &mut UM, amount_to_increment: u32) {
    machine.program_counter += amount_to_increment;
}

/// Function that is called for OPCODE 0 (Conditional Move)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn conditional_move(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    if machine.registers[register_c as usize] != 0 {
        machine.registers[register_a as usize] = machine.registers[register_b as usize];
        inc_program_counter(machine, 1);
    } else {
        panic!()
    }  
}

/// Function that is called for OPCODE 1 (Segmented Load)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn segmented_load(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = machine.memory[register_b as usize][register_c as usize];
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 2 (Segmented Store)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn segmented_store(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.memory[register_a as usize][register_b as usize] = machine.registers[register_c as usize];
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 3 (Addition) which involves adding the values in registers B and C, modding that value by 2^32 and
///  then putting that sum into register A
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn addition(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = (machine.registers[register_b as usize] + machine.registers[register_c as usize]) % 2_u32.pow(32);
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 4 (Multiplication) which involves multiplying the values in registers B and C, modding that value by 2^32 and
/// then putting that product into register A
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn multiplication(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = (machine.registers[register_b as usize] * machine.registers[register_c as usize]) % 2_u32.pow(32);
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 5 (Division) which involves dividing the values in registers B and C, and
/// then putting that product into register A
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn division(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = machine.registers[register_b as usize] / machine.registers[register_c as usize];
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 6 (Bitwise NAND [NOT AND]) which involves calculating the bitwise AND in registers b and c and then taking the complement
/// of that value
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn bit_nand(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = !(machine.registers[register_b as usize] & machine.registers[register_c as usize]);
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 7 (halt), terminating the program
fn halt() {
    exit(0);
}

/// Function that is called for OPCODE 8 (Map Segment), creating a new segment and putting that segment into
/// our memory.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn map_segment(machine: &mut UM, register_b: u32, register_c: u32) {
    // create a segment that has r[c] words in it
    let num_words = machine.registers[register_c as usize];
    let initialized_word: u32 = 0;
    let new_seg_to_map = vec![initialized_word; num_words as usize];
    let address = map_memory_segment(machine, new_seg_to_map);
    machine.registers[register_b as usize] = address; 
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 9 (Unmap Segment) that gets rid of a segment in our memory. We push that ID
/// into a queue, allowing us to reuse that memory if we every need to allocate more.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_c`: u32 value that represents the value that is in register C
fn unmap_segment(machine: &mut UM, register_c: u32) {
    machine.queue.push(register_c);
    // did this so that the indices would not change when we got rid of something
    machine.memory[register_c as usize] = [].to_vec();
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 10 (Output) that outputs the value in register C iff the value is greater than
/// 0 and less than 255.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_c`: u32 value that represents the value that is in register C
fn output(machine: &mut UM, register_c: u32) {
    if machine.registers[register_c as usize] > 255 {
        panic!();
    } else {
        print!("{}", char::from_u32(machine.registers[register_c as usize]).unwrap());
    }
    inc_program_counter(machine, 1);
}

/// Function that is called for OPCODE 11 (Input) that inputs a value into register C.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_c`: u32 value that represents the value that is in register C
fn input(machine: &mut UM, register_c: u32) {
    // input bytes 
    // read chars from stdin 
    // rum meant to be able to interact w real world through stdin and stdout 
    // read 1 char at a time through std in 
    // stdin is a vec of bytes 
    // input instructions reads bytes one at a time in order 
    // 7 6 5 4 => r[c] := 7; r[c] := 6; ...; r[c] := 4;
    // When end of input is reached, r[c] is loaded with full 32 bit word where every  bit is 1 
    // Why not 1 char at a time? stdin is actually a buffer of u8s 
    // buffer is temporary place where bytes representing input are stored 
    
    todo!()
}

/// Function that is called for OPCODE 12 (Load Program) which takes an already existing memory segment, duplicating
/// that segment and then putting that segment into the machine's instruction vector.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
fn load_program(mut machine: &mut UM, register_b: u32, register_c: u32) { // DO NOT INC PROGRAM COUNTER HERE
    let duplicated_seg: Vec<u32> = machine.memory[register_b as usize].clone();
    machine.memory[0] = duplicated_seg;
    machine.program_counter = machine.memory[0][machine.registers[register_c as usize] as usize];
}

/// Function that is called for OPCODE 13 (Load Value) that loads a number into register A (the 3 bits after the
/// OPCODE)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a_prime`: u32 value that represents the value that is in register A (but not the same register A 
/// as the other instructions)
/// * `val_to_load`: u32 value that represents the value to load into `register_a_prime`
fn load_value(machine: &mut UM, register_a_prime: u32, val_to_load: u32) {
    machine.registers[register_a_prime as usize] = val_to_load;
    inc_program_counter(machine, 1);
}