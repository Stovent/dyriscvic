use dyriscvic::rvi::assembler::*;

#[test]
fn assembler_i32() {
    let add = ADD(1, 2, 3);
    let add_ = 0b0000000_00011_00010_000_00001_0110011u32;
    assert_eq!(add, add_, "ADD {:X} {:X}", add, add_);

    let addi = ADDI(29, 30, 0);
    let addi_ = 0b000000000000_11110_000_11101_0010011u32;
    assert_eq!(addi, addi_, "ADDI {:X} {:X}", addi, addi_);

    let and = AND(2, 3, 4);
    let and_ = 0b0000000_00100_00011_111_00010_0110011u32;
    assert_eq!(and, and_, "AND {:X} {:X}", and, and_);

    let andi = ANDI(30, 31, 0b100000000001);
    let andi_ = 0b100000000001_11111_111_11110_0010011u32;
    assert_eq!(andi, andi_, "ANDI {:X} {:X}", andi, andi_);

    let beq = BEQ(20, 21, 0b101010010101 << 1);
    let beq_ = 0b1101001_10101_10100_000_01010_1100011u32;
    assert_eq!(beq, beq_, "BEQ {:X} {:X}", beq, beq_);

    let bge = BGE(21, 22, 0b101001100101 << 1);
    let bge_ = 0b1100110_10110_10101_101_01010_1100011u32;
    assert_eq!(bge, bge_, "BGE {:X} {:X}", bge, bge_);

    let bgeu = BGEU(22, 23, 0b101011110101 << 1);
    let bgeu_ = 0b1101111_10111_10110_111_01010_1100011u32;
    assert_eq!(bgeu, bgeu_, "BGEU {:X} {:X}", bgeu, bgeu_);

    let blt = BLT(23, 24, 0b100111111001 << 1);
    let blt_ = 0b1011111_11000_10111_100_10010_1100011u32;
    assert_eq!(blt, blt_, "BLT {:X} {:X}", blt, blt_);

    let bltu = BLTU(24, 25, 0b101111111101 << 1);
    let bltu_ = 0b1111111_11001_11000_110_11010_1100011u32;
    assert_eq!(bltu, bltu_, "BLTU {:X} {:X}", bltu, bltu_);

    let bne = BNE(25, 26, 0b011111111110 << 1);
    let bne_ = 0b0111111_11010_11001_001_11101_1100011u32;
    assert_eq!(bne, bne_, "BNE {:X} {:X}", bne, bne_);

    let auipc = LUI(31, 0xFEDC_BA98);
    let auipc_ = 0xFEDC_BFB7u32;
    assert_eq!(auipc, auipc_, "AUIPC {:X} {:X}", auipc, auipc_);

    assert_eq!(EBREAK(), 0x0010_0073, "EBREAK");
    assert_eq!(ECALL(), 0x0000_0073, "ECALL");

    let fence = FENCE(11, 12, 0b0110, 0b1001, 0b1011);
    let fence_ = 0b1011_1001_0110_01100_000_01011_0001111u32;
    assert_eq!(fence, fence_, "FENCE {:X} {:X}", fence, fence_);

    let jal = JAL(0b10101, 0b1_11001100_1_1001100110 << 1);
    let jal_ = 0xCCDCCAEFu32;
    assert_eq!(jal, jal_, "JAL {:X} {:X}", jal, jal_);

    let jalr = JALR(0b11100, 0b11011, 0x800);
    let jalr_ = 0x800D_8E67u32;
    assert_eq!(jalr, jalr_, "JALR {:X} {:X}", jalr, jalr_);

    let lb = LB(15, 16, 0b111001100111);
    let lb_ = 0b111001100111_10000_000_01111_0000011u32;
    assert_eq!(lb, lb_, "LB {:X} {:X}", lb, lb_);

    let lbu = LBU(16, 17, 0b110011110011);
    let lbu_ = 0b110011110011_10001_100_10000_0000011u32;
    assert_eq!(lbu, lbu_, "LBU {:X} {:X}", lbu, lbu_);

    let lh = LH(17, 18, 0b100111111001);
    let lh_ = 0b100111111001_10010_001_10001_0000011u32;
    assert_eq!(lh, lh_, "LH {:X} {:X}", lh, lh_);

    let lhu = LHU(18, 19, 0b000011110000);
    let lhu_ = 0b000011110000_10011_101_10010_0000011u32;
    assert_eq!(lhu, lhu_, "LHU {:X} {:X}", lhu, lhu_);

    let lui = LUI(5, 0xFEDC_BA98);
    let lui_ = 0xFEDC_B2B7u32;
    assert_eq!(lui, lui_, "LUI {:X} {:X}", lui, lui_);

    let lw = LW(19, 20, 0b111100001111);
    let lw_ = 0b111100001111_10100_010_10011_0000011u32;
    assert_eq!(lw, lw_, "LW {:X} {:X}", lw, lw_);

    let or = OR(3, 4, 5);
    let or_ = 0b0000000_00101_00100_110_00011_0110011u32;
    assert_eq!(or, or_, "OR {:X} {:X}", or, or_);

    let ori = ORI(31, 1, 0b110000000011);
    let ori_ = 0b110000000011_00001_110_11111_0010011u32;
    assert_eq!(ori, ori_, "ORI {:X} {:X}", ori, ori_);

    let sb = SB(12, 13, 0b011010010011);
    let sb_ = 0b0110100_01100_01101_000_10011_0100011_u32;
    assert_eq!(sb, sb_, "SB {:X} {:X}", sb, sb_);

    let sh = SH(13, 14, 0b011010011100);
    let sh_ = 0b0110100_01101_01110_001_11100_0100011_u32;
    assert_eq!(sh, sh_, "SH {:X} {:X}", sh, sh_);

    let sll = SLL(4, 5, 6);
    let sll_ = 0b0000000_00110_00101_001_00100_0110011u32;
    assert_eq!(sll, sll_, "SLL {:X} {:X}", sll, sll_);

    let slli = SLLI(26, 27, 1);
    let slli_ = 0b0000000_00001_11011_001_11010_0010011u32;
    assert_eq!(slli, slli_, "SLLI {:X} {:X}", slli, slli_);

    let slt = SLT(5, 6, 7);
    let slt_ = 0b0000000_00111_00110_010_00101_0110011u32;
    assert_eq!(slt, slt_, "SLT {:X} {:X}", slt, slt_);

    let slti = SLTI(1, 2, 0b111000000111);
    let slti_ = 0b111000000111_00010_010_00001_0010011u32;
    assert_eq!(slti, slti_, "SLTI {:X} {:X}", slti, slti_);

    let sltiu = SLTIU(2, 3, 0b111100001111);
    let sltiu_ = 0b111100001111_00011_011_00010_0010011u32;
    assert_eq!(sltiu, sltiu_, "SLTIU {:X} {:X}", sltiu, sltiu_);

    let sltu = SLTU(6, 7, 8);
    let sltu_ = 0b0000000_01000_00111_011_00110_0110011u32;
    assert_eq!(sltu, sltu_, "SLTU {:X} {:X}", sltu, sltu_);

    let sra = SRA(7, 8, 9);
    let sra_ = 0b0100000_01001_01000_101_00111_0110011u32;
    assert_eq!(sra, sra_, "SRA {:X} {:X}", sra, sra_);

    let srai = SRAI(27, 28, 3);
    let srai_ = 0b0100000_00011_11100_101_11011_0010011u32;
    assert_eq!(srai, srai_, "SRAI {:X} {:X}", srai, srai_);

    let srl = SRL(8, 9, 10);
    let srl_ = 0b0000000_01010_01001_101_01000_0110011u32;
    assert_eq!(srl, srl_, "SRL {:X} {:X}", srl, srl_);

    let srli = SRLI(28, 29, 5);
    let srli_ = 0b0000000_00101_11101_101_11100_0010011u32;
    assert_eq!(srli, srli_, "SRLI {:X} {:X}", srli, srli_);

    let sub = SUB(9, 10, 11);
    let sub_ = 0b0100000_01011_01010_000_01001_0110011u32;
    assert_eq!(sub, sub_, "SUB {:X} {:X}", sub, sub_);

    let sw = SW(14, 15, 0b101010010110);
    let sw_ = 0b1010100_01110_01111_010_10110_0100011_u32;
    assert_eq!(sw, sw_, "SW {:X} {:X}", sw, sw_);

    let xor = XOR(10, 11, 12);
    let xor_ = 0b0000000_01100_01011_100_01010_0110011u32;
    assert_eq!(xor, xor_, "XOR {:X} {:X}", xor, xor_);

    let xori = XORI(3, 4, 0b111110011111);
    let xori_ = 0b111110011111_00100_100_00011_0010011u32;
    assert_eq!(xori, xori_, "XORI {:X} {:X}", xori, xori_);
}

