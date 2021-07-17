#![allow(overflowing_literals)]

use dyriscvic::common::instruction::*;
use dyriscvic::common::isa::*;

fn type_r(inst: &Instruction64, isa: ISA, rs2: u8, rs1: u8, rd: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.rs2, rs2, "rs2 {} {}", inst.rs2, rs2);
    assert_eq!(inst.rs1, rs1, "rs1 {} {}", inst.rs1, rs1);
    assert_eq!(inst.rd, rd, "rd {} {}", inst.rd, rd);
}

fn type_i(inst: &Instruction64, isa: ISA, imm: i64, rs1: u8, rd: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.imm, imm, "imm {} {}", inst.imm, imm);
    assert_eq!(inst.rs1, rs1, "rs1 {} {}", inst.rs1, rs1);
    assert_eq!(inst.rd, rd, "rd {} {}", inst.rd, rd);
}

fn type_s_b(inst: &Instruction64, isa: ISA, imm: i64, rs2: u8, rs1: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.imm, imm, "imm {} {}", inst.imm, imm);
    assert_eq!(inst.rs2, rs2, "rs2 {} {}", inst.rs2, rs2);
    assert_eq!(inst.rs1, rs1, "rs1 {} {}", inst.rs1, rs1);
}

fn type_u_j(inst: &Instruction64, isa: ISA, imm: i64, rd: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.imm, imm, "imm {} {}", inst.imm, imm);
    assert_eq!(inst.rd, rd, "rd {} {}", inst.rd, rd);
}

