https://content.riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf

32-bit instructions: lowest two bits set to 11
others: 00, 01 or 10

## RV32I Base Integer Instruction Set

31 general purpose registers: x1 to x31
x0 = 0
standard convention: x1 to return address on a call

RV32: 32-bit wide registers
RV64: 64-bit wide registers
Called XLEN (current width of an x register in bits)

PC: address of the current instruction

rs1 & rs2: source registers
rd: destination register

Integer computational instructions: I-type (register-immediate)
Register-register: R-type format
