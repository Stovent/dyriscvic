//! Traits and enums that has to be defined by the user and provided to dyriscvic.

/// Memory access interface. Has to be defined by the host and provided to dyriscvic.
pub trait MemoryAccess<ADDR> {
    /// Returns the byte at the given address.
    fn get_byte(&mut self, addr: ADDR) -> u8;

    /// Returns the half-word (16 bits) at the given address.
    fn get_half(&mut self, addr: ADDR) -> u16;

    /// Returns the word (32 bits) at the given address.
    fn get_word(&mut self, addr: ADDR) -> u32;

    /// Sets the given byte at the given address.
    fn set_byte(&mut self, addr: ADDR, data: u8);

    /// Sets the given half-word (16 bits) at the given address.
    fn set_half(&mut self, addr: ADDR, data: u16);

    /// Sets the given word (32 bits) at the given address.
    fn set_word(&mut self, addr: ADDR, data: u32);

    /// Returns the 32-bits opcode at the given address.
    fn get_opcode_32(&mut self, addr: ADDR) -> u32;
}

#[derive(Debug)]
pub enum Traps {
    Breakpoint,
    InstructionAddressMisaligned,
    IllegalInstruction,
    SystemCall,
}

pub trait ExecutionEnvironmentInterface<ADDR> : MemoryAccess<ADDR> {
    fn trap(&mut self, trap: Traps);
}
