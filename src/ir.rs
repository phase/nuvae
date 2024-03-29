use generational_arena::{Arena, Index};
use crate::ast::{BinOpType, ExpressionIndex, Path, TypeName};
use crate::ir::FloatTy::*;
use crate::ir::IntTy::*;
use crate::ir::UIntTy::*;

pub(crate) mod translate;

pub type IrTypeIndex = Index;
pub type IrNodeIndex = Index;
pub type IrBlockIndex = Index;
pub type IrInstructionIndex = Index;

pub struct ModuleArena {
    pub type_arena: Arena<IrType>,
    pub node_arena: Arena<IrNode>,
    pub block_arena: Arena<IrBlock>,
    pub instruction_arena: Arena<IrInstruction>,
}

impl ModuleArena {
    pub fn new() -> ModuleArena {
        ModuleArena {
            type_arena: Arena::new(),
            node_arena: Arena::new(),
            block_arena: Arena::new(),
            instruction_arena: Arena::new(),
        }
    }

    pub fn add_instruction(&mut self, block: &mut IrBlock, ins: IrInstruction) {
        let index = self.instruction_arena.insert(ins);
        block.instructions.push(index);
    }
}

pub struct Module {
    pub path: Path,
    pub name: String,
    pub imports: Vec<Path>,
    pub module_arena: ModuleArena,
}

impl Module {
    pub fn typ(&self, index: IrTypeIndex) -> &IrType {
        self.module_arena.type_arena.get(index).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Access {
    Public,
    Internal,
    Generated,
}

impl Access {
    pub fn from(ast_access: crate::ast::Access) -> Self {
        match ast_access {
            crate::ast::Access::Public => Access::Public,
            crate::ast::Access::Internal => Access::Internal,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum IntTy {
    ISize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

impl IntTy {
    pub fn from<Str: AsRef<str>>(name: Str) -> Option<Self> {
        match name.as_ref() {
            "IntSize" => Some(ISize),
            "Int8" => Some(I8),
            "Int16" => Some(I16),
            "Int32" => Some(I32),
            "Int64" => Some(I64),
            "Int128" => Some(I128),
            &_ => None
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            ISize => 64, // todo
            I8 => 8,
            I16 => 16,
            I32 => 32,
            I64 => 64,
            I128 => 128,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum UIntTy {
    USize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl UIntTy {
    pub fn from<Str: AsRef<str>>(name: Str) -> Option<Self> {
        match name.as_ref() {
            "USize" => Some(USize),
            "UInt8" => Some(U8),
            "UInt16" => Some(U16),
            "UInt32" => Some(U32),
            "UInt64" => Some(U64),
            "UInt128" => Some(U128),
            &_ => None
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            USize => 64, // todo
            U8 => 8,
            U16 => 16,
            U32 => 32,
            U64 => 64,
            U128 => 128,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}

impl FloatTy {
    pub fn from<Str: AsRef<str>>(name: Str) -> Option<Self> {
        match name.as_ref() {
            "Float16" => Some(F16),
            "Float32" => Some(F32),
            "Float64" => Some(F64),
            "Float128" => Some(F128),
            &_ => None
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            F16 => 16,
            F32 => 32,
            F64 => 64,
            F128 => 128,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IrTypedName {
    typ: IrTypeIndex,
    name: String,
}

#[derive(Clone, Debug)]
pub enum IrType {
    Bool,
    Int(IntTy),
    UInt(UIntTy),
    Float(FloatTy),
    Base(String),
    Refinement(String, IrTypeIndex, IrBlockIndex),
    Row(Vec<IrTypedName>),
    Reference(IrTypeIndex, bool),
    Optional(IrTypeIndex),
    Function(Vec<IrTypeIndex>, IrTypeIndex),
    Void,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct IrFunction {
    pub access: Access,
    pub name: String,
    pub type_params: Vec<IrTypedName>,
    pub return_type: IrTypeIndex,
    pub blocks: Vec<IrBlockIndex>,
}

#[derive(Clone, Debug)]
pub enum IrNode {
    Function(IrFunction),
    Struct {
        nodes: Vec<IrNodeIndex>,
    },
    Error,
}

#[derive(Clone, Debug)]
pub struct IrBlock {
    instructions: Vec<IrInstructionIndex>,
}

impl IrBlock {
    fn new() -> Self {
        Self { instructions: vec![] }
    }
}

#[derive(Clone, Debug)]
pub enum IrInstruction {
    Ref(String),
    NatLiteral(i64),
    BoolLiteral(bool),
    BinOp(IrInstructionIndex, BinOpType, IrInstructionIndex),
    FieldAccessor {
        aggregate: IrInstructionIndex,
        value: IrInstructionIndex,
    },
    FunctionCall {
        function: IrInstructionIndex,
        args: Vec<IrInstructionIndex>,
    },
    New {
        typ: IrTypeIndex,
        allocator: IrInstructionIndex,
    },
    Dereference {
        pointer: IrInstructionIndex,
    },
    Denull {
        optional: IrInstructionIndex,
    },
    Borrow {
        value: IrInstructionIndex,
    },
    Branch {
        condition: IrInstructionIndex,
        true_branch: IrBlockIndex,
        false_branch: IrBlockIndex,
    },
    Return {
        value: IrInstructionIndex,
    },
    Error,
}
