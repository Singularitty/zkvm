
#[derive(Debug, Clone, PartialEq, Eq)]
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
    Immediate(i32),
    Label(String)
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    ADD(Operand, Operand, Operand),
    ADDI(Operand, Operand, Operand),
    MOV(Operand, Operand),
    JMP(Operand),
    JZ(Operand, Operand),
    HALT
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Label(String),
    Instr(Instruction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program(pub Vec<Stmt>);