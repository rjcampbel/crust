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
   If(Expr, Box<Stmt>, Box<Option<Stmt>>),
   Null,
}

#[derive(Clone)]
pub enum Expr {
   Integer(i64, usize),
   Var(String, usize),
   UnaryOp {
      operator: UnaryOp,
      expr: Box<Expr>,
      line_number: usize,
   },
   BinaryOp {
      operator: BinaryOp,
      left: Box<Expr>,
      right: Box<Expr>,
      line_number: usize,
   },
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