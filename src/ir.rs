use generational_arena::{Arena, Index};
use crate::ast::Path;

pub type IrNodeIndex = Index;
pub type IrBlockIndex = Index;
pub type IrInstructionIndex = Index;

pub struct ModuleArena {
    pub node_arena: Arena<IrNode>,
    pub block_arena: Arena<IrBlock>,
    pub instruction_arena: Arena<IrInstruction>,
}

impl ModuleArena {
    pub fn new() -> ModuleArena {
        ModuleArena {
            node_arena: Arena::new(),
            block_arena: Arena::new(),
            instruction_arena: Arena::new(),
        }
    }
}

pub struct Module {
    pub path: Path,
    pub name: String,
    pub imports: Vec<Path>,
    pub module_arena: ModuleArena,
}

#[derive(Clone, Copy, Debug)]
pub enum Access {
    Public,
    Internal,
    Generated,
}

pub enum IrNode {
    Function {
        access: Access,
        name: String,
        blocks: Vec<IrBlockIndex>
    },
    Struct {
        nodes: Vec<IrNodeIndex>,
    },
}

pub struct IrBlock {
    instructions: Vec<IrInstructionIndex>,
}

pub enum IrInstruction {
    IntegerLiteral(u64),
}
