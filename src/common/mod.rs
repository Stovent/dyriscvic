/// Returns the width of the instruction word in bytes, 24 if greater than 192 bits.
pub fn get_instruction_length(inst: u16) -> u16 {
    if (inst & 0b11) != 0b11 {
        2
    } else if (inst & 0b1_1111) != 0b1_1111 {
        4
    } else if (inst & 0b11_1111) != 0b01_1111 {
        6
    } else if (inst & 0b111_1111) != 0b011_1111 {
        8
    } else if (inst & 0b0111_0000_0111_1111) != 0b0111_0000_0111_1111 {
        let nnn = inst >> 12 & 0b111;
        10 + 2 * nnn
    } else {
        24
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub pc: u32,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub imm: i32,
}

impl Instruction {
    pub fn new_empty(pc: u32) -> Self {
        Self { pc, rd: 0, rs1: 0, rs2: 0, imm: 0 }
    }
}

pub fn decode_type_fail(pc: u32, opcode: u32) -> Instruction {
    println!("Bad format {:#X}", opcode);
    Instruction::new_empty(pc)
}

pub fn decode_type_empty(pc: u32, _: u32) -> Instruction {
    Instruction::new_empty(pc)
}

pub fn decode_type_r(pc: u32, opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as usize;
    let rs1 = (opcode >> 15 & 0b1_1111) as usize;
    let rs2 = (opcode >> 20 & 0b1_1111) as usize;
    Instruction { pc, rd, rs1, rs2, imm: 0 }
}

pub fn decode_type_i(pc: u32, opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as usize;
    let rs1 = (opcode >> 15 & 0b1_1111) as usize;
    let imm = opcode as i32 >> 20;
    Instruction { pc, rd, rs1, rs2: 0, imm }
}

pub fn decode_type_s(pc: u32, opcode: u32) -> Instruction {
    let rs1 = (opcode >> 15 & 0b1_1111) as usize;
    let rs2 = (opcode >> 20 & 0b1_1111) as usize;
    let imm = opcode as i32 >> 20 & 0xFFFF_FFE0 | opcode as i32 >> 7 & 0b1_1111;
    Instruction { pc, rd: 0, rs1, rs2, imm}
}

pub fn decode_type_b(pc: u32, opcode: u32) -> Instruction {
    let rs1 = (opcode >> 15 & 0b1_1111) as usize;
    let rs2 = (opcode >> 20 & 0b1_1111) as usize;
    let imm = opcode as i32 >> 19 & 0x1000 | (opcode as i32) << 4 & 0x0800 | opcode as i32 >> 20 & 0x07E0 | opcode as i32 >> 7 & 0x001E;
    Instruction { pc, rd: 0, rs1, rs2, imm}
}

pub fn decode_type_u(pc: u32, opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as usize;
    let imm = opcode as i32 & 0xFFFF_F000;
    Instruction { pc, rd, rs1: 0, rs2: 0, imm }
}

pub fn decode_type_j(pc: u32, opcode: u32) -> Instruction {
    let rd = (opcode >> 7 & 0b1_1111) as usize;
    let imm = opcode as i32 >> 11 & 0xFFF0_0000 | opcode as i32 & 0xF_F000 | opcode as i32 >> 9 & 0x0800 | opcode as i32 >> 20 & 0x7FE;
    Instruction { pc, rd, rs1: 0, rs2: 0, imm}
}