#[test]
fn decode_i32() {
    let add = 0b0000000_11100_00011_000_11111_0110011u32;
    let add_ = Instruction64::from_opcode_32(0, add);
    type_r(&add_, ISA::ADD, 28, 3, 31);

    let addi = 0b111000111000_01101_000_01110_0010011u32;
    let addi_ = Instruction64::from_opcode_32(0, addi);
    type_i(&addi_, ISA::ADDI, -456, 13, 14);

    let and = 0b0000000_11100_00011_111_11111_0110011u32;
    let and_ = Instruction64::from_opcode_32(0, and);
    type_r(&and_, ISA::AND, 28, 3, 31);

    let andi = 0b111000111000_01101_111_01110_0010011u32;
    let andi_ = Instruction64::from_opcode_32(0, andi);
    type_i(&andi_, ISA::ANDI, -456, 13, 14);

    let auipc = 0b10010101101010110101_11100_0010111u32;
    let auipc_ = Instruction64::from_opcode_32(0, auipc);
    type_u_j(&auipc_, ISA::AUIPC, -1_783_934_976, 28);

    let beq = 0b1000111_00100_00110_000_11011_1100011u32;
    let beq_ = Instruction64::from_opcode_32(0, beq);
    type_s_b(&beq_, ISA::BEQ, -1798, 4, 6);

    let bne = 0b1000111_00100_00110_001_11011_1100011u32;
    let bne_ = Instruction64::from_opcode_32(0, bne);
    type_s_b(&bne_, ISA::BNE, -1798, 4, 6);

    let blt = 0b1000111_00100_00110_100_11011_1100011u32;
    let blt_ = Instruction64::from_opcode_32(0, blt);
    type_s_b(&blt_, ISA::BLT, -1798, 4, 6);

    let bge = 0b1000111_00100_00110_101_11011_1100011u32;
    let bge_ = Instruction64::from_opcode_32(0, bge);
    type_s_b(&bge_, ISA::BGE, -1798, 4, 6);

    let bltu = 0b1000111_00100_00110_110_11011_1100011u32;
    let bltu_ = Instruction64::from_opcode_32(0, bltu);
    type_s_b(&bltu_, ISA::BLTU, -1798, 4, 6);

    let bgeu = 0b1000111_00100_00110_111_11011_1100011u32;
    let bgeu_ = Instruction64::from_opcode_32(0, bgeu);
    type_s_b(&bgeu_, ISA::BGEU, -1798, 4, 6);

    let ebreak = 0b000000000001_00000_000_00000_1110011u32;
    let ebreak_ = Instruction64::from_opcode_32(0, ebreak);
    assert_eq!(ebreak_.inst, ISA::EBREAK);

    let ecall = 0b000000000000_00000_000_00000_1110011u32;
    let ecall_ = Instruction64::from_opcode_32(0, ecall);
    assert_eq!(ecall_.inst, ISA::ECALL);

    let fence = 0b1010_0101_0110_11001_000_01111_0001111u32;
    let fence_ = Instruction64::from_opcode_32(0, fence);
    type_i(&fence_, ISA::FENCE, 0b1111_1010_0101_0110i16 as i64, 25, 15);

    let lui = 0b11101000101000111101_11011_0110111u32;
    let lui_ = Instruction64::from_opcode_32(0, lui);
    type_u_j(&lui_, ISA::LUI, -391_917_568, 27);

    let jal = 0b1_0111111000_0_10010111_11111_1101111u32;
    let jal_ = Instruction64::from_opcode_32(0, jal);
    type_u_j(&jal_, ISA::JAL, -429_072, 31);

    let jalr = 0b110011001100_10101_000_01010_1100111u32;
    let jalr_ = Instruction64::from_opcode_32(0, jalr);
    type_i(&jalr_, ISA::JALR, -820, 21, 10);

    let lb = 0b100011100110_11000_000_11001_0000011u32;
    let lb_ = Instruction64::from_opcode_32(0, lb);
    type_i(&lb_, ISA::LB, -1818, 24, 25);

    let lbu = 0b100011100110_11000_100_11001_0000011u32;
    let lbu_ = Instruction64::from_opcode_32(0, lbu);
    type_i(&lbu_, ISA::LBU, -1818, 24, 25);

    let lh = 0b100011100110_11000_001_11001_0000011u32;
    let lh_ = Instruction64::from_opcode_32(0, lh);
    type_i(&lh_, ISA::LH, -1818, 24, 25);

    let lhu = 0b100011100110_11000_101_11001_0000011u32;
    let lhu_ = Instruction64::from_opcode_32(0, lhu);
    type_i(&lhu_, ISA::LHU, -1818, 24, 25);

    let lw = 0b100011100110_11000_010_11001_0000011u32;
    let lw_ = Instruction64::from_opcode_32(0, lw);
    type_i(&lw_, ISA::LW, -1818, 24, 25);

    let or = 0b0000000_11100_00011_110_11111_0110011u32;
    let or_ = Instruction64::from_opcode_32(0, or);
    type_r(&or_, ISA::OR, 28, 3, 31);

    let ori = 0b111000111000_01101_110_01110_0010011u32;
    let ori_ = Instruction64::from_opcode_32(0, ori);
    type_i(&ori_, ISA::ORI, -456, 13, 14);

    let sb = 0b1110000_10011_00110_000_00111_0100011u32;
    let sb_ = Instruction64::from_opcode_32(0, sb);
    type_s_b(&sb_, ISA::SB, -505, 19, 6);

    let sh = 0b1110010_11110_11101_001_11110_0100011u32;
    let sh_ = Instruction64::from_opcode_32(0, sh);
    type_s_b(&sh_, ISA::SH, -418, 30, 29);

    let sll = 0b0000000_11100_00011_001_11111_0110011u32;
    let sll_ = Instruction64::from_opcode_32(0, sll);
    type_r(&sll_, ISA::SLL, 28, 3, 31);

    let slli = 0b0000000_01011_01001_001_10010_0010011u32;
    let slli_ = Instruction64::from_opcode_32(0, slli);
    type_i(&slli_, ISA::SLLI, 11, 9, 18);

    let slt = 0b0000000_11100_00011_010_11111_0110011u32;
    let slt_ = Instruction64::from_opcode_32(0, slt);
    type_r(&slt_, ISA::SLT, 28, 3, 31);

    let slti = 0b111000111000_01101_010_01110_0010011u32;
    let slti_ = Instruction64::from_opcode_32(0, slti);
    type_i(&slti_, ISA::SLTI, -456, 13, 14);

    let sltiu = 0b111111111111_01101_011_01110_0010011u32;
    let sltiu_ = Instruction64::from_opcode_32(0, sltiu);
    type_i(&sltiu_, ISA::SLTIU, -1, 13, 14);

    let sltu = 0b0000000_11100_00011_011_11111_0110011u32;
    let sltu_ = Instruction64::from_opcode_32(0, sltu);
    type_r(&sltu_, ISA::SLTU, 28, 3, 31);

    let sra = 0b0100000_11100_00011_101_11111_0110011u32;
    let sra_ = Instruction64::from_opcode_32(0, sra);
    type_r(&sra_, ISA::SRA, 28, 3, 31);

    let srai = 0b0100000_01011_01001_101_10010_0010011u32;
    let srai_ = Instruction64::from_opcode_32(0, srai);
    type_i(&srai_, ISA::SRAI, 1035, 9, 18);

    let srl = 0b0000000_11100_00011_101_11111_0110011u32;
    let srl_ = Instruction64::from_opcode_32(0, srl);
    type_r(&srl_, ISA::SRL, 28, 3, 31);

    let srli = 0b0000000_01011_01001_101_10010_0010011u32;
    let srli_ = Instruction64::from_opcode_32(0, srli);
    type_i(&srli_, ISA::SRLI, 11, 9, 18);

    let sub = 0b0100000_11100_00011_000_11111_0110011u32;
    let sub_ = Instruction64::from_opcode_32(0, sub);
    type_r(&sub_, ISA::SUB, 28, 3, 31);

    let sw = 0b0101110_01011_01110_010_11011_0100011u32;
    let sw_ = Instruction64::from_opcode_32(0, sw);
    type_s_b(&sw_, ISA::SW, 1499, 11, 14);

    let xor = 0b0000000_11100_00011_100_11111_0110011u32;
    let xor_ = Instruction64::from_opcode_32(0, xor);
    type_r(&xor_, ISA::XOR, 28, 3, 31);

    let xori = 0b111111000000_01101_100_01110_0010011u32;
    let xori_ = Instruction64::from_opcode_32(0, xori);
    type_i(&xori_, ISA::XORI, -64, 13, 14);
}

