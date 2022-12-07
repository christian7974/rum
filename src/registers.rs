/// Initializes the registers that will hold integer values to be used in operations, returns the registers vector
/// No parameters needed for this function, since our rum will always initialize 8 registers by default
pub fn initialize_registers() -> Vec<u32>  {
    let rum_memory: Vec<u32> = vec![0; 8];
    return rum_memory;
}

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

pub fn disassemble(inst: Umi) -> String {
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            // run function for cmov
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst)) },

        o if o == Opcode::SegLoad as u32 => {
            // run segload function
            format!("r{} := m[r{}][r{}];", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },

        o if o == Opcode::SegStore as u32 => {
            // run segstore function
            format!("m[r{}][r{}] := r{}", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },

        o if o == Opcode::ADD as u32 => {
            // add func
            format!("r{} := (r{} + r{}) mod 2^32", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },

        o if o == Opcode::MUL as u32 => {
            // mul function
            format!("r{} := (r{} * r{}) mod 2^32", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },

        o if o == Opcode::DIV as u32 => {
            // div function
            format!("r{} := (r{} / r{}) (integer division)", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },
            
        o if o == Opcode::BitNAND as u32 => {
            // bitnand function
            format!("r{} := ¬(r{} ^ r{}) mod 2^32", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },

        o if o == Opcode::HALT as u32 => {
            // run halt
            format!("halt") },

        o if o == Opcode::MapSeg as u32 => {
            // run mapseg
            format!("A new segment is created with a number of words
                equal to the value in $r[C]. Each word in the
                new segment is initialized to zero. A bit pattern
                that is not all zeroes and does not identify any
                currently mapped segment is placed in $r[B].
                The new segment is mapped as $m[$r[B]]")},

        o if o == Opcode::UnmapSeg as u32 => {
            // run unmap seg
            format!(" The segment $m[$r[C]] is unmapped.
            Future Map Segment instructions may reuse the
            identifier $r[C].") },

        o if o == Opcode::Output as u32 => {
            // run output
            format!("The value in $r[C] is displayed on the I/O device immediately. Only values from 0 to 255 are allowed.") },
           
        o if o == Opcode::Input as u32 => {
            // run input
            format!("The UM waits for input on the I/O device. When
            input arrives, $r[c] is loaded with the input,
            which must be a value from 0 to 255. If the end
            of input has been signaled, then $r[C] is loaded
            with a full 32-bit word in which every bit is 1") },

        o if o == Opcode::LoadProgram as u32 => {
            // run loadprogram
            format!("r{} := ¬(r{} ^ r{}) mod 2^32", get(&RA, inst), get(&RB, inst), get(&RC, inst)) },

        o if o == Opcode::LoadValue as u32 => {
            // run loadvalue
            format!("load a value into register A") },

        _ => {
            format!("INVALID OPERATION") },
}}

fn conditional_move(register_a: u32, register_b: u32, register_c: u32) {

}