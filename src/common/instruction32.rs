use crate::common::isa::ISA;

#[derive(Clone, Copy, Debug)]
pub struct Instruction32 {
    pub inst: ISA,
    pub pc: u32,

    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl Instruction32 {
    pub fn new_empty(inst: ISA, pc: u32) -> Self {
        Self { inst, pc, rd: 0, rs1: 0, rs2: 0, imm: 0 }
    }

    pub fn decode_type_fail(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        println!("Bad format {:#X}", opcode);
        Instruction32::new_empty(inst, pc)
    }

    pub fn decode_type_empty(inst: ISA, pc: u32, _: u32) -> Instruction32 {
        Instruction32::new_empty(inst, pc)
    }

    pub fn decode_type_r(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        Instruction32 { inst, pc, rd, rs1, rs2, imm: 0 }
    }

    pub fn decode_type_i(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20;
        Instruction32 { inst, pc, rd, rs1, rs2: 0, imm }
    }

    pub fn decode_type_s(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 20 & 0xFFFF_FFE0 | opcode as i32 >> 7 & 0b1_1111;
        Instruction32 { inst, pc, rd: 0, rs1, rs2, imm}
    }

    pub fn decode_type_b(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        let rs1 = (opcode >> 15 & 0b1_1111) as u8;
        let rs2 = (opcode >> 20 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 19 & 0xFFFF_F000 | (opcode as i32) << 4 & 0x0800 | opcode as i32 >> 20 & 0x07E0 | opcode as i32 >> 7 & 0x001E;
        Instruction32 { inst, pc, rd: 0, rs1, rs2, imm}
    }

    pub fn decode_type_u(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 & 0xFFFF_F000;
        Instruction32 { inst, pc, rd, rs1: 0, rs2: 0, imm }
    }

    pub fn decode_type_j(inst: ISA, pc: u32, opcode: u32) -> Instruction32 {
        let rd = (opcode >> 7 & 0b1_1111) as u8;
        let imm = opcode as i32 >> 11 & 0xFFF0_0000 | opcode as i32 & 0xF_F000 | opcode as i32 >> 9 & 0x0800 | opcode as i32 >> 20 & 0x7FE;
        Instruction32 { inst, pc, rd, rs1: 0, rs2: 0, imm}
    }
}
