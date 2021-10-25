use std::cell::RefCell;
use std::fmt;
use std::fmt::Formatter;
use generational_arena::{Arena, Index};
use crate::ast::Node::Function;

pub type TypeIndex = Index;
pub type NodeIndex = Index;
pub type StatementIndex = Index;
pub type ExpressionIndex = Index;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Path(pub Vec<String>);

impl Path {
    pub fn new() -> Self {
        Path(vec![])
    }

    pub fn of(s: &str) -> Self {
        Self(vec![s.to_string()])
    }

    pub fn append(&self, s: String) -> Self {
        let mut vec = self.0.clone();
        vec.push(s);
        Self(vec)
    }

    pub fn pop(&mut self) -> String {
        self.0.pop().expect("tried to pop empty path")
    }

    pub fn to_string(&self) -> String {
        self.0.join("::")
    }
}

#[derive(Clone, Debug)]
pub struct ProgramArena {
    pub type_arena: Arena<Type>,
    pub node_arena: Arena<Node>,
    pub statement_arena: Arena<Statement>,
    pub expression_arena: Arena<Expression>,
}

impl ProgramArena {
    pub fn new() -> ProgramArena {
        ProgramArena {
            type_arena: Arena::new(),
            node_arena: Arena::new(),
            statement_arena: Arena::new(),
            expression_arena: Arena::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub path: Path,
    pub file_name: String,
    pub imports: Vec<Path>,
    pub program_arena: ProgramArena,
}

#[derive(Clone, Debug)]
pub struct TypedName {
    pub name: String,
    pub typ: Option<TypeIndex>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Base(TypeName),
    Refinement(String, TypeIndex, ExpressionIndex),
    Row(Vec<TypedName>)
}

#[derive(Clone, Debug)]
pub struct TypeName {
    pub path: Path,
    pub name: String,
    pub arguments: Vec<Box<TypeName>>,
}

impl TypeName {
    pub fn to_string(&self) -> String {
        let mut name = format!("{}::{}", self.path.to_string(), self.name);
        if self.arguments.len() > 0 {
            name.push_str("[");
            for typ in self.arguments.iter() {
                name.push_str(&typ.to_string());
            }
            name.push_str("]");
        }
        name
    }
}

impl From<(Path, String)> for TypeName {
    fn from(pair: (Path, String)) -> Self {
        Self {
            path: pair.0,
            name: pair.1,
            arguments: vec![],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Access {
    Public,
    Internal
}

#[derive(Clone, Debug)]
pub enum Node {
    TypeAlias {
        access: Access,
        name: String,
        value: TypeIndex,
    },
    Variable {
        access: Access,
        name: TypedName,
        value: Option<ExpressionIndex>,
    },
    Function {
        access: Access,
        name: String,
        params: Vec<TypedName>,
        return_type: TypeName,
        statements: Vec<StatementIndex>,
    },
    FunctionPrototype {
        name: String,
        params: Vec<TypedName>,
        return_type: TypeName,
    },
    Struct {
        access: Access,
        name: String,
        params: Vec<TypedName>,
        children: Vec<NodeIndex>,
    },
    Interface {
        name: String,
        params: Vec<TypedName>,
        children: Vec<NodeIndex>,
    },
    Error,
}

#[derive(Clone, Debug)]
pub enum Statement {
    If {
        condition: ExpressionIndex,
        body: Vec<StatementIndex>,
        else_if: Option<StatementIndex>,
    },
    Call {
        function: ExpressionIndex,
        args: Vec<ExpressionIndex>,
    },
    Let {
        name: TypedName,
        value: ExpressionIndex,
    },
    Assign {
        name: String,
        value: ExpressionIndex,
    },
    Return {
        value: ExpressionIndex,
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Ref(String),
    NatLiteral(i64),
    BoolLiteral(bool),
    BinOp(ExpressionIndex, BinOpType, ExpressionIndex),
    FieldAccessor {
        aggregate: ExpressionIndex,
        name: String,
    }
}

impl Expression {
    pub fn to_string(&self, program_arena: &ProgramArena) -> String {
        match self {
            Expression::BinOp(a, o, b) => {
                let a_opt = program_arena.expression_arena.get(*a);
                let b_opt = program_arena.expression_arena.get(*b);
                if let (Some(a_exp), Some(b_exp)) = (a_opt, b_opt) {
                    format!("({} {} {})", a_exp.to_string(program_arena), o, b_exp.to_string(program_arena))
                } else {
                    format!("{}", self)
                }
            }
            Expression::FieldAccessor {aggregate, name } => {
                let agg_opt = program_arena.expression_arena.get(*aggregate);
                if let Some(agg_exp) = agg_opt {
                    format!("{}.{}", agg_exp.to_string(program_arena), name)
                } else {
                    format!("{}", self)
                }
            }
            e => format!("{}", e)
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ref(r) => {
                write!(f, "{}", r)
            }
            Expression::NatLiteral(n) => {
                write!(f, "{}", n)
            }
            Expression::BoolLiteral(b) => {
                write!(f, "{}", b)
            }
            Expression::BinOp(a, o, b) => {
                let (a_index, _) = a.into_raw_parts();
                let (b_index, _) = b.into_raw_parts();
                write!(f, "#{} {} #{}", a_index, o, b_index)
            }
            Expression::FieldAccessor { aggregate, name } => {
                let (agg_parts, _) = aggregate.into_raw_parts();
                write!(f, "#{}.{}", agg_parts, name)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BinOpType {
    Plus,
    Minus,
    Star,
    ForwardSlash,
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo,
    And,
    Or,
}

impl fmt::Display for BinOpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            BinOpType::Plus => "+",
            BinOpType::Minus => "-",
            BinOpType::Star => "*",
            BinOpType::ForwardSlash => "/",
            BinOpType::LessThan => "<",
            BinOpType::GreaterThan => ">",
            BinOpType::LessThanEqualTo => "<=",
            BinOpType::GreaterThanEqualTo => ">=",
            BinOpType::And => "and",
            BinOpType::Or => "or",
        })
    }
}