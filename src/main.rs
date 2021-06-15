use dyriscvic::public::*;
use dyriscvic::{rvi::*, common::*};

struct ExecutionEnvironment {
    pub memory: [u8; 4096],
}

impl MemoryAccess<u32> for ExecutionEnvironment {
    fn get_byte(&mut self, addr: u32) -> u8 {
        return self.memory[addr as usize];
    }

    fn get_half(&mut self, addr: u32) -> u16 {
        return (self.get_byte(addr + 1) as u16) << 8 | self.get_byte(addr) as u16;
    }

    fn get_word(&mut self, addr: u32) -> u32 {
        return (self.get_half(addr + 2) as u32) << 16 | self.get_half(addr) as u32;
    }

    fn set_byte(&mut self, addr: u32, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn set_half(&mut self, addr: u32, data: u16) {
        self.set_byte(addr, data as u8);
        self.set_byte(addr + 1, (data >> 8) as u8);
    }

    fn set_word(&mut self, addr: u32, data: u32) {
        self.set_half(addr, data as u16);
        self.set_half(addr + 2, (data >> 16) as u16);
    }

    fn get_opcode_32(&mut self, addr: u32) -> u32 {
        return self.get_word(addr);
    }
}

impl ExecutionEnvironmentInterface<u32> for ExecutionEnvironment {
    fn trap(&mut self, trap: Traps) {
        println!("Trap: {:?}", trap);
    }
}

fn main() {
    let mut eei = ExecutionEnvironment {
        memory: [0; 4096],
    };

    eei.memory[0..4].copy_from_slice(&u32_to_slice_le(assemble::ANDI(1, 0, 0))); // ANDI x1, x0, 0
    eei.memory[4..8].copy_from_slice(&u32_to_slice_le(assemble::JAL(0, -4i32 as u32))); // JAL x0, -4

    let mut rv32i = RV32I::new([0; 32], 0, "", eei);

    for _ in 0..130_000_000 {
        rv32i.single_step();
    }
}
