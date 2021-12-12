use crate::common::get_x_register_name;
use crate::common::isa::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub inst: Isa,
    pub pc: u64,

    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl Instruction {
    pub const fn empty(inst: Isa, pc: u64, _: u32) -> Self {
        Self { inst, pc, rd: 0, rs1: 0, rs2: 0, imm: 0 }
    }

    pub const fn decode_type_r(inst: Isa, pc: u64, opcode: u32) -> Self {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        Instruction { inst, pc, rd, rs1, rs2, imm: 0 }
    }

    pub const fn decode_type_i(inst: Isa, pc: u64, opcode: u32) -> Self {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20;
        Instruction { inst, pc, rd, rs1, rs2: 0, imm }
    }

    pub const fn decode_type_s(inst: Isa, pc: u64, opcode: u32) -> Self {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20 & 0xFFFF_FFE0 | opcode as i32 >> 7 & 0b1_1111;
        Instruction { inst, pc, rd: 0, rs1, rs2, imm }
    }

    pub const fn decode_type_b(inst: Isa, pc: u64, opcode: u32) -> Self {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 19 & 0xFFFF_F000 | (opcode as i32) << 4 & 0x0800 | opcode as i32 >> 20 & 0x07E0 | opcode as i32 >> 7 & 0x001E;
        Instruction { inst, pc, rd: 0, rs1, rs2, imm }
    }

    pub const fn decode_type_u(inst: Isa, pc: u64, opcode: u32) -> Self {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 & 0xFFFF_F000;
        Instruction { inst, pc, rd, rs1: 0, rs2: 0, imm }
    }

    pub const fn decode_type_j(inst: Isa, pc: u64, opcode: u32) -> Self {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 11 & 0xFFF0_0000 | opcode as i32 & 0xF_F000 | opcode as i32 >> 9 & 0x0800 | opcode as i32 >> 20 & 0x7FE;
        Instruction { inst, pc, rd, rs1: 0, rs2: 0, imm }
    }

    pub const fn encode_type_r(opcode: u8, rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8) -> u32 {
        (funct7 as u32) << 25 & 0xFE00_0000 | (rs2 as u32) << 20 & 0x1F0_0000 | (rs1 as u32) << 15 & 0xF_8000 |
        (funct3 as u32) << 12 & 0x7000 | (rd as u32) << 7 & 0xF80 | (opcode as u32) & 0x7F
    }

    pub const fn encode_type_i(opcode: u8, rd: u8, funct3: u8, rs1: u8, imm: u32) -> u32 {
        imm << 20 & 0xFFF0_0000 | (rs1 as u32) << 15 & 0x000F_8000 | (funct3 as u32) << 12 & 0x7000 | (rd as u32) << 7 & 0xF80 | opcode as u32 & 0x7F
    }

    pub const fn encode_type_s(opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: u32) -> u32 {
        imm << 20 & 0xFE00_0000 | (rs2 as u32) << 20 & 0x1F0_0000 | (rs1 as u32) << 15 & 0xF_8000 |
        (funct3 as u32) << 12 & 0x7000 | imm << 7 & 0xF80 | (opcode as u32) & 0x7F
    }

    pub const fn encode_type_b(opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: u32) -> u32 {
        imm << 19 & 0x8000_0000 | imm << 20 & 0x7E00_0000 | (rs2 as u32) << 20 & 0x1F0_0000 | (rs1 as u32) << 15 & 0xF_8000 |
        (funct3 as u32) << 12 & 0x7000 | imm << 7 & 0xF00 | imm >> 4 & 0x80 | (opcode as u32) & 0x7F
    }

    pub const fn encode_type_u(opcode: u8, rd: u8, imm: u32) -> u32 {
        (imm & 0xFFFF_F000) | (rd as u32) << 7 & 0xF80 | opcode as u32 & 0x7F
    }

    pub const fn encode_type_j(opcode: u8, rd: u8, imm: u32) -> u32 {
        (imm << 11 & 0x8000_0000) | (imm << 20 & 0x7FE0_0000) | (imm << 9 & 0x0010_0000) | (imm & 0x000F_F000) | (rd as u32) << 7 & 0x0F80 | opcode as u32 & 0x7F
    }

    pub fn disassemble_type_r(self, abi_name: bool) -> String {
        format!("{}, {}, {}", get_x_register_name(self.rd, abi_name), get_x_register_name(self.rs1, abi_name), get_x_register_name(self.rs2, abi_name))
    }

    pub fn disassemble_type_i(self, abi_name: bool) -> String {
        format!("{}, {}, {}", get_x_register_name(self.rd, abi_name), get_x_register_name(self.rs1, abi_name), self.imm)
    }

    pub fn disassemble_type_s(self, abi_name: bool) -> String {
        format!("{}, {}, {}", get_x_register_name(self.rs2, abi_name), get_x_register_name(self.rs1, abi_name), self.imm)
    }

    pub fn disassemble_type_b(self, abi_name: bool) -> String {
        format!("{}, {}, {}", get_x_register_name(self.rs1, abi_name), get_x_register_name(self.rs2, abi_name), self.imm)
    }

    pub fn disassemble_type_u_j(self, abi_name: bool) -> String {
        format!("{}, {}", get_x_register_name(self.rd, abi_name), self.imm)
    }
}
