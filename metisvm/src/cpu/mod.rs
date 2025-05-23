pub struct Cpu {
    r: [u8; 8], //Registers
    mar: u8,    //Memory address register
    mbr: u8,    //Memory buffer register
    pc: u8,     //Program Counter (Points towards instruction to be fetched)
    ir: u8,     //Instruction Register (Stores the instruction to be executed)
    icc: u8,    //CPU cycle stage
}
