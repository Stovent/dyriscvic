use dyriscvic::{common::memory::MemoryAccess, rv32i::RV32I};

struct RAM {
    pub memory: [u8; 4096],
}

impl MemoryAccess for RAM {
    fn get8(&mut self, addr: u64) -> u8 {
        return self.memory[addr as usize];
    }

    fn get16(&mut self, addr: u64) -> u16 {
        return self.get16le(addr);
    }

    fn get16le(&mut self, addr: u64) -> u16 {
        return (self.get8(addr + 1) as u16) << 8 | self.get8(addr) as u16;
    }

    fn get32(&mut self, addr: u64) -> u32 {
        return self.get32le(addr);
    }

    fn get32le(&mut self, addr: u64) -> u32 {
        return (self.get16le(addr + 2) as u32) << 16 | self.get16le(addr) as u32;
    }

    fn set8(&mut self, addr: u64, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn set16(&mut self, addr: u64, data: u16) {
        self.set16le(addr, data);
    }

    fn set16le(&mut self, addr: u64, data: u16) {
        self.set8(addr, data as u8);
        self.set8(addr + 1, (data >> 8) as u8);
    }

    fn set32(&mut self, addr: u64, data: u32) {
        return self.set32le(addr, data);
    }

    fn set32le(&mut self, addr: u64, data: u32) {
        self.set16le(addr, data as u16);
        self.set16le(addr + 2, (data >> 16) as u16);
    }
}

fn main() {
    let mut mem = RAM { memory: [0; 4096] };
    // mem.memory[0..4].copy_from_slice(&[0b0_001_0011, 0b1_100_0110, 0b0000_0001, 0xF8]); // XORI
    mem.memory[0..4].copy_from_slice(&[0b1_110_1111, 0b0000_0000, 0b000_0_0000, 0b0_0000000]); // JAL 0
    let mut rv32 = RV32I::new("", &mut mem);
    for _ in 1..130_000_000 {
        rv32.single_step();
    }
}
