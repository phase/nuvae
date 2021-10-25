use generational_arena::Arena;
use crate::ast::Path;

pub struct Module {
    pub path: Path,
    pub name: String,
    pub imports: Vec<Path>,
    pub instruction_arena: Arena<IrInstruction>
}

pub enum IrInstruction {

}
