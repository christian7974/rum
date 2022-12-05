/// Initializes the registers that will hold integer values to be used in operations, returns the registers vector
/// No parameters needed for this function, since our rum will always initialize 8 registers by default
pub fn initialize_registers() -> Vec<u32>  {
    let rum_memory: Vec<u32> = vec![0; 8];
    return rum_memory;
}