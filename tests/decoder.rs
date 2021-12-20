#![allow(overflowing_literals)]

use dyriscvic::common::instruction::*;
use dyriscvic::common::isa::*;
use dyriscvic::rvi::RV32I;
use dyriscvic::public::{ExecutionEnvironmentInterface, MemoryAccess, Trap};

fn type_r(inst: &Instruction32, isa: Isa, rs2: u8, rs1: u8, rd: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.rs2, rs2, "rs2 {} {}", inst.rs2, rs2);
    assert_eq!(inst.rs1, rs1, "rs1 {} {}", inst.rs1, rs1);
    assert_eq!(inst.rd, rd, "rd {} {}", inst.rd, rd);
}

fn type_i(inst: &Instruction32, isa: Isa, imm: i64, rs1: u8, rd: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.imm as i64, imm, "imm {} {}", inst.imm, imm);
    assert_eq!(inst.rs1, rs1, "rs1 {} {}", inst.rs1, rs1);
    assert_eq!(inst.rd, rd, "rd {} {}", inst.rd, rd);
}

fn type_s_b(inst: &Instruction32, isa: Isa, imm: i64, rs2: u8, rs1: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.imm as i64, imm, "imm {} {}", inst.imm, imm);
    assert_eq!(inst.rs2, rs2, "rs2 {} {}", inst.rs2, rs2);
    assert_eq!(inst.rs1, rs1, "rs1 {} {}", inst.rs1, rs1);
}

fn type_u_j(inst: &Instruction32, isa: Isa, imm: i64, rd: u8) {
    assert_eq!(inst.inst, isa, "ISA {:?} {:?}", inst.inst, isa);
    assert_eq!(inst.imm as i64, imm, "imm {} {}", inst.imm, imm);
    assert_eq!(inst.rd, rd, "rd {} {}", inst.rd, rd);
}

fn from_opcode_32<EEI: ExecutionEnvironmentInterface>(opcode: u32) -> Instruction32 {
    let isa = Isa::from_opcode_32(opcode);
    let entry = &RV32I::<EEI>::ISA_LUT[isa as usize];
    (entry.decoder)(isa, 0, opcode)
}

#[test]
fn decode_i32() {
    let add = 0b0000000_11100_00011_000_11111_0110011;
    let add_ = from_opcode_32::<ExecutionEnvironment>(add);
    type_r(&add_, Isa::ADD, 28, 3, 31);

    let addi = 0b111000111000_01101_000_01110_0010011u32;
    let addi_ = from_opcode_32::<ExecutionEnvironment>(addi);
    type_i(&addi_, Isa::ADDI, -456, 13, 14);

    let and = 0b0000000_11100_00011_111_11111_0110011u32;
    let and_ = from_opcode_32::<ExecutionEnvironment>(and);
    type_r(&and_, Isa::AND, 28, 3, 31);

    let andi = 0b111000111000_01101_111_01110_0010011u32;
    let andi_ = from_opcode_32::<ExecutionEnvironment>(andi);
    type_i(&andi_, Isa::ANDI, -456, 13, 14);

    let auipc = 0b10010101101010110101_11100_0010111u32;
    let auipc_ = from_opcode_32::<ExecutionEnvironment>(auipc);
    type_u_j(&auipc_, Isa::AUIPC, -1_783_934_976, 28);

    let beq = 0b1000111_00100_00110_000_11011_1100011u32;
    let beq_ = from_opcode_32::<ExecutionEnvironment>(beq);
    type_s_b(&beq_, Isa::BEQ, -1798, 4, 6);

    let bne = 0b1000111_00100_00110_001_11011_1100011u32;
    let bne_ = from_opcode_32::<ExecutionEnvironment>(bne);
    type_s_b(&bne_, Isa::BNE, -1798, 4, 6);

    let blt = 0b1000111_00100_00110_100_11011_1100011u32;
    let blt_ = from_opcode_32::<ExecutionEnvironment>(blt);
    type_s_b(&blt_, Isa::BLT, -1798, 4, 6);

    let bge = 0b1000111_00100_00110_101_11011_1100011u32;
    let bge_ = from_opcode_32::<ExecutionEnvironment>(bge);
    type_s_b(&bge_, Isa::BGE, -1798, 4, 6);

    let bltu = 0b1000111_00100_00110_110_11011_1100011u32;
    let bltu_ = from_opcode_32::<ExecutionEnvironment>(bltu);
    type_s_b(&bltu_, Isa::BLTU, -1798, 4, 6);

    let bgeu = 0b1000111_00100_00110_111_11011_1100011u32;
    let bgeu_ = from_opcode_32::<ExecutionEnvironment>(bgeu);
    type_s_b(&bgeu_, Isa::BGEU, -1798, 4, 6);

    let ebreak = 0b000000000001_00000_000_00000_1110011u32;
    let ebreak_ = from_opcode_32::<ExecutionEnvironment>(ebreak);
    assert_eq!(ebreak_.inst, Isa::EBREAK);

    let ecall = 0b000000000000_00000_000_00000_1110011u32;
    let ecall_ = from_opcode_32::<ExecutionEnvironment>(ecall);
    assert_eq!(ecall_.inst, Isa::ECALL);

    let fence = 0b1010_0101_0110_11001_000_01111_0001111u32;
    let fence_ = from_opcode_32::<ExecutionEnvironment>(fence);
    type_i(&fence_, Isa::FENCE, 0b1111_1010_0101_0110i16 as i64, 25, 15);

    let lui = 0b11101000101000111101_11011_0110111u32;
    let lui_ = from_opcode_32::<ExecutionEnvironment>(lui);
    type_u_j(&lui_, Isa::LUI, -391_917_568, 27);

    let jal = 0b1_0111111000_0_10010111_11111_1101111u32;
    let jal_ = from_opcode_32::<ExecutionEnvironment>(jal);
    type_u_j(&jal_, Isa::JAL, -429_072, 31);

    let jalr = 0b110011001100_10101_000_01010_1100111u32;
    let jalr_ = from_opcode_32::<ExecutionEnvironment>(jalr);
    type_i(&jalr_, Isa::JALR, -820, 21, 10);

    let lb = 0b100011100110_11000_000_11001_0000011u32;
    let lb_ = from_opcode_32::<ExecutionEnvironment>(lb);
    type_i(&lb_, Isa::LB, -1818, 24, 25);

    let lbu = 0b100011100110_11000_100_11001_0000011u32;
    let lbu_ = from_opcode_32::<ExecutionEnvironment>(lbu);
    type_i(&lbu_, Isa::LBU, -1818, 24, 25);

    let lh = 0b100011100110_11000_001_11001_0000011u32;
    let lh_ = from_opcode_32::<ExecutionEnvironment>(lh);
    type_i(&lh_, Isa::LH, -1818, 24, 25);

    let lhu = 0b100011100110_11000_101_11001_0000011u32;
    let lhu_ = from_opcode_32::<ExecutionEnvironment>(lhu);
    type_i(&lhu_, Isa::LHU, -1818, 24, 25);

    let lw = 0b100011100110_11000_010_11001_0000011u32;
    let lw_ = from_opcode_32::<ExecutionEnvironment>(lw);
    type_i(&lw_, Isa::LW, -1818, 24, 25);

    let or = 0b0000000_11100_00011_110_11111_0110011u32;
    let or_ = from_opcode_32::<ExecutionEnvironment>(or);
    type_r(&or_, Isa::OR, 28, 3, 31);

    let ori = 0b111000111000_01101_110_01110_0010011u32;
    let ori_ = from_opcode_32::<ExecutionEnvironment>(ori);
    type_i(&ori_, Isa::ORI, -456, 13, 14);

    let sb = 0b1110000_10011_00110_000_00111_0100011u32;
    let sb_ = from_opcode_32::<ExecutionEnvironment>(sb);
    type_s_b(&sb_, Isa::SB, -505, 19, 6);

    let sh = 0b1110010_11110_11101_001_11110_0100011u32;
    let sh_ = from_opcode_32::<ExecutionEnvironment>(sh);
    type_s_b(&sh_, Isa::SH, -418, 30, 29);

    let sll = 0b0000000_11100_00011_001_11111_0110011u32;
    let sll_ = from_opcode_32::<ExecutionEnvironment>(sll);
    type_r(&sll_, Isa::SLL, 28, 3, 31);

    let slli = 0b0000000_01011_01001_001_10010_0010011u32;
    let slli_ = from_opcode_32::<ExecutionEnvironment>(slli);
    type_i(&slli_, Isa::SLLI, 11, 9, 18);

    let slt = 0b0000000_11100_00011_010_11111_0110011u32;
    let slt_ = from_opcode_32::<ExecutionEnvironment>(slt);
    type_r(&slt_, Isa::SLT, 28, 3, 31);

    let slti = 0b111000111000_01101_010_01110_0010011u32;
    let slti_ = from_opcode_32::<ExecutionEnvironment>(slti);
    type_i(&slti_, Isa::SLTI, -456, 13, 14);

    let sltiu = 0b111111111111_01101_011_01110_0010011u32;
    let sltiu_ = from_opcode_32::<ExecutionEnvironment>(sltiu);
    type_i(&sltiu_, Isa::SLTIU, -1, 13, 14);

    let sltu = 0b0000000_11100_00011_011_11111_0110011u32;
    let sltu_ = from_opcode_32::<ExecutionEnvironment>(sltu);
    type_r(&sltu_, Isa::SLTU, 28, 3, 31);

    let sra = 0b0100000_11100_00011_101_11111_0110011u32;
    let sra_ = from_opcode_32::<ExecutionEnvironment>(sra);
    type_r(&sra_, Isa::SRA, 28, 3, 31);

    let srai = 0b0100000_01011_01001_101_10010_0010011u32;
    let srai_ = from_opcode_32::<ExecutionEnvironment>(srai);
    type_i(&srai_, Isa::SRAI, 1035, 9, 18);

    let srl = 0b0000000_11100_00011_101_11111_0110011u32;
    let srl_ = from_opcode_32::<ExecutionEnvironment>(srl);
    type_r(&srl_, Isa::SRL, 28, 3, 31);

    let srli = 0b0000000_01011_01001_101_10010_0010011u32;
    let srli_ = from_opcode_32::<ExecutionEnvironment>(srli);
    type_i(&srli_, Isa::SRLI, 11, 9, 18);

    let sub = 0b0100000_11100_00011_000_11111_0110011u32;
    let sub_ = from_opcode_32::<ExecutionEnvironment>(sub);
    type_r(&sub_, Isa::SUB, 28, 3, 31);

    let sw = 0b0101110_01011_01110_010_11011_0100011u32;
    let sw_ = from_opcode_32::<ExecutionEnvironment>(sw);
    type_s_b(&sw_, Isa::SW, 1499, 11, 14);

    let xor = 0b0000000_11100_00011_100_11111_0110011u32;
    let xor_ = from_opcode_32::<ExecutionEnvironment>(xor);
    type_r(&xor_, Isa::XOR, 28, 3, 31);

    let xori = 0b111111000000_01101_100_01110_0010011u32;
    let xori_ = from_opcode_32::<ExecutionEnvironment>(xori);
    type_i(&xori_, Isa::XORI, -64, 13, 14);
}

