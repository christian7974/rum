/// Initializes the memory segments that our UM will use as our RAM
/// * `instructions_to_be_inserted`: Vector of u32 integers that represent an instruction in the inputted binary
pub fn initialize_memory(instructions_to_be_inserted: Vec<u32>) -> Vec<Vec<u32>>  {
    let memory: Vec<Vec<u32>> = vec![instructions_to_be_inserted];
    return memory;
}