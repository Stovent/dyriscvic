pub trait MemoryAccess<ADDR> {
    fn get8(&mut self, addr: ADDR) -> u8;
    fn get16(&mut self, addr: ADDR) -> u16;
    fn get16le(&mut self, addr: ADDR) -> u16;
    fn get32(&mut self, addr: ADDR) -> u32;
    fn get32le(&mut self, addr: ADDR) -> u32;

    fn set8(&mut self, addr: ADDR, data: u8);
    fn set16(&mut self, addr: ADDR, data: u16);
    fn set16le(&mut self, addr: ADDR, data: u16);
    fn set32(&mut self, addr: ADDR, data: u32);
    fn set32le(&mut self, addr: ADDR, data: u32);
}

#[derive(Debug)]
pub enum Exceptions {
    InstructionAddressMisaligned,
    IllegalInstruction,
}

pub trait ExecutionEnvironmentInterface<ADDR> : MemoryAccess<ADDR> {
    fn exception(&mut self, exception: Exceptions);
    fn interrupt(&mut self);
}
