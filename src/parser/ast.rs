pub struct AST {
   pub program: Program,
}

pub struct Program {
   pub func_decls: Vec<FuncDecl>,
}

pub struct FuncDecl {
   pub name: String,
   pub params: Vec<String>,
   pub body: Option<Block>,
}

pub struct VarDecl {
   pub name: String,
   pub init: Option<Expr>,
   pub line: usize,
}

pub struct Block {
   pub items: Vec<BlockItem>,
}

pub enum BlockItem {
   Stmt(Stmt),
   Decl(Decl)
}

pub enum Decl {
   VarDecl(VarDecl),
   FuncDel(FuncDecl)
}

pub enum ForInit {
   Decl(VarDecl),
   Expr(Expr),
}

pub enum Stmt {
   Return(Expr),
   Expression(Expr),
   If(Expr, Box<Stmt>, Option<Box<Stmt>>),
   Compound(Block),
   Break(String, usize),
   Continue(String, usize),
   While(Expr, Box<Stmt>, String),
   DoWhile(Box<Stmt>, Expr, String),
   For(Option<ForInit>, Option<Expr>, Option<Expr>, Box<Stmt>, String),
   Null,
}

#[derive(Clone)]
pub enum Expr {
   Integer(i64),
   Var(String, usize),
   UnaryOp(UnaryOp, Box<Expr>),
   BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
   Assignment(Box<Expr>, Box<Expr>, usize),
   Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
   FunctionCall(String, Vec<Expr>, usize),
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