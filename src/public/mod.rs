//! Traits and enums that has to be defined by the user and provided to dyriscvic.

/// Memory access interface. Has to be defined by the host and provided to dyriscvic.
pub trait MemoryAccess<ADDR> {
    /// Returns the byte at the given address.
    fn get_8(&mut self, addr: ADDR) -> u8;

    /// Returns the half-word (16 bits) at the given address.
    fn get_16(&mut self, addr: ADDR) -> u16;

    /// Returns the word (32 bits) at the given address.
    fn get_32(&mut self, addr: ADDR) -> u32;

    /// Returns the double-word (64 bits) at the given address.
    fn get_64(&mut self, addr: ADDR) -> u64;

    /// Sets the given byte at the given address.
    fn set_8(&mut self, addr: ADDR, data: u8);

    /// Sets the given half-word (16 bits) at the given address.
    fn set_16(&mut self, addr: ADDR, data: u16);

    /// Sets the given word (32 bits) at the given address.
    fn set_32(&mut self, addr: ADDR, data: u32);

    /// Sets the given double-word (64 bits) at the given address.
    fn set_64(&mut self, addr: ADDR, data: u64);

    /// Returns the 32-bits opcode at the given address.
    fn get_opcode_32(&mut self, addr: ADDR) -> u32;
}

/// Traps that can occur during execution.
#[derive(Clone, Copy, Debug)]
pub enum Traps {
    /// Occurs when an EBREAK instruction is executed.
    Breakpoint,
    /// Occurs when branching to a misaligned address.
    InstructionAddressMisaligned,
    /// Occurs when an illegal instruction or unknown opcode is executed.
    IllegalInstruction,
    /// Occurs when an ECALL instruction is executed.
    SystemCall,
}

/// Trait representing the execution environment.
pub trait ExecutionEnvironmentInterface<ADDR> : MemoryAccess<ADDR> {
    /// Called by the library when a trap occurs. See [`Traps`] for a list of the possible traps.
    fn trap(&mut self, trap: Traps);
}
