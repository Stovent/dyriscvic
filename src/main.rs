use dyriscvic::common::Instruction32;
use dyriscvic::common::isa::ISA;
use dyriscvic::public::ExecutionEnvironmentInterface;
use dyriscvic::public::MemoryAccess;
use dyriscvic::rv32i::RV32I;

struct ExecutionEnvironment {
    pub memory: [u8; 4096],
}

impl MemoryAccess for ExecutionEnvironment {
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

impl ExecutionEnvironmentInterface for ExecutionEnvironment {
    fn trap(&mut self) {}
}

fn main() {
    let mut eei = ExecutionEnvironment {
        memory: [0; 4096],
    };
    // eei.memory[0..4].copy_from_slice(&[0b0_0010011, 0b1_100_0110, 0b0000_0001, 0xF8]); // XORI
    // eei.memory[4..8].copy_from_slice(&[0b1_1101111, 0b0000_0000, 0b000_0_0000, 0b0_0000000]); // JAL 0
    eei.memory[0..4].copy_from_slice(&[0b1_0010011, 0b0_111_0000, 0b000_0_0000, 0b0_0000000]); // ANDI 0
    eei.memory[4..8].copy_from_slice(&[0b1_1101111, 0b1111_0000, 0b110_1_1111, 0b1_1111111]); // JAL -4

    let mut rv32i = RV32I {
        x: [0; 32],
        pc: 0,
        inst: Instruction32::new_empty(ISA::UNKNOWN, 0),
        ext: String::from(""),
        eei: &mut eei,
    };

    for _ in 0..130_000_000 {
        rv32i.single_step();
    }
}