#[test]
fn decode_i64() {
    let addiw = 0b110000111110_00101_000_00100_0011011u32;
    let addiw_ = Instruction64::from_opcode_32(0, addiw);
    type_i(&addiw_, ISA::ADDIW, -962, 5, 4);

    let addw = 0b0000000_10110_10001_000_10011_0111011u32;
    let addw_ = Instruction64::from_opcode_32(0, addw);
    type_r(&addw_, ISA::ADDW, 22, 17, 19);

    let ld = 0b110001110011_11000_011_00001_0000011u32;
    let ld_ = Instruction64::from_opcode_32(0, ld);
    type_i(&ld_, ISA::LD, -909, 24, 1);

    let lwu = 0b110001110011_11000_110_00001_0000011u32;
    let lwu_ = Instruction64::from_opcode_32(0, lwu);
    type_i(&lwu_, ISA::LWU, -909, 24, 1);

    let slli = 0b000000_111000_00000_001_11000_0010011u32;
    let slli_ = Instruction64::from_opcode_32(0, slli);
    type_i(&slli_, ISA::SLLI, 56, 0, 24);

    let slliw = 0b0000000_11110_00101_001_00100_0011011u32;
    let slliw_ = Instruction64::from_opcode_32(0, slliw);
    type_i(&slliw_, ISA::SLLIW, 30, 5, 4);

    let sllw = 0b0000000_10110_10001_001_10011_0111011u32;
    let sllw_ = Instruction64::from_opcode_32(0, sllw);
    type_r(&sllw_, ISA::SLLW, 22, 17, 19);

    let srai = 0b010000_111100_00000_101_01000_0010011u32;
    let srai_ = Instruction64::from_opcode_32(0, srai);
    type_i(&srai_, ISA::SRAI, 1084, 0, 8);

    let sraiw = 0b0100000_11010_00101_101_00100_0011011u32;
    let sraiw_ = Instruction64::from_opcode_32(0, sraiw);
    type_i(&sraiw_, ISA::SRAIW, 1050, 5, 4);

    let sraw = 0b0100000_10110_10001_101_10011_0111011u32;
    let sraw_ = Instruction64::from_opcode_32(0, sraw);
    type_r(&sraw_, ISA::SRAW, 22, 17, 19);

    let srli = 0b000000_111110_00000_101_11000_0010011u32;
    let srli_ = Instruction64::from_opcode_32(0, srli);
    type_i(&srli_, ISA::SRLI, 62, 0, 24);

    let srliw = 0b0000000_11111_00101_101_00100_0011011u32;
    let srliw_ = Instruction64::from_opcode_32(0, srliw);
    type_i(&srliw_, ISA::SRLIW, 31, 5, 4);

    let srlw = 0b0000000_10110_10001_101_10011_0111011u32;
    let srlw_ = Instruction64::from_opcode_32(0, srlw);
    type_r(&srlw_, ISA::SRLW, 22, 17, 19);

    let sd = 0b1100001_11100_10111_011_11111_0100011u32;
    let sd_ = Instruction64::from_opcode_32(0, sd);
    type_s_b(&sd_, ISA::SD, -961, 28, 23);

    let subw = 0b0100000_10110_10001_000_10011_0111011u32;
    let subw_ = Instruction64::from_opcode_32(0, subw);
    type_r(&subw_, ISA::SUBW, 22, 17, 19);
}
