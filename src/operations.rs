use core::{panic};
use std::io::Read;
use std::process::{exit};
use crate::binary::{UM};

/// Function that is called for OPCODE 0 (Conditional Move)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn conditional_move(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) -> u32{
    if machine.registers[register_c as usize] != 0 {
        machine.registers[register_a as usize] = machine.registers[register_b as usize];
    }
    return machine.registers[register_a as usize];
    
}

/// Function that is called for OPCODE 1 (Segmented Load)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn segmented_load(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32)-> u32 {
    machine.registers[register_a as usize] = machine.memory[machine.registers[register_b as usize] as usize][machine.registers[register_c as usize] as usize];
    return machine.registers[register_a as usize];
}

/// Function that is called for OPCODE 2 (Segmented Store)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn segmented_store(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) {
    machine.memory[machine.registers[register_a as usize] as usize][machine.registers[register_b as usize] as usize] = machine.registers[register_c as usize];
}

/// Function that is called for OPCODE 3 (Addition) which involves adding the values in registers B and C, modding that value by 2^32 and
///  then putting that sum into register A
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn addition(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) -> u32 {
    machine.registers[register_a as usize] = ((machine.registers[register_b as usize] as u64 + machine.registers[register_c as usize] as u64) % 2_u64.pow(32)) as u32;
    return machine.registers[register_a as usize];
}

/// Function that is called for OPCODE 4 (Multiplication) which involves multiplying the values in registers B and C, modding that value by 2^32 and
/// then putting that product into register A
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn multiplication(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) -> u32 {
    machine.registers[register_a as usize] = ((machine.registers[register_b as usize] as u64 * machine.registers[register_c as usize] as u64) % 2_u64.pow(32)) as u32;
    return machine.registers[register_a as usize];
}

/// Function that is called for OPCODE 5 (Division) which involves dividing the values in registers B and C, and
/// then putting that product into register A
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn division(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) -> u32 {
    if machine.registers[register_c as usize] == 0 {
        panic!();
    }
    machine.registers[register_a as usize] = machine.registers[register_b as usize] / machine.registers[register_c as usize];
    return machine.registers[register_a as usize];
}

/// Function that is called for OPCODE 6 (Bitwise NAND [NOT AND]) which involves calculating the bitwise AND in registers b and c and then taking the complement
/// of that value
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a`: u32 value that represents the value that is in register A
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn bit_nand(machine: &mut UM, register_a: u32, register_b: u32, register_c: u32) -> u32 {
    machine.registers[register_a as usize] = !(machine.registers[register_b as usize] & machine.registers[register_c as usize]);
    return machine.registers[register_a as usize];
}

/// Function that is called for OPCODE 7 (halt), terminating the program
pub fn halt() {
    exit(0);
}

/// Function that is called for OPCODE 8 (Map Segment), creating a new segment and putting that segment into
/// our memory.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn map_segment(machine: &mut UM, register_b: u32, register_c: u32) -> u32 {
    let num_words = machine.registers[register_c as usize];
    let inititialized_word = 0;
    let new_seg_to_map = vec![inititialized_word; num_words as usize];
    if !machine.queue.is_empty() {
        let index_num = machine.queue.pop().unwrap();
        machine.memory[index_num as usize] = new_seg_to_map;
        machine.registers[register_b as usize] = index_num;
    } else {
        machine.registers[register_b as usize] = machine.memory.len() as u32;
        machine.memory.push(new_seg_to_map);
    }
    return machine.registers[register_b as usize];
}

/// Function that is called for OPCODE 9 (Unmap Segment) that gets rid of a segment in our memory. We push that ID
/// into a queue, allowing us to reuse that memory if we every need to allocate more.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_c`: u32 value that represents the value that is in register C
pub fn unmap_segment(machine: &mut UM, register_c: u32) {
    machine.queue.push(machine.registers[register_c as usize]);
    machine.memory[machine.registers[register_c as usize] as usize] = [].to_vec(); 
}

/// Function that is called for OPCODE 10 (Output) that outputs the value in register C iff the value is greater than
/// 0 and less than 255.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_c`: u32 value that represents the value that is in register C
pub fn output(machine: &mut UM, register_c: u32) {
    if machine.registers[register_c as usize] > 255 {
        panic!();
    } else {
        print!("{}", char::from_u32(machine.registers[register_c as usize]).unwrap());
    }
}

/// Function that is called for OPCODE 11 (Input) that inputs a value into register C.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_c`: u32 value that represents the value that is in register C
pub fn input(machine: &mut UM, register_c: u32) -> u32 {
    let mut buff:[u8; 1] = [0];
    let num_bytes_read = std::io::stdin().read(&mut buff); // update the contents of buffer from stdin, returns the number of bytes read
    match num_bytes_read {
        Ok(1) => return buff[0] as u32,
        Ok(0) => return u32::MAX,
        _ => panic!(),
    }
}

/// Function that is called for OPCODE 12 (Load Program) which takes an already existing memory segment, duplicating
/// that segment and then putting that segment into the machine's instruction vector.
/// * `machine`: the machine to operate on (of type UM)
/// * `register_b`: u32 value that represents the value that is in register B
/// * `register_c`: u32 value that represents the value that is in register C
pub fn load_program(mut machine: &mut UM, register_b: u32, register_c: u32) -> u32 { // DO NOT INC PROGRAM COUNTER HERE
    if machine.registers[register_b as usize] != 0 {
        machine.memory[0] = machine.memory[machine.registers[register_b as usize] as usize].clone();

    } 
    machine.program_counter = machine.registers[register_c as usize];  
    return machine.program_counter; 
}

/// Function that is called for OPCODE 13 (Load Value) that loads a number into register A (the 3 bits after the
/// OPCODE)
/// * `machine`: the machine to operate on (of type UM)
/// * `register_a_prime`: u32 value that represents the value that is in register A (but not the same register A 
/// as the other instructions)
/// * `val_to_load`: u32 value that represents the value to load into `register_a_prime`
pub fn load_value(machine: &mut UM, register_a_prime: u32, val_to_load: u32) -> u32 {
    machine.registers[register_a_prime as usize] = val_to_load;
    return machine.registers[register_a_prime as usize];
}