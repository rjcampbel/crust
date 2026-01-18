pub struct AST {
   pub program: Program,
}

pub enum Program {
   Function {
      name: String,
      stmt: Stmt,
   }
}

pub enum Stmt {
   Return(Expr)
}

#[derive(Clone)]
pub enum Expr {
   Integer(i64),
   UnaryOp {
      operator: UnaryOp,
      expr: Box<Expr>,
   },
   BinaryOp {
      operator: BinaryOp,
      left: Box<Expr>,
      right: Box<Expr>,
   }
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