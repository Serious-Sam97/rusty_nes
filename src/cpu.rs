struct CPU {
    a: u8; //Accumulator
    y: u8; //Index Register Y
    x: u8; //Index Register X
    pc: u16; //Program Counter
    sp: u8; //Stack Pointer
    p:  u8; //Status Register
}