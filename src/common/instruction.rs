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

    pub fn from_opcode_32(pc: U, opcode: u32) -> Instruction<U, S> {
        let isa = ISA::from_opcode_32(opcode);
        ISA::FORMAT[isa as usize](isa, pc, opcode)
    }
}
