use dyriscvic::common::instruction::Instruction32;
use dyriscvic::common::isa::Isa;

#[test]
fn formats_32() {
    let isa = Isa::UNKNOWN;

    let type_r_1 = Instruction32::decode_type_r(isa, 0, 0b1000000_00101_00100_000_01000_0000000); // rs2 = 5, rs1 = 4, rd = 8
    assert_eq!(type_r_1.rs2, 5); assert_eq!(type_r_1.rs1, 4); assert_eq!(type_r_1.rd, 8);
    let type_r_2 = Instruction32::decode_type_r(isa, 0, 0b1000000_10101_11100_000_01010_0000000); // rs2 = 21, rs1 = 28, rd = 10
    assert_eq!(type_r_2.rs2, 21); assert_eq!(type_r_2.rs1, 28); assert_eq!(type_r_2.rd, 10);

    let type_i_1 = Instruction32::decode_type_i(isa, 0, 0b011011100000_00101_000_10100_0000000); // imm = 1760, rs1 = 5, rd = 20
    assert_eq!(type_i_1.imm, 1760); assert_eq!(type_i_1.rs1, 5); assert_eq!(type_i_1.rd, 20);
    let type_i_2 = Instruction32::decode_type_i(isa, 0, 0b100111010010_11000_010_11111_0000000); // imm = -1582, rs1 = 24, rd = 31
    assert_eq!(type_i_2.imm, -1582); assert_eq!(type_i_2.rs1, 24); assert_eq!(type_i_2.rd, 31);

    let type_s_1 = Instruction32::decode_type_s(isa, 0, 0b0110011_01111_00111_000_10011_0000000); // imm = 1651, rs2 = 15, rs1 = 7
    assert_eq!(type_s_1.imm, 1651); assert_eq!(type_s_1.rs2, 15); assert_eq!(type_s_1.rs1, 7);
    let type_s_2 = Instruction32::decode_type_s(isa, 0, 0b1111010_11101_01011_000_10010_0000000); // imm = -174, rs2 = 29, rs1 = 11
    assert_eq!(type_s_2.imm, -174); assert_eq!(type_s_2.rs2, 29); assert_eq!(type_s_2.rs1, 11);

    let type_b_1 = Instruction32::decode_type_b(isa, 0, 0b0_100110_00110_01001_000_1111_0_0000000); // imm = 1246, rs2 = 6, rs1 = 9
    assert_eq!(type_b_1.imm, 1246); assert_eq!(type_b_1.rs2, 6); assert_eq!(type_b_1.rs1, 9);
    let type_b_2 = Instruction32::decode_type_b(isa, 0, 0b1_001101_11110_00000_000_1001_1_0000000); // imm = -1 614, rs2 = 30, rs1 = 0
    assert_eq!(type_b_2.imm, -1_614); assert_eq!(type_b_2.rs2, 30); assert_eq!(type_b_2.rs1, 0);

    let type_u_1 = Instruction32::decode_type_u(isa, 0, 0b01101110000000101001_11101_0000000); // imm = 1 845 661 696, rd = 29
    assert_eq!(type_u_1.imm, 1_845_661_696); assert_eq!(type_u_1.rd, 29);
    let type_u_2 = Instruction32::decode_type_u(isa, 0, 0b10011101001011000011_11011_0000000); // imm = -1 658 048 512, rd = 27
    assert_eq!(type_u_2.imm, -1_658_048_512); assert_eq!(type_u_2.rd, 27);

    let type_j_1 = Instruction32::decode_type_j(isa, 0, 0b0_1101110000_1_00101001_10001_0000000); // imm = 171 744, rd = 17
    assert_eq!(type_j_1.imm, 171_744); assert_eq!(type_j_1.rd, 17);
    let type_j_2 = Instruction32::decode_type_j(isa, 0, 0b1_0011101001_0_11000011_10011_0000000); // imm = -249 390, rd = 19
    assert_eq!(type_j_2.imm, -249_390); assert_eq!(type_j_2.rd, 19);
}
