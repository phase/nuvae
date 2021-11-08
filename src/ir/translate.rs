use crate::ast::{AstFunction, Node, Program, ProgramArena, Type, TypedName, TypeIndex};
use crate::ir::{Access, FloatTy, IntTy, IrFunction, IrNode, IrType, IrTypedName, IrTypeIndex, Module, ModuleArena, UIntTy};

pub struct IrBuilderContext<'ctx> {
    program: &'ctx Program,
    module_arena: ModuleArena,
    void_index: IrTypeIndex,
    unknown_index: IrTypeIndex,
}

impl<'ctx> IrBuilderContext<'ctx> {
    pub fn new(program: &'ctx Program) -> IrBuilderContext {
        let mut module_arena = ModuleArena::new();

        let void_index = module_arena.type_arena.insert(IrType::Void);
        let unknown_index = module_arena.type_arena.insert(IrType::Unknown);

        IrBuilderContext {
            program,
            module_arena: ModuleArena::new(),
            void_index,
            unknown_index,
        }
    }
}

pub struct IrBuilder {}

impl IrBuilder {
    pub fn new() -> IrBuilder {
        IrBuilder {}
    }

    pub fn convert(&self, program: Program) -> Module {
        let mut ctx = IrBuilderContext::new(&program);
        for (_index, node) in program.program_arena.node_arena.iter() {
            match node {
                Node::TypeAlias { .. } => {}
                Node::Variable { .. } => {}
                Node::Function(ast_function) => {
                    let node = self.build_function(&mut ctx, ast_function);
                    ctx.module_arena.node_arena.insert(node);
                }
                Node::FunctionPrototype { .. } => {}
                Node::Struct { .. } => {}
                Node::Enum { .. } => {}
                Node::Interface { .. } => {}
                Node::Error => {}
            }
        }
        Module {
            path: program.path.clone(),
            name: program.file_name.clone(),
            imports: program.imports.clone(),
            module_arena: ctx.module_arena,
        }
    }

    fn build_type(&self, ctx: &mut IrBuilderContext, ast_type: TypeIndex) -> IrTypeIndex {
        if let Some(ast_type) = ctx.program.program_arena.type_arena.get(ast_type) {
            match ast_type {
                Type::Base(name) => {
                    if let Some(int_type) = IntTy::from(&name.name) {
                        ctx.module_arena.type_arena.insert(IrType::Int(int_type))
                    } else if let Some(int_type) = UIntTy::from(&name.name) {
                        ctx.module_arena.type_arena.insert(IrType::UInt(int_type))
                    } else if let Some(float_type) = FloatTy::from(&name.name) {
                        ctx.module_arena.type_arena.insert(IrType::Float(float_type))
                    } else if "Void" == name.name {
                        ctx.void_index
                    } else {
                        ctx.unknown_index
                    }
                }
                Type::Refinement(_, _, _) => ctx.unknown_index,
                Type::Row(_) => ctx.unknown_index,
                Type::Reference(_, _) => ctx.unknown_index,
                Type::Optional(_) => ctx.unknown_index,
                Type::Function(_, _) => ctx.unknown_index,
            }
        } else {
            ctx.unknown_index
        }
    }

    fn build_typed_name(&self, ctx: &mut IrBuilderContext, ast_typed_name: TypedName) -> IrTypedName {
        IrTypedName {
            typ: ast_typed_name.typ.map_or(ctx.void_index, |ty| self.build_type(ctx, ty)),
            name: ast_typed_name.name,
        }
    }

    fn build_function(&self, ctx: &mut IrBuilderContext, func: &AstFunction) -> IrNode {
        IrNode::Function(IrFunction {
            access: Access::from(func.access),
            name: func.name.clone(),
            type_params: vec![],
            return_type: self.build_type(ctx, func.return_type),
            blocks: vec![],
        })
    }
}