#[test]
fn assembler_i64() {
    let addiw = ADDIW(1, 2, 1);
    let addiw_ = 0b000000000001_00010_000_00001_0011011u32;
    assert_eq!(addiw, addiw_, "ADDIW {:X} {:X}", addiw, addiw_);

    let addw = ADDW(3, 4, 5);
    let addw_ = 0b0000000_00101_00100_000_00011_0111011u32;
    assert_eq!(addw, addw_, "ADDW {:X} {:X}", addw, addw_);

    let ld = LD(6, 7, -2i32 as u32);
    let ld_ = 0b111111111110_00111_011_00110_0000011u32;
    assert_eq!(ld, ld_, "LD {:X} {:X}", ld, ld_);

    let lwu = LWU(8, 9, -3i32 as u32);
    let lwu_ = 0b111111111101_01001_110_01000_0000011u32;
    assert_eq!(lwu, lwu_, "LWU {:X} {:X}", lwu, lwu_);

    let sd = SD(10, 11, -4i32 as u32);
    let sd_ = 0b1111111_01011_01010_011_11100_0100011u32;
    assert_eq!(sd, sd_, "SD {:X} {:X}", sd, sd_);

    let slliw = SLLIW(12, 13, 1);
    let slliw_ = 0b0000000_00001_01101_001_01100_0011011u32;
    assert_eq!(slliw, slliw_, "SLLIW {:X} {:X}", slliw, slliw_);

    let sllw = SLLW(14, 15, 16);
    let sllw_ = 0b0000000_10000_01111_001_01110_0111011u32;
    assert_eq!(sllw, sllw_, "SLLW {:X} {:X}", sllw, sllw_);

    let sraiw = SRAIW(17, 18, 2);
    let sraiw_ = 0b0100000_00010_10010_101_10001_0011011u32;
    assert_eq!(sraiw, sraiw_, "SRAIW {:X} {:X}", sraiw, sraiw_);

    let sraw = SRAW(19, 20, 21);
    let sraw_ = 0b0100000_10101_10100_101_10011_0111011u32;
    assert_eq!(sraw, sraw_, "SRAW {:X} {:X}", sraw, sraw_);

    let srliw = SRLIW(22, 23, 3);
    let srliw_ = 0b0000000_00011_10111_101_10110_0011011u32;
    assert_eq!(srliw, srliw_, "SRLIW {:X} {:X}", srliw, srliw_);

    let srlw = SRLW(24, 25, 26);
    let srlw_ = 0b0000000_11010_11001_101_11000_0111011u32;
    assert_eq!(srlw, srlw_, "SRLW {:X} {:X}", srlw, srlw_);

    let subw = SUBW(27, 28, 29);
    let subw_ = 0b0100000_11101_11100_000_11011_0111011u32;
    assert_eq!(subw, subw_, "SUBW {:X} {:X}", subw, subw_);
}

