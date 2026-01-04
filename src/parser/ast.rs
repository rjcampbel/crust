pub enum Program {
   Function(Function),
}

pub struct Function {
   pub name: String,
   pub stmt: Stmt,
}

pub enum Stmt {
   Return(Expr)
}

pub enum Expr {
   Integer(u64),
}