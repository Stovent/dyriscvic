pub trait MemoryAccess {
    fn get8(&mut self, addr: u64) -> u8;
    fn get16(&mut self, addr: u64) -> u16;
    fn get16le(&mut self, addr: u64) -> u16;
    fn get32(&mut self, addr: u64) -> u32;
    fn get32le(&mut self, addr: u64) -> u32;

    fn set8(&mut self, addr: u64, data: u8);
    fn set16(&mut self, addr: u64, data: u16);
    fn set16le(&mut self, addr: u64, data: u16);
    fn set32(&mut self, addr: u64, data: u32);
    fn set32le(&mut self, addr: u64, data: u32);
}

pub trait ExecutionEnvironmentInterface : MemoryAccess {
    fn trap(&mut self);
}
