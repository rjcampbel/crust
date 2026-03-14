use crate::validator::symbol_table::SymbolTable;

pub struct AST {
   pub program: Program,
   pub symbol_table: SymbolTable,
}

pub struct Program {
   pub decls: Vec<Decl>,
}

pub struct FuncDecl {
   pub name: String,
   pub params: Vec<String>,
   pub body: Option<Block>,
   pub storage_class: Option<StorageClass>,
   pub line_number: usize,
}

pub struct VarDecl {
   pub name: String,
   pub init: Option<Expr>,
   pub storage_class: Option<StorageClass>,
   pub line_number: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum StorageClass {
   Static,
   Extern,
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
   FuncDecl(FuncDecl)
}

pub enum ForInit {
   Decl(VarDecl),
   Expr(Expr),
}

pub enum Stmt {
   Return(Expr, Vec<Label>, usize),
   Expression(Expr, Vec<Label>, usize),
   If(Expr, Box<Stmt>, Option<Box<Stmt>>, Vec<Label>, usize),
   Compound(Block, Vec<Label>, usize),
   Break(Label, Vec<Label>, usize),
   Continue(Label, Vec<Label>, usize),
   While(Expr, Box<Stmt>, Vec<Label>, usize),
   DoWhile(Box<Stmt>, Expr, Vec<Label>, usize),
   For(Option<ForInit>, Option<Expr>, Option<Expr>, Box<Stmt>, Vec<Label>, usize),
   Goto(String, Vec<Label>, usize),
   Null(Vec<Label>, usize),
}

#[derive(Clone, Eq, Hash)]
pub struct Label {
   pub name: String,
   pub line_number: usize,
}

impl From<&str> for Label {
   fn from(item: &str) -> Self {
      Self { name: item.to_string(), line_number: 0 }
   }
}

impl PartialEq for Label {
   fn eq(&self, label: &Label) -> bool {
      self.name == label.name
   }
}

impl Label {
   pub fn new(name: String, line_number: usize) -> Self {
      Self {
         name,
         line_number
      }
   }
}

#[derive(Clone)]
pub enum Expr {
   Integer(i64),
   Var(String, usize),
   UnaryOp(UnaryOp, Box<Expr>, usize),
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
   PreIncrement,
   PreDecrement,
   PostIncrement,
   PostDecrement,
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