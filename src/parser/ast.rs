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
   Decl(String, Option<Expr>),
}

pub enum Stmt {
   Return(Expr),
   Expression(Expr),
   Null,
}

#[derive(Clone)]
pub enum Expr {
   Integer(i64),
   Var(String),
   UnaryOp {
      operator: UnaryOp,
      expr: Box<Expr>,
   },
   BinaryOp {
      operator: BinaryOp,
      left: Box<Expr>,
      right: Box<Expr>,
   },
   Assignment(Box<Expr>, Box<Expr>),
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