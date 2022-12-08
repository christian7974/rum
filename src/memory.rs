use crate::binary::UM;

/// Function maps a given segment to memory. Returns the address to which the segment was mapped
/// * `machine`: the machine to operate on (of type UM)
/// * `to_map`: Vec<u32> the segment to map to memory 
pub fn map_memory_segment(machine: &mut UM, to_map: Vec<u32>) -> u32 {
    let address: u32; 
    if machine.queue.is_empty() {
        address = machine.memory.len() as u32;
        machine.memory.push(to_map);
    }else { 
        address = machine.queue.pop().unwrap();
        machine.memory[address as usize] = to_map;
    }
    return address; 
}