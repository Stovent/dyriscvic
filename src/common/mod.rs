pub mod memory;

/// Returns the width of the instruction word in bytes, 24 if greater than 192 bits.
pub fn get_instruction_length(inst: u16) -> u16 {
    if (inst & 0b11) != 0b11 {
        return 2;
    }
    if (inst & 0b1_1111) != 0b1_1111 {
        return 4;
    }
    if (inst & 0b11_1111) != 0b01_1111 {
        return 6;
    }
    if (inst & 0b111_1111) != 0b011_1111 {
        return 8;
    }
    if (inst & 0b0111_0000_0111_1111) != 0b0111_0000_0111_1111 {
        let nnn = inst >> 12 & 0b111;
        return 10 + 2 * nnn;
    }
    return 24;
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl Instruction {
    pub fn new_empty() -> Self {
        Self { rd: 0, rs1: 0, rs2: 0, imm: 0 }
    }
}

pub fn decode_type_fail(opcode: u32) -> Instruction {
    println!("Bad format {:#X}", opcode);
    Instruction::new_empty()
}

pub fn decode_type_empty(_: u32) -> Instruction {
    Instruction::new_empty()
}

pub fn decode_type_r(opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as u8;
    let rs1 = (opcode >> 15 & 0b1_1111) as u8;
    let rs2 = (opcode >> 20 & 0b1_1111) as u8;
    Instruction { rd, rs1, rs2, imm: 0 }
}

pub fn decode_type_i(opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as u8;
    let rs1 = (opcode >> 15 & 0b1_1111) as u8;
    let imm = opcode as i32 >> 20;
    Instruction { rd, rs1, rs2: 0, imm }
}

pub fn decode_type_s(opcode: u32) -> Instruction {
    let rs1 = (opcode >> 15 & 0b1_1111) as u8;
    let rs2 = (opcode >> 20 & 0b1_1111) as u8;
    let imm = opcode as i32 >> 20 | opcode as i32 >> 7 & 0b1_1111;
    Instruction { rd: 0, rs1, rs2, imm}
}

pub fn decode_type_b(opcode: u32) -> Instruction {
    let rs1 = (opcode >> 15 & 0b1_1111) as u8;
    let rs2 = (opcode >> 20 & 0b1_1111) as u8;
    let imm = opcode as i32 >> 19 & 0x1000 | (opcode as i32) << 4 & 0x0800 | opcode as i32 >> 20 & 0x07E0 | opcode as i32 >> 7 & 0x001E;
    Instruction { rd: 0, rs1, rs2, imm}
}

pub fn decode_type_u(opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as u8;
    let imm = opcode as i32 & 0xFFFF_F000;
    Instruction { rd, rs1: 0, rs2: 0, imm }
}

pub fn decode_type_j(opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as u8;
    let imm = opcode as i32 >> 11 & 0x10_0000 | opcode as i32 & 0xF_F000 | opcode as i32 >> 9 & 0x0800 | opcode as i32 >> 20 & 0x7FE;
    Instruction { rd, rs1: 0, rs2: 0, imm}
}
