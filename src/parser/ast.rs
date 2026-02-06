pub struct AST {
   pub program: Program,
}

pub enum Program {
   FunctionDefinition(FunctionDefinition),
}

pub enum FunctionDefinition {
   Function(String, Vec<BlockItem>),
}

pub enum BlockItem {
   Stmt(Stmt),
   Decl(Decl)
}

pub enum Decl {
   Decl(String, Option<Expr>, usize),
}

pub enum Stmt {
   Return(Expr),
   Expression(Expr),
   If(Expr, Box<Stmt>, Option<Box<Stmt>>),
   Null,
}

#[derive(Clone)]
pub enum Expr {
   Integer(i64),
   Var(String, usize),
   UnaryOp(UnaryOp, Box<Expr>),
   BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
   Assignment(Box<Expr>, Box<Expr>, usize),
   Conditional(Box<Expr>, Box<Expr>, Box<Expr>)
}

#[derive(Clone)]
pub enum UnaryOp {
   Complement,
   Negate,
   Not,
}

#[derive(Clone)]
pub enum BinaryOp {
   Add,
   Subtract,
   Multiply,
   Divide,
   Modulus,
   BitwiseOr,
   BitwiseAnd,
   BitwiseXor,
   LeftShift,
   RightShift,
   LogicalAnd,
   LogicalOr,
   Equal,
   NotEqual,
   LessThan,
   LessOrEqual,
   GreaterThan,
   GreaterOrEqual,
}