use core::panic;
use std::process::{exit};

use crate::binary::{UM, load};

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
            input(machine)
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

fn inc_program_counter(machine: &mut UM, amount_to_increment: u32) {
    machine.program_counter += amount_to_increment;
}

fn conditional_move(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    if machine.registers[register_c as usize] != 0 {
        machine.registers[register_a as usize] = machine.registers[register_b as usize];
        inc_program_counter(machine, 1);
    } else {
        panic!()
    }  
}

fn segmented_load(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = machine.memory[register_b as usize][register_c as usize];
    inc_program_counter(machine, 1);
}

fn segmented_store(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    inc_program_counter(machine, 1);
    todo!()
}

fn addition(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = (machine.registers[register_b as usize] + machine.registers[register_c as usize]) % 2_u32.pow(32);
    inc_program_counter(machine, 1);
}

fn multiplication(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = (machine.registers[register_b as usize] * machine.registers[register_c as usize]) % 2_u32.pow(32);
    inc_program_counter(machine, 1);
}

fn division(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = machine.registers[register_b as usize] / machine.registers[register_c as usize];
    inc_program_counter(machine, 1);
}

fn bit_nand(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.registers[register_a as usize] = !(machine.registers[register_b as usize] & machine.registers[register_c as usize]);
    inc_program_counter(machine, 1);
}

fn halt() {
    exit(1);
}

// ask about this, still confusing
fn map_segment(machine: &mut UM, register_b: u32, register_c: u32) {
    let new_seg: Vec<u32> = vec![0; machine.registers[register_c as usize] as usize];
    if register_c != 0 { // add other boolean check
        machine.registers = new_seg
    } else {
        machine.memory[register_c as usize] = new_seg
    }
}

fn unmap_segment(machine: &mut UM, register_c: u32) {
    machine.queue.push(register_c);

    machine.memory[register_c as usize] = [].to_vec();
    inc_program_counter(machine, 1);
}

fn output(machine: &mut UM, register_c: u32) {
    if machine.registers[register_c as usize] > 255 {
        panic!();
    } else {
        println!("{}", machine.registers[register_c as usize]);
    }
    inc_program_counter(machine, 1)
}

fn input(machine: &mut UM) {

}

fn load_program(mut machine: &mut UM, register_b: u32, register_c: u32) { // DO NOT INC PROGRAM COUNTER HERE
    let duplicated_seg: Vec<u32> = machine.memory[register_b as usize].clone();
    machine.memory[0] = duplicated_seg;
    machine.program_counter = machine.memory[0][machine.registers[register_c as usize] as usize];
}

fn load_value(machine: &mut UM, register_a_prime: u32, val_to_load: u32) {
    machine.registers[register_a_prime as usize] = val_to_load;
}