#[test]
fn decode_i64() {/*
    let addiw = 0b110000111110_00101_000_00100_0011011u32;
    let addiw_ = from_opcode_32::<ExecutionEnvironment>(addiw);
    type_i(&addiw_, Isa::ADDIW, -962, 5, 4);

    let addw = 0b0000000_10110_10001_000_10011_0111011u32;
    let addw_ = from_opcode_32::<ExecutionEnvironment>(addw);
    type_r(&addw_, Isa::ADDW, 22, 17, 19);

    let ld = 0b110001110011_11000_011_00001_0000011u32;
    let ld_ = from_opcode_32::<ExecutionEnvironment>(ld);
    type_i(&ld_, Isa::LD, -909, 24, 1);

    let lwu = 0b110001110011_11000_110_00001_0000011u32;
    let lwu_ = from_opcode_32::<ExecutionEnvironment>(lwu);
    type_i(&lwu_, Isa::LWU, -909, 24, 1);

    let slli = 0b000000_111000_00000_001_11000_0010011u32;
    let slli_ = from_opcode_32::<ExecutionEnvironment>(slli);
    type_i(&slli_, Isa::SLLI, 56, 0, 24);

    let slliw = 0b0000000_11110_00101_001_00100_0011011u32;
    let slliw_ = from_opcode_32::<ExecutionEnvironment>(slliw);
    type_i(&slliw_, Isa::SLLIW, 30, 5, 4);

    let sllw = 0b0000000_10110_10001_001_10011_0111011u32;
    let sllw_ = from_opcode_32::<ExecutionEnvironment>(sllw);
    type_r(&sllw_, Isa::SLLW, 22, 17, 19);

    let srai = 0b010000_111100_00000_101_01000_0010011u32;
    let srai_ = from_opcode_32::<ExecutionEnvironment>(srai);
    type_i(&srai_, Isa::SRAI, 1084, 0, 8);

    let sraiw = 0b0100000_11010_00101_101_00100_0011011u32;
    let sraiw_ = from_opcode_32::<ExecutionEnvironment>(sraiw);
    type_i(&sraiw_, Isa::SRAIW, 1050, 5, 4);

    let sraw = 0b0100000_10110_10001_101_10011_0111011u32;
    let sraw_ = from_opcode_32::<ExecutionEnvironment>(sraw);
    type_r(&sraw_, Isa::SRAW, 22, 17, 19);

    let srli = 0b000000_111110_00000_101_11000_0010011u32;
    let srli_ = from_opcode_32::<ExecutionEnvironment>(srli);
    type_i(&srli_, Isa::SRLI, 62, 0, 24);

    let srliw = 0b0000000_11111_00101_101_00100_0011011u32;
    let srliw_ = from_opcode_32::<ExecutionEnvironment>(srliw);
    type_i(&srliw_, Isa::SRLIW, 31, 5, 4);

    let srlw = 0b0000000_10110_10001_101_10011_0111011u32;
    let srlw_ = from_opcode_32::<ExecutionEnvironment>(srlw);
    type_r(&srlw_, Isa::SRLW, 22, 17, 19);

    let sd = 0b1100001_11100_10111_011_11111_0100011u32;
    let sd_ = from_opcode_32::<ExecutionEnvironment>(sd);
    type_s_b(&sd_, Isa::SD, -961, 28, 23);

    let subw = 0b0100000_10110_10001_000_10011_0111011u32;
    let subw_ = from_opcode_32::<ExecutionEnvironment>(subw);
    type_r(&subw_, Isa::SUBW, 22, 17, 19);*/
}

#[derive(Clone, Copy)]
struct ExecutionEnvironment;

impl MemoryAccess for ExecutionEnvironment {
    fn get_8(&mut self, _: u64) -> u8 {
        0
    }

    fn get_16(&mut self, _: u64) -> u16 {
        0
    }

    fn get_32(&mut self, _: u64) -> u32 {
        0
    }

    fn get_64(&mut self, _: u64) -> u64 {
        0
    }

    fn set_8(&mut self, _: u64, _: u8) {}

    fn set_16(&mut self, _: u64, _: u16) {}

    fn set_32(&mut self, _: u64, _: u32) {}

    fn set_64(&mut self, _: u64, _: u64) {}

    fn get_opcode_32(&mut self, _: u64) -> u32 {
        0
    }
}

impl ExecutionEnvironmentInterface for ExecutionEnvironment {
    fn trap(&mut self, _: Trap) {}
}
