use crate::common::*;
use crate::public::ExecutionEnvironmentInterface;

pub struct RV32I<'a> {
    pub x: [i32; 32],
    pub pc: u32,

    pub inst: Instruction,
    pub ext: String,
    pub eei: &'a mut dyn ExecutionEnvironmentInterface,
}

#[derive(Clone, Copy)]
enum ISA {
    UNKNOWN,
    ADD,
    ADDI,
    AND,
    ANDI,
    AUIPC,
    BEQ,
    BGE,
    BGEU,
    BLT,
    BLTU,
    BNE,
    EBREAK,
    ECALL,
    FENCE,
    JAL,
    JALR,
    LB,
    LBU,
    LH,
    LHU,
    LUI,
    LW,
    OR,
    ORI,
    SB,
    SH,
    SLL,
    SLLI,
    SLT,
    SLTI,
    SLTIU,
    SLTU,
    SRA,
    SRAI,
    SRL,
    SRLI,
    SUB,
    SW,
    XOR,
    XORI,
}

impl<'a> RV32I<'a> {
    /// Create a new context of RV32I.
    /// {extensions} is a string containing the capital letters of RISC-V standard extensions
    pub fn new(extensions: &str, eei: &'a mut dyn ExecutionEnvironmentInterface) -> Self {
        Self {
            x: [0; 32],
            pc: 0,
            inst: Instruction::new_empty(),
            ext: String::from(extensions),
            eei,
        }
    }

    pub fn single_step(&mut self) {
        let opcode = self.eei.get32le(self.pc as u64);
        let inst_size = get_instruction_length(opcode as u16);
        self.pc += inst_size as u32;
        match inst_size {
//            2 => if self.ext.contains('C'),
            4 => {
                let inst = RV32I::get_instruction_from_opcode(opcode);
                self.inst = RV32I::FORMAT[inst as usize](opcode);

                #[cfg(debug_assertions)]
                RV32I::DISASSEMBLE[inst as usize](self.inst);

                RV32I::EXECUTE[inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#x}", opcode, self.pc),
        };
    }
}

// Decoder
impl<'a> RV32I<'a> {
    const FORMAT: [fn(u32) -> Instruction; 41] = [
        decode_type_fail, decode_type_r, decode_type_i, decode_type_r, decode_type_i,     decode_type_u,     decode_type_b, decode_type_b,
        decode_type_b,    decode_type_b, decode_type_b, decode_type_b, decode_type_empty, decode_type_empty, decode_type_i, decode_type_j,
        decode_type_i,    decode_type_i, decode_type_i, decode_type_i, decode_type_i,     decode_type_u,     decode_type_i, decode_type_r,
        decode_type_i,    decode_type_s, decode_type_s, decode_type_r, decode_type_r,     decode_type_r,     decode_type_i, decode_type_i,
        decode_type_r,    decode_type_r, decode_type_r, decode_type_r, decode_type_r,     decode_type_r,     decode_type_s, decode_type_r, decode_type_i,
    ];

    const ISA_ARITHMETIC: [ISA; 8] = [ISA::ADD, ISA::SLL, ISA::SLT, ISA::SLTU, ISA::XOR, ISA::SRL, ISA::OR, ISA::AND];
    const ISA_BRANCH: [ISA; 8] = [ISA::BEQ, ISA::BNE, ISA::UNKNOWN, ISA::UNKNOWN, ISA::BLT, ISA::BGE, ISA::BLTU, ISA::BGEU];
    const ISA_IMMEDIATE: [ISA; 8] = [ISA::ADDI, ISA::SLLI, ISA::SLTI, ISA::SLTIU, ISA::XORI, ISA::SRLI, ISA::ORI, ISA::ANDI];
    const ISA_LOAD: [ISA; 8] = [ISA::LB, ISA::LH, ISA::LW, ISA::UNKNOWN, ISA::LBU, ISA::LHU, ISA::UNKNOWN, ISA::UNKNOWN];
    const ISA_STORE: [ISA; 4] = [ISA::SB, ISA::SH, ISA::SW, ISA::UNKNOWN];

    fn get_instruction_from_opcode(opcode: u32) -> ISA {
        match opcode & 0b111_1111 {
            0b000_0011 => RV32I::ISA_LOAD[opcode as usize >> 12 & 0b111],
            0b000_1111 => ISA::FENCE,
            0b001_0011 => RV32I::get_instruction_from_opcode_immediate(opcode),
            0b001_0111 => ISA::AUIPC,
            0b010_0011 => RV32I::ISA_STORE[opcode as usize >> 12 & 0b11],
            0b011_0011 => RV32I::get_instruction_from_opcode_arithmetic(opcode),
            0b011_0111 => ISA::LUI,
            0b110_0011 => RV32I::ISA_BRANCH[opcode as usize >> 12 & 0b111],
            0b110_0111 => ISA::JALR,
            0b110_1111 => ISA::JAL,
            0b111_0011 => if opcode == 0x0000_0073 { ISA::ECALL } else if opcode == 0x0010_0073 { ISA::EBREAK } else { ISA::UNKNOWN },
            _ => ISA::UNKNOWN,
        }
    }

    fn get_instruction_from_opcode_immediate(opcode: u32) -> ISA {
        if opcode >> 12 & 0b111 == 0b101 && opcode >> 25 & 0b111_1111 == 0b010_0000 {
            ISA::SRAI
        } else {
            RV32I::ISA_IMMEDIATE[opcode as usize >> 12 & 0b111]
        }
    }

    fn get_instruction_from_opcode_arithmetic(opcode: u32) -> ISA {
        if opcode >> 12 & 0b111 == 0 && opcode >> 25 & 0b111_1111 == 0b010_0000 {
            ISA::SUB
        } else if opcode >> 12 & 0b111 == 0b101 && opcode >> 25 & 0b111_1111 == 0b010_0000 {
            ISA::SRA
        } else if opcode >> 25 & 0b111_1111 != 0 {
            ISA::UNKNOWN
        } else {
            RV32I::ISA_ARITHMETIC[opcode as usize >> 12 & 0b111]
        }
    }
}

// Interpreter
impl<'a> RV32I<'a> {
    const EXECUTE: [fn(&mut RV32I<'a>); 41] = [
        RV32I::UNKNOWN, RV32I::ADD, RV32I::ADDI, RV32I::AND,  RV32I::ANDI,   RV32I::AUIPC, RV32I::BEQ,   RV32I::BGE,
        RV32I::BGEU,    RV32I::BLT, RV32I::BLTU, RV32I::BNE,  RV32I::EBREAK, RV32I::ECALL, RV32I::FENCE, RV32I::JAL,
        RV32I::JALR,    RV32I::LB,  RV32I::LBU,  RV32I::LH,   RV32I::LHU,    RV32I::LUI,   RV32I::LW,    RV32I::OR,
        RV32I::ORI,     RV32I::SB,  RV32I::SH,   RV32I::SLL,  RV32I::SLLI,   RV32I::SLT,   RV32I::SLTI,  RV32I::SLTIU,
        RV32I::SLTU,    RV32I::SRA, RV32I::SRAI, RV32I::SRL,  RV32I::SRLI,   RV32I::SUB,   RV32I::SW,    RV32I::XOR,   RV32I::XORI,
    ];

    pub fn UNKNOWN(&mut self) {}

    pub fn ADD(&mut self) {
    }

    pub fn ADDI(&mut self) {
    }

    pub fn AND(&mut self) {
    }

    pub fn ANDI(&mut self) {
    }

    pub fn AUIPC(&mut self) {
    }

    pub fn BEQ(&mut self) {
    }

    pub fn BGE(&mut self) {
    }

    pub fn BGEU(&mut self) {
    }

    pub fn BLT(&mut self) {
    }

    pub fn BLTU(&mut self) {
    }

    pub fn BNE(&mut self) {
    }

    pub fn EBREAK(&mut self) {
    }

    pub fn ECALL(&mut self) {
    }

    pub fn FENCE(&mut self) {
    }

    pub fn JAL(&mut self) {
        self.x[self.inst.rd as usize] = self.pc as i32;
        self.pc = (self.pc as i32 + self.inst.imm - 4) as u32;
    }

    pub fn JALR(&mut self) {
    }

    pub fn LB(&mut self) {
        // if rd == 0, throw exception
        self.x[self.inst.rd as usize] = self.eei.get8((self.x[self.inst.rs1 as usize] + self.inst.imm) as u64) as i8 as i32;
    }

    pub fn LBU(&mut self) {
    }

    pub fn LH(&mut self) {
    }

    pub fn LHU(&mut self) {
    }

    pub fn LUI(&mut self) {
    }

    pub fn LW(&mut self) {
    }

    pub fn OR(&mut self) {
    }

    pub fn ORI(&mut self) {
    }

    pub fn SB(&mut self) {
    }

    pub fn SH(&mut self) {
    }

    pub fn SLL(&mut self) {
    }

    pub fn SLLI(&mut self) {
    }

    pub fn SLT(&mut self) {
    }

    pub fn SLTI(&mut self) {
    }

    pub fn SLTIU(&mut self) {
    }

    pub fn SLTU(&mut self) {
    }

    pub fn SRA(&mut self) {
    }

    pub fn SRAI(&mut self) {
    }

    pub fn SRL(&mut self) {
    }

    pub fn SRLI(&mut self) {
    }

    pub fn SUB(&mut self) {
    }

    pub fn SW(&mut self) {
    }

    pub fn XOR(&mut self) {
    }

    pub fn XORI(&mut self) {
    }
}

// Disassembler
impl<'a> RV32I<'a> {
    const DISASSEMBLE: [fn(Instruction); 41] = [
        RV32I::disassemble_UNKNOWN, RV32I::disassemble_ADD, RV32I::disassemble_ADDI, RV32I::disassemble_AND,  RV32I::disassemble_ANDI,   RV32I::disassemble_AUIPC, RV32I::disassemble_BEQ,   RV32I::disassemble_BGE,
        RV32I::disassemble_BGEU,    RV32I::disassemble_BLT, RV32I::disassemble_BLTU, RV32I::disassemble_BNE,  RV32I::disassemble_EBREAK, RV32I::disassemble_ECALL, RV32I::disassemble_FENCE, RV32I::disassemble_JAL,
        RV32I::disassemble_JALR,    RV32I::disassemble_LB,  RV32I::disassemble_LBU,  RV32I::disassemble_LH,   RV32I::disassemble_LHU,    RV32I::disassemble_LUI,   RV32I::disassemble_LW,    RV32I::disassemble_OR,
        RV32I::disassemble_ORI,     RV32I::disassemble_SB,  RV32I::disassemble_SH,   RV32I::disassemble_SLL,  RV32I::disassemble_SLLI,   RV32I::disassemble_SLT,   RV32I::disassemble_SLTI,  RV32I::disassemble_SLTIU,
        RV32I::disassemble_SLTU,    RV32I::disassemble_SRA, RV32I::disassemble_SRAI, RV32I::disassemble_SRL,  RV32I::disassemble_SRLI,   RV32I::disassemble_SUB,   RV32I::disassemble_SW,    RV32I::disassemble_XOR,   RV32I::disassemble_XORI,
    ];

    pub fn disassemble_UNKNOWN(inst: Instruction) {
        println!("Error: unknown instruction {:?}", inst);
    }

    pub fn disassemble_ADD(inst: Instruction) {
        println!("Instruction: ADD {:?}", inst);
    }

    pub fn disassemble_ADDI(inst: Instruction) {
        println!("Instruction: ADDI {:?}", inst);
    }

    pub fn disassemble_AND(inst: Instruction) {
        println!("Instruction: AND {:?}", inst);
    }

    pub fn disassemble_ANDI(inst: Instruction) {
        println!("Instruction: ANDI {:?}", inst);
    }

    pub fn disassemble_AUIPC(inst: Instruction) {
        println!("Instruction: AUIPC {:?}", inst);
    }

    pub fn disassemble_BEQ(inst: Instruction) {
        println!("Instruction: BEQ {:?}", inst);
    }

    pub fn disassemble_BGE(inst: Instruction) {
        println!("Instruction: BGE {:?}", inst);
    }

    pub fn disassemble_BGEU(inst: Instruction) {
        println!("Instruction: BGEU {:?}", inst);
    }

    pub fn disassemble_BLT(inst: Instruction) {
        println!("Instruction: BLT {:?}", inst);
    }

    pub fn disassemble_BLTU(inst: Instruction) {
        println!("Instruction: BLTU {:?}", inst);
    }

    pub fn disassemble_BNE(inst: Instruction) {
        println!("Instruction: BNE {:?}", inst);
    }

    pub fn disassemble_EBREAK(inst: Instruction) {
        println!("Instruction: EBREAK {:?}", inst);
    }

    pub fn disassemble_ECALL(inst: Instruction) {
        println!("Instruction: ECALL {:?}", inst);
    }

    pub fn disassemble_FENCE(inst: Instruction) {
        println!("Instruction: FENCE {:?}", inst);
    }

    pub fn disassemble_JAL(inst: Instruction) {
        println!("Instruction: JAL {:?}", inst);
    }

    pub fn disassemble_JALR(inst: Instruction) {
        println!("Instruction: JALR {:?}", inst);
    }

    pub fn disassemble_LB(inst: Instruction) {
        println!("Instruction: LB {:?}", inst);
    }

    pub fn disassemble_LBU(inst: Instruction) {
        println!("Instruction: LBU {:?}", inst);
    }

    pub fn disassemble_LH(inst: Instruction) {
        println!("Instruction: LH {:?}", inst);
    }

    pub fn disassemble_LHU(inst: Instruction) {
        println!("Instruction: LHU {:?}", inst);
    }

    pub fn disassemble_LUI(inst: Instruction) {
        println!("Instruction: LUI {:?}", inst);
    }

    pub fn disassemble_LW(inst: Instruction) {
        println!("Instruction: LW {:?}", inst);
    }

    pub fn disassemble_OR(inst: Instruction) {
        println!("Instruction: OR {:?}", inst);
    }

    pub fn disassemble_ORI(inst: Instruction) {
        println!("Instruction: ORI {:?}", inst);
    }

    pub fn disassemble_SB(inst: Instruction) {
        println!("Instruction: SB {:?}", inst);
    }

    pub fn disassemble_SH(inst: Instruction) {
        println!("Instruction: SH {:?}", inst);
    }

    pub fn disassemble_SLL(inst: Instruction) {
        println!("Instruction: SLL {:?}", inst);
    }

    pub fn disassemble_SLLI(inst: Instruction) {
        println!("Instruction: SLLI {:?}", inst);
    }

    pub fn disassemble_SLT(inst: Instruction) {
        println!("Instruction: SLT {:?}", inst);
    }

    pub fn disassemble_SLTI(inst: Instruction) {
        println!("Instruction: SLTI {:?}", inst);
    }

    pub fn disassemble_SLTIU(inst: Instruction) {
        println!("Instruction: SLTIU {:?}", inst);
    }

    pub fn disassemble_SLTU(inst: Instruction) {
        println!("Instruction: SLTU {:?}", inst);
    }

    pub fn disassemble_SRA(inst: Instruction) {
        println!("Instruction: SRA {:?}", inst);
    }

    pub fn disassemble_SRAI(inst: Instruction) {
        println!("Instruction: SRAI {:?}", inst);
    }

    pub fn disassemble_SRL(inst: Instruction) {
        println!("Instruction: SRL {:?}", inst);
    }

    pub fn disassemble_SRLI(inst: Instruction) {
        println!("Instruction: SRLI {:?}", inst);
    }

    pub fn disassemble_SUB(inst: Instruction) {
        println!("Instruction: SUB {:?}", inst);
    }

    pub fn disassemble_SW(inst: Instruction) {
        println!("Instruction: SW {:?}", inst);
    }

    pub fn disassemble_XOR(inst: Instruction) {
        println!("Instruction: XOR {:?}", inst);
    }

    pub fn disassemble_XORI(inst: Instruction) {
        println!("Instruction: XORI {:?}", inst);
    }
}
