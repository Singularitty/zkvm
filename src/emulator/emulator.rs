use std::ops::Index;

use indexmap::IndexMap;

use crate::asm::asm::{Operand, Register, Stmt};

pub enum MachineState {
    Halted,
    Running
}

fn fresh_register_file() -> IndexMap<Register, Operand> {
    let mut rft = IndexMap::new();
    rft.insert(Register::X0, Operand::Immediate(0));
    rft.insert(Register::X1, Operand::Immediate(0));
    rft.insert(Register::X2, Operand::Immediate(0));
    rft.insert(Register::X3, Operand::Immediate(0));
    rft.insert(Register::X4, Operand::Immediate(0));
    rft.insert(Register::X5, Operand::Immediate(0));
    rft.insert(Register::X6, Operand::Immediate(0));
    rft.insert(Register::X7, Operand::Immediate(0));
    rft
}

pub struct Emulator {
    register_file: IndexMap<Register, Operand>,
    program: Program,
    state: MachineState,
    blocks: IndexMap<Stmt, Vec<Stmt>>
}

impl Emulator {
    pub fn new(program: Program) -> Self {
        Self {
            register_file: fresh_register_file(),
            program: program,
            state: MachineState::Running,
            blocks: gather_blocks(program)
        }
    }

    //TODO: Finish this
    fn map_blocks(program: Program) -> IndexMap<Stmt, Vec<Stmt>> {
        let mut block_map = IndexMap::new();
        let mut in_block = false;
        for stmt in program {
            match stmt {
                Stmt::Label(label) => todo!(),
                Stmt::Instr(instr) => todo!(),
            }
        }
        block_map
    } 
}