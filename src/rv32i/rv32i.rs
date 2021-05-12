use crate::common::{*, isa::*};
use crate::public::ExecutionEnvironmentInterface;

pub struct RV32I<'a> {
    pub x: [i32; 32],
    pub pc: u32,

    pub inst: Instruction32,
    pub ext: String,
    pub eei: &'a mut dyn ExecutionEnvironmentInterface,
}

impl<'a> RV32I<'a> {
    pub fn single_step(&mut self) {
        let pc = self.pc;
        self.pc += 4;
        let opcode = self.eei.get32le(pc as u64);
        let inst_size = get_instruction_length(opcode as u16);
        match inst_size {
//            2 => if self.ext.contains('C'),
            4 => {
                self.inst = get_instruction32_from_opcode(pc, opcode);

                #[cfg(debug_assertions)]
                RV32I::DISASSEMBLE[self.inst.inst as usize](self.inst);

                RV32I::EXECUTE[self.inst.inst as usize](self);
            },
            _ => println!("Unknown opcode {:#X} at {:#x}", opcode, pc),
        };
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
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.x[self.inst.rs2 as usize];
        }
    }

    pub fn ADDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] + self.inst.imm;
        }
    }

    pub fn AND(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.x[self.inst.rs2 as usize];
        }
    }

    pub fn ANDI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] & self.inst.imm;
        }
    }

    pub fn AUIPC(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc as i32 + self.inst.imm;
        }
    }

    pub fn BEQ(&mut self) {
        if self.x[self.inst.rs1 as usize] == self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    pub fn BGE(&mut self) {
        if self.x[self.inst.rs1 as usize] >= self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    pub fn BGEU(&mut self) {
        if (self.x[self.inst.rs1 as usize] as u32) >= (self.x[self.inst.rs2 as usize] as u32) {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    pub fn BLT(&mut self) {
        if self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    pub fn BLTU(&mut self) {
        if (self.x[self.inst.rs1 as usize] as u32) < (self.x[self.inst.rs2 as usize] as u32) {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    pub fn BNE(&mut self) {
        if self.x[self.inst.rs1 as usize] != self.x[self.inst.rs2 as usize] {
            self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        }
    }

    pub fn EBREAK(&mut self) {
    }

    pub fn ECALL(&mut self) {
    }

    pub fn FENCE(&mut self) {
    }

    pub fn JAL(&mut self) {
        self.pc = (self.inst.pc as i32 + self.inst.imm) as u32;
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc as i32 + 4;
        }
    }

    pub fn JALR(&mut self) {
        self.pc = (self.x[self.inst.rs1 as usize] + self.inst.imm) as u32 & 0xFFFF_FFFE;
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.pc as i32 + 4;
        }
    }

    pub fn LB(&mut self) {
        // if rd == 0, throw exception
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.eei.get8((self.x[self.inst.rs1 as usize] + self.inst.imm) as u64) as i8 as i32;
        }
    }

    pub fn LBU(&mut self) {
    }

    pub fn LH(&mut self) {
    }

    pub fn LHU(&mut self) {
    }

    pub fn LUI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.inst.imm;
        }
    }

    pub fn LW(&mut self) {
    }

    pub fn OR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.x[self.inst.rs2 as usize];
        }
    }

    pub fn ORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] | self.inst.imm;
        }
    }

    pub fn SB(&mut self) {
    }

    pub fn SH(&mut self) {
    }

    pub fn SLL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.x[self.inst.rs2 as usize] & 0x1F);
        }
    }

    pub fn SLLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] << (self.inst.imm & 0x1F);
        }
    }

    pub fn SLT(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.x[self.inst.rs2 as usize]) as i32;
        }
    }

    pub fn SLTI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] < self.inst.imm) as i32;
        }
    }

    pub fn SLTIU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u32) < (self.inst.imm as u32)) as i32;
        }
    }

    pub fn SLTU(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u32) < self.x[self.inst.rs2 as usize] as u32) as i32;
        }
    }

    pub fn SRA(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.x[self.inst.rs2 as usize] & 0x1F);
        }
    }

    pub fn SRAI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] >> (self.inst.imm & 0x1F);
        }
    }

    pub fn SRL(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = ((self.x[self.inst.rs1 as usize] as u32) >> (self.x[self.inst.rs2 as usize] as u32 & 0x1F)) as i32;
        }
    }

    pub fn SRLI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = (self.x[self.inst.rs1 as usize] as u32 >> (self.inst.imm as u32 & 0x1F)) as i32;
        }
    }

    pub fn SUB(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] - self.x[self.inst.rs2 as usize];
        }
    }

    pub fn SW(&mut self) {
    }

    pub fn XOR(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.x[self.inst.rs2 as usize];
        }
    }

    pub fn XORI(&mut self) {
        if self.inst.rd != 0 {
            self.x[self.inst.rd as usize] = self.x[self.inst.rs1 as usize] ^ self.inst.imm;
        }
    }
}

