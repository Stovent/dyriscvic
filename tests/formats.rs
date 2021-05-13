use dyriscvic::common::Instruction32;
use dyriscvic::common::isa::ISA;

#[test]
fn formats_32() {
    let isa = ISA::UNKNOWN;

    let type_r_u_1 = Instruction32::decode_type_r(isa, 0, 0b1000000_00101_00100_000_01000_0000000); // rs2 = 5, rs1 = 4, rd = 8
    let type_r_s_1 = Instruction32::decode_type_r(isa, 0, 0b1000000_10101_11100_000_01010_0000000); // rs2 = 21, rs1 = 28, rd = 10
    assert_eq!(type_r_u_1.rs2, 5);  assert_eq!(type_r_u_1.rs1, 4);  assert_eq!(type_r_u_1.rd, 8);
    assert_eq!(type_r_s_1.rs2, 21); assert_eq!(type_r_s_1.rs1, 28); assert_eq!(type_r_s_1.rd, 10);
}