#[test]
fn assembler_m32() {
    let div = DIV(1, 2, 1);
    let div_ = 0b0000001_00001_00010_100_00001_0110011u32;
    assert_eq!(div, div_, "DIV {:X} {:X}", div, div_);

    let divu = DIVU(3, 4, 5);
    let divu_ = 0b0000001_00101_00100_101_00011_0110011u32;
    assert_eq!(divu, divu_, "DIVU {:X} {:X}", divu, divu_);

    let mul = MUL(6, 7, 8);
    let mul_ = 0b0000001_01000_00111_000_00110_0110011u32;
    assert_eq!(mul, mul_, "MUL {:X} {:X}", mul, mul_);

    let mulh = MULH(8, 9, 10);
    let mulh_ = 0b0000001_01010_01001_001_01000_0110011u32;
    assert_eq!(mulh, mulh_, "MULH {:X} {:X}", mulh, mulh_);

    let mulhsu = MULHSU(10, 11, 12);
    let mulhsu_ = 0b0000001_01100_01011_010_01010_0110011u32;
    assert_eq!(mulhsu, mulhsu_, "MULHSU {:X} {:X}", mulhsu, mulhsu_);

    let mulhu = MULHU(12, 13, 1);
    let mulhu_ = 0b0000001_00001_01101_011_01100_0110011u32;
    assert_eq!(mulhu, mulhu_, "MULHU {:X} {:X}", mulhu, mulhu_);

    let rem = REM(14, 15, 16);
    let rem_ = 0b0000001_10000_01111_110_01110_0110011u32;
    assert_eq!(rem, rem_, "REM {:X} {:X}", rem, rem_);

    let remu = REMU(17, 18, 2);
    let remu_ = 0b0000001_00010_10010_111_10001_0110011u32;
    assert_eq!(remu, remu_, "REMU {:X} {:X}", remu, remu_);
}

#[test]
fn assembler_m64() {
    let divuw = DIVUW(22, 23, 3);
    let divuw_ = 0b0000001_00011_10111_101_10110_0111011u32;
    assert_eq!(divuw, divuw_, "DIVUW {:X} {:X}", divuw, divuw_);

    let divw = DIVW(19, 20, 21);
    let divw_ = 0b0000001_10101_10100_100_10011_0111011u32;
    assert_eq!(divw, divw_, "DIVW {:X} {:X}", divw, divw_);

    let mulw = MULW(24, 25, 26);
    let mulw_ = 0b0000001_11010_11001_000_11000_0111011u32;
    assert_eq!(mulw, mulw_, "MULW {:X} {:X}", mulw, mulw_);

    let remuw = REMUW(27, 28, 29);
    let remuw_ = 0b0000001_11101_11100_111_11011_0111011u32;
    assert_eq!(remuw, remuw_, "REMUW {:X} {:X}", remuw, remuw_);

    let remw = REMW(27, 28, 29);
    let remw_ = 0b0000001_11101_11100_110_11011_0111011u32;
    assert_eq!(remw, remw_, "REMUW {:X} {:X}", remw, remw_);
}
