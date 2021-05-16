use crate::common::{isa::ISA, types::*};

#[derive(Clone, Copy, Debug)]
pub struct Instruction<PC, IMM> {
    pub inst: ISA,
    pub pc: PC,

    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: IMM,
}

pub type Instruction32 = Instruction<u32, i32>;
pub type Instruction64 = Instruction<u64, i64>;

impl<PC: Unsigned, IMM: Signed> Instruction<PC, IMM> {
    pub fn new_empty(inst: ISA, pc: PC) -> Self {
        Self { inst, pc, rd: 0, rs1: 0, rs2: 0, imm: 0.into() }
    }

    pub fn decode_type_fail(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        println!("Bad format {:#X}", opcode);
        Instruction::new_empty(inst, pc)
    }

    pub fn decode_type_empty(inst: ISA, pc: PC, _: u32) -> Instruction<PC, IMM> {
        Instruction::new_empty(inst, pc)
    }

    pub fn decode_type_r(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        Instruction { inst, pc, rd, rs1, rs2, imm: 0.into() }
    }

    pub fn decode_type_i(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20;
        Instruction { inst, pc, rd, rs1, rs2: 0, imm: imm.into() }
    }

    pub fn decode_type_s(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20 & 0xFFFF_FFE0 | opcode as i32 >> 7 & 0b1_1111;
        Instruction { inst, pc, rd: 0, rs1, rs2, imm: imm.into() }
    }

    pub fn decode_type_b(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 19 & 0xFFFF_F000 | (opcode as i32) << 4 & 0x0800 | opcode as i32 >> 20 & 0x07E0 | opcode as i32 >> 7 & 0x001E;
        Instruction { inst, pc, rd: 0, rs1, rs2, imm: imm.into() }
    }

    pub fn decode_type_u(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 & 0xFFFF_F000;
        Instruction { inst, pc, rd, rs1: 0, rs2: 0, imm: imm.into() }
    }

    pub fn decode_type_j(inst: ISA, pc: PC, opcode: u32) -> Instruction<PC, IMM> {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 11 & 0xFFF0_0000 | opcode as i32 & 0xF_F000 | opcode as i32 >> 9 & 0x0800 | opcode as i32 >> 20 & 0x7FE;
        Instruction { inst, pc, rd, rs1: 0, rs2: 0, imm: imm.into() }
    }
}
