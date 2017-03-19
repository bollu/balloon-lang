use std::fmt;

pub type SpanPos = (usize, usize);

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    StrictEquals,
}

#[derive(Debug, Clone)]
pub enum LogicalBinaryOp {
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
}

#[derive(Debug, Clone)]
pub enum LogicalUnaryOp {
    Not,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::FloorDiv => write!(f, "//"),
            BinaryOp::LessThan => write!(f, "<"),
            BinaryOp::LessThanOrEqual => write!(f, "<="),
            BinaryOp::GreaterThan => write!(f, ">"),
            BinaryOp::GreaterThanOrEqual => write!(f, ">="),
            BinaryOp::StrictEquals => write!(f, "=="),
        }
    }
}

impl fmt::Display for LogicalBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogicalBinaryOp::LogicalAnd => write!(f, "and"),
            LogicalBinaryOp::LogicalOr => write!(f, "or"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnaryOp::Minus => write!(f, "-"),
        }
    }
}

impl fmt::Display for LogicalUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogicalUnaryOp::Not => write!(f, "not"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum LhsExpr {
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Variable {
    Identifier(BindingType, String),
}

#[derive(Debug, Clone)]
pub enum BindingType {
    Mutable,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    BinaryExpression(Box<ExprNode>, BinaryOp, Box<ExprNode>),
    BinaryLogicalExpression(Box<ExprNode>, LogicalBinaryOp, Box<ExprNode>),
    UnaryExpression(UnaryOp, Box<ExprNode>),
    UnaryLogicalExpression(LogicalUnaryOp, Box<ExprNode>),
    FunctionCall(String, Vec<ExprNode>),
}

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub pos: SpanPos,
    pub data: Expr,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(LhsExpr, ExprNode),
    VariableDeclaration(Variable, ExprNode),
    Expression(ExprNode),
    Block(Vec<StatementNode>),
    IfThen(ExprNode, Box<StatementNode>),
    IfThenElse(ExprNode, Box<StatementNode>, Box<StatementNode>),
    Loop(Box<StatementNode>),
    Break,
    Empty,
}

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub pos: SpanPos,
    pub data: Statement,
}
