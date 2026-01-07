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

pub enum Expr {
   Integer(u64),
   UnaryOp {
      operator: UnaryOp,
      expr: Box<Expr>,
   },
}

pub enum UnaryOp {
   Complement,
   Negate,
}