// Disassembler
#[cfg(debug_assertions)]
impl<'a> RV32I<'a> {
    const DISASSEMBLE: [fn(Instruction32); 41] = [
        RV32I::disassemble_UNKNOWN, RV32I::disassemble_ADD, RV32I::disassemble_ADDI, RV32I::disassemble_AND,  RV32I::disassemble_ANDI,   RV32I::disassemble_AUIPC, RV32I::disassemble_BEQ,   RV32I::disassemble_BGE,
        RV32I::disassemble_BGEU,    RV32I::disassemble_BLT, RV32I::disassemble_BLTU, RV32I::disassemble_BNE,  RV32I::disassemble_EBREAK, RV32I::disassemble_ECALL, RV32I::disassemble_FENCE, RV32I::disassemble_JAL,
        RV32I::disassemble_JALR,    RV32I::disassemble_LB,  RV32I::disassemble_LBU,  RV32I::disassemble_LH,   RV32I::disassemble_LHU,    RV32I::disassemble_LUI,   RV32I::disassemble_LW,    RV32I::disassemble_OR,
        RV32I::disassemble_ORI,     RV32I::disassemble_SB,  RV32I::disassemble_SH,   RV32I::disassemble_SLL,  RV32I::disassemble_SLLI,   RV32I::disassemble_SLT,   RV32I::disassemble_SLTI,  RV32I::disassemble_SLTIU,
        RV32I::disassemble_SLTU,    RV32I::disassemble_SRA, RV32I::disassemble_SRAI, RV32I::disassemble_SRL,  RV32I::disassemble_SRLI,   RV32I::disassemble_SUB,   RV32I::disassemble_SW,    RV32I::disassemble_XOR,   RV32I::disassemble_XORI,
    ];

    pub fn disassemble_UNKNOWN(inst: Instruction32) {
        println!("Error: unknown instruction {:?}", inst);
    }

    pub fn disassemble_ADD(inst: Instruction32) {
        println!("Instruction: ADD {:?}", inst);
    }

    pub fn disassemble_ADDI(inst: Instruction32) {
        println!("Instruction: ADDI {:?}", inst);
    }

    pub fn disassemble_AND(inst: Instruction32) {
        println!("Instruction: AND {:?}", inst);
    }

    pub fn disassemble_ANDI(inst: Instruction32) {
        println!("Instruction: ANDI {:?}", inst);
    }

    pub fn disassemble_AUIPC(inst: Instruction32) {
        println!("Instruction: AUIPC {:?}", inst);
    }

    pub fn disassemble_BEQ(inst: Instruction32) {
        println!("Instruction: BEQ {:?}", inst);
    }

    pub fn disassemble_BGE(inst: Instruction32) {
        println!("Instruction: BGE {:?}", inst);
    }

    pub fn disassemble_BGEU(inst: Instruction32) {
        println!("Instruction: BGEU {:?}", inst);
    }

