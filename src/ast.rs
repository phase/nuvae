use generational_arena::Index;
use crate::ast::Node::Function;

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
pub struct Program {
    pub path: Path,
    pub file_name: String,
    pub imports: Vec<Path>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct TypedName {
    pub name: String,
    pub typ: Option<Type>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Base(TypeName),
    Refinement(String, TypeName, ExpressionIndex),
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

#[derive(Clone, Debug)]
pub enum Node {
    Function {
        name: String,
        params: Vec<TypedName>,
        return_type: TypeName,
        statements: Vec<StatementIndex>,
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
