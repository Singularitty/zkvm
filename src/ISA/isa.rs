pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7
}

pub enum Operand {
    Register(Register),
    Immediate(i32),
    Label(String)
}

pub enum Instruction {
    ADD(Register, Register, Register),
    ADDI(Register, Register, Immediate),
    MOVI(Register, Operand),
    JMP(Operand),
    JZ(Operand, Operand),
    HALT
}