    pub fn disassemble_BLT(inst: Instruction32) {
        println!("Instruction: BLT {:?}", inst);
    }

    pub fn disassemble_BLTU(inst: Instruction32) {
        println!("Instruction: BLTU {:?}", inst);
    }

    pub fn disassemble_BNE(inst: Instruction32) {
        println!("Instruction: BNE {:?}", inst);
    }

    pub fn disassemble_EBREAK(inst: Instruction32) {
        println!("Instruction: EBREAK {:?}", inst);
    }

    pub fn disassemble_ECALL(inst: Instruction32) {
        println!("Instruction: ECALL {:?}", inst);
    }

    pub fn disassemble_FENCE(inst: Instruction32) {
        println!("Instruction: FENCE {:?}", inst);
    }

    pub fn disassemble_JAL(inst: Instruction32) {
        println!("Instruction: JAL {:?}", inst);
    }

    pub fn disassemble_JALR(inst: Instruction32) {
        println!("Instruction: JALR {:?}", inst);
    }

    pub fn disassemble_LB(inst: Instruction32) {
        println!("Instruction: LB {:?}", inst);
    }

    pub fn disassemble_LBU(inst: Instruction32) {
        println!("Instruction: LBU {:?}", inst);
    }

    pub fn disassemble_LH(inst: Instruction32) {
        println!("Instruction: LH {:?}", inst);
    }

    pub fn disassemble_LHU(inst: Instruction32) {
        println!("Instruction: LHU {:?}", inst);
    }

    pub fn disassemble_LUI(inst: Instruction32) {
        println!("Instruction: LUI {:?}", inst);
    }

    pub fn disassemble_LW(inst: Instruction32) {
        println!("Instruction: LW {:?}", inst);
    }

    pub fn disassemble_OR(inst: Instruction32) {
        println!("Instruction: OR {:?}", inst);
    }

    pub fn disassemble_ORI(inst: Instruction32) {
        println!("Instruction: ORI {:?}", inst);
    }

    pub fn disassemble_SB(inst: Instruction32) {
        println!("Instruction: SB {:?}", inst);
    }

    pub fn disassemble_SH(inst: Instruction32) {
        println!("Instruction: SH {:?}", inst);
    }

    pub fn disassemble_SLL(inst: Instruction32) {
        println!("Instruction: SLL {:?}", inst);
    }

    pub fn disassemble_SLLI(inst: Instruction32) {
        println!("Instruction: SLLI {:?}", inst);
    }

    pub fn disassemble_SLT(inst: Instruction32) {
        println!("Instruction: SLT {:?}", inst);
    }

    pub fn disassemble_SLTI(inst: Instruction32) {
        println!("Instruction: SLTI {:?}", inst);
    }

    pub fn disassemble_SLTIU(inst: Instruction32) {
        println!("Instruction: SLTIU {:?}", inst);
    }

    pub fn disassemble_SLTU(inst: Instruction32) {
        println!("Instruction: SLTU {:?}", inst);
    }

    pub fn disassemble_SRA(inst: Instruction32) {
        println!("Instruction: SRA {:?}", inst);
    }

    pub fn disassemble_SRAI(inst: Instruction32) {
        println!("Instruction: SRAI {:?}", inst);
    }

    pub fn disassemble_SRL(inst: Instruction32) {
        println!("Instruction: SRL {:?}", inst);
    }

    pub fn disassemble_SRLI(inst: Instruction32) {
        println!("Instruction: SRLI {:?}", inst);
    }

    pub fn disassemble_SUB(inst: Instruction32) {
        println!("Instruction: SUB {:?}", inst);
    }

    pub fn disassemble_SW(inst: Instruction32) {
        println!("Instruction: SW {:?}", inst);
    }

    pub fn disassemble_XOR(inst: Instruction32) {
        println!("Instruction: XOR {:?}", inst);
    }

    pub fn disassemble_XORI(inst: Instruction32) {
        println!("Instruction: XORI {:?}", inst);
    }
}
