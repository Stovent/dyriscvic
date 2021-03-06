# dyriscvic

dyriscvic is an experimental RISC-V assembler, disassembler and interpreter written in Rust. It serves as a test project to learn Rust and RISC-V.

Short-term goal is to have an interpreter of as many extensions as possible. Long-term is to make a JIT compiler.

## Compatibility

### RV32

| Extension | Assembler | Disassembler |     Interpreter     |
| :-------: | :-------: | :----------: | :-----------------: |
|     A     |           |              |                     |
|     C     |           |              |                     |
|     D     |           |              |                     |
|     F     |           |              |                     |
|     I     |   v2.1    |     v2.1     | v2.1 (except FENCE) |
|     M     |           |              |                     |
|   Zicsr   |           |              |                     |

### RV64

| Extension | Assembler | Disassembler | Interpreter |
| :-------: | :-------: | :----------: | :---------: |
|     A     |           |              |             |
|     C     |           |              |             |
|     D     |           |              |             |
|     F     |           |              |             |
|     I     |   v2.1    |     v2.1     |    v2.1     |
|     M     |           |              |             |
|   Zicsr   |           |              |             |

