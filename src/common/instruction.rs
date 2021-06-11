use crate::common::{isa::*, decoder::*, types::*};

#[derive(Clone, Copy, Debug)]
pub struct Instruction<U, S> {
    pub inst: ISA,
    pub pc: U,

    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: S,
}

pub type Instruction32 = Instruction<u32, i32>;
pub type Instruction64 = Instruction<u64, i64>;

impl<U: Unsigned<S>, S: Signed<U>> Instruction<U, S> {
    pub fn from_opcode_32(pc: U, opcode: u32) -> Instruction<U, S> {
        let isa = ISA::from_opcode_32(opcode);
        ISA::FORMAT[isa as usize](isa, pc, opcode)
    }

    pub fn new_empty(inst: ISA, pc: U) -> Self {
        Self { inst, pc, rd: 0, rs1: 0, rs2: 0, imm: 0.into() }
    }

    pub fn decode_type_fail(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        println!("Bad format {:#X}", opcode);
        Instruction::new_empty(inst, pc)
    }

    pub fn decode_type_empty(inst: ISA, pc: U, _: u32) -> Instruction<U, S> {
        Instruction::new_empty(inst, pc)
    }

    pub fn decode_type_r(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        Instruction { inst, pc, rd, rs1, rs2, imm: 0.into() }
    }

    pub fn decode_type_i(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20;
        Instruction { inst, pc, rd, rs1, rs2: 0, imm: imm.into() }
    }

    pub fn decode_type_s(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20 & 0xFFFF_FFE0 | opcode as i32 >> 7 & 0b1_1111;
        Instruction { inst, pc, rd: 0, rs1, rs2, imm: imm.into() }
    }

    pub fn decode_type_b(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 19 & 0xFFFF_F000 | (opcode as i32) << 4 & 0x0800 | opcode as i32 >> 20 & 0x07E0 | opcode as i32 >> 7 & 0x001E;
        Instruction { inst, pc, rd: 0, rs1, rs2, imm: imm.into() }
    }

    pub fn decode_type_u(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 & 0xFFFF_F000;
        Instruction { inst, pc, rd, rs1: 0, rs2: 0, imm: imm.into() }
    }

    pub fn decode_type_j(inst: ISA, pc: U, opcode: u32) -> Instruction<U, S> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 11 & 0xFFF0_0000 | opcode as i32 & 0xF_F000 | opcode as i32 >> 9 & 0x0800 | opcode as i32 >> 20 & 0x7FE;
        Instruction { inst, pc, rd, rs1: 0, rs2: 0, imm: imm.into() }
    }
}

pub fn encode_type_r(opcode: u8, rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8) -> u32 {
    (funct7 as u32) << 25 & 0xFE00_0000 | (rs2 as u32) << 20 & 0x1F0_0000 | (rs1 as u32) << 15 & 0xF_8000 |
    (funct3 as u32) << 12 & 0x7000 | (rd as u32) << 7 & 0xF80 | (opcode as u32) & 0x7F
}

pub fn encode_type_i(opcode: u8, rd: u8, funct3: u8, rs1: u8, imm: u32) -> u32 {
    imm << 20 & 0xFFF0_0000 | (rs1 as u32) << 15 & 0x000F_8000 | (funct3 as u32) << 12 & 0x7000 | (rd as u32) << 7 & 0xF80 | opcode as u32 & 0x7F
}

pub fn encode_type_s(opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: u32) -> u32 {
    imm << 20 & 0xFE00_0000 | (rs2 as u32) << 20 & 0x1F0_0000 | (rs1 as u32) << 15 & 0xF_8000 |
    (funct3 as u32) << 12 & 0x7000 | imm << 7 & 0xF80 | (opcode as u32) & 0x7F
}

pub fn encode_type_b(opcode: u8, funct3: u8, rs1: u8, rs2: u8, imm: u32) -> u32 {
    imm << 19 & 0x8000_0000 | imm << 20 & 0x7E00_0000 | (rs2 as u32) << 20 & 0x1F0_0000 | (rs1 as u32) << 15 & 0xF_8000 |
    (funct3 as u32) << 12 & 0x7000 | imm << 7 & 0xF00 | imm >> 4 & 0x80 | (opcode as u32) & 0x7F
}

pub fn encode_type_u(opcode: u8, rd: u8, imm: u32) -> u32 {
    (imm & 0xFFFF_F000) | (rd as u32) << 7 & 0xF80 | opcode as u32 & 0x7F
}

pub fn encode_type_j(opcode: u8, rd: u8, imm: u32) -> u32 {
    (imm << 11 & 0x8000_0000) | (imm << 20 & 0x7FE0_0000) | (imm << 9 & 0x0010_0000) | (imm & 0x000F_F000) | (rd as u32) << 7 & 0x0F80 | opcode as u32 & 0x7F
}
