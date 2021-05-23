use dyriscvic::public::*;
use dyriscvic::rvi::*;

struct ExecutionEnvironment {
    pub memory: [u8; 4096],
}

impl MemoryAccess<u32> for ExecutionEnvironment {
    fn get8(&mut self, addr: u32) -> u8 {
        return self.memory[addr as usize];
    }

    fn get16(&mut self, addr: u32) -> u16 {
        return self.get16le(addr);
    }

    fn get16le(&mut self, addr: u32) -> u16 {
        return (self.get8(addr + 1) as u16) << 8 | self.get8(addr) as u16;
    }

    fn get32(&mut self, addr: u32) -> u32 {
        return self.get32le(addr);
    }

    fn get32le(&mut self, addr: u32) -> u32 {
        return (self.get16le(addr + 2) as u32) << 16 | self.get16le(addr) as u32;
    }

    fn set8(&mut self, addr: u32, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn set16(&mut self, addr: u32, data: u16) {
        self.set16le(addr, data);
    }

    fn set16le(&mut self, addr: u32, data: u16) {
        self.set8(addr, data as u8);
        self.set8(addr + 1, (data >> 8) as u8);
    }

    fn set32(&mut self, addr: u32, data: u32) {
        return self.set32le(addr, data);
    }

    fn set32le(&mut self, addr: u32, data: u32) {
        self.set16le(addr, data as u16);
        self.set16le(addr + 2, (data >> 16) as u16);
    }
}

impl ExecutionEnvironmentInterface<u32> for ExecutionEnvironment {
    fn exception(&mut self, exception: Exceptions) {
        println!("Exception: {:?}", exception);
    }
    fn interrupt(&mut self) {}
}

fn main() {
    let mut eei = ExecutionEnvironment {
        memory: [0; 4096],
    };
    // eei.memory[0..4].copy_from_slice(&[0b0_0010011, 0b1_100_0110, 0b0000_0001, 0xF8]); // XORI
    // eei.memory[4..8].copy_from_slice(&[0b1_1101111, 0b0000_0000, 0b000_0_0000, 0b0_0000000]); // JAL 0
    eei.memory[0..4].copy_from_slice(&[0b1_0010011, 0b0_111_0000, 0b000_0_0000, 0b0_0000000]); // ANDI 0
    eei.memory[4..8].copy_from_slice(&[0b1_1101111, 0b1111_0000, 0b110_1_1111, 0b1_1111111]); // JAL -4

    let mut rv32i = RV32I::new([0; 32], 0, "", eei);

    for _ in 0..130_000_000 {
        rv32i.single_step();
    }
}
