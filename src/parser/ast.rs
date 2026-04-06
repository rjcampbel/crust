use crate::validator::symbol_table::SymbolTable;

use std::hash::{Hash, Hasher};

pub struct AST {
   pub program: Program,
   pub symbol_table: SymbolTable,
}

pub struct Program {
   pub decls: Vec<Decl>,
}

#[derive(Clone)]
pub struct FuncDecl {
   pub name: String,
   pub params: Vec<String>,
   pub body: Option<Block>,
   pub storage_class: Option<StorageClass>,
   pub line_number: usize,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Block {
   pub items: Vec<BlockItem>,
}

#[derive(Clone)]
pub enum BlockItem {
   Stmt(Stmt),
   Decl(Decl)
}

#[derive(Clone)]
pub enum Decl {
   VarDecl(VarDecl),
   FuncDecl(FuncDecl)
}

#[derive(Clone)]
pub enum ForInit {
   Decl(VarDecl),
   Expr(Expr),
}

#[derive(Clone)]
pub enum Stmt {
   Return(Expr, Vec<Label>, ()),
   Expression(Expr, Vec<Label>, ()),
   If(Expr, Box<Stmt>, Option<Box<Stmt>>, Vec<Label>, ()),
   Compound(Block, Vec<Label>, ()),
   Break(String, Vec<Label>, usize),
   Continue(String, Vec<Label>, usize),
   While(Expr, Box<Stmt>, Vec<Label>, usize),
   DoWhile(Box<Stmt>, Expr, Vec<Label>, usize),
   For(Option<ForInit>, Option<Expr>, Option<Expr>, Box<Stmt>, Vec<Label>, usize),
   Goto(String, Vec<Label>, usize),
   Switch(Expr, Box<Stmt>, Vec<Label>, SwitchInfo, ()),
   Null(Vec<Label>, ()),
}

#[derive(Clone)]
pub struct SwitchInfo {
   pub cases: Vec<CaseInfo>,
   pub default: Option<Label>,
   pub end_label: String,
}

#[derive(Clone)]
pub struct CaseInfo {
   pub value: Expr,
   pub label: Label,
   pub line_number: usize,
}

impl Eq for CaseInfo {}

impl Hash for CaseInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for CaseInfo {
   fn eq(&self, other: &CaseInfo) -> bool {
      self.value == other.value
   }
}

#[derive(Clone, Eq, Hash)]
pub struct Label {
   pub name: String,
   pub line_number: usize,
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Expr {
   Integer(i64),
   Var(String, usize),
   UnaryOp(UnaryOp, Box<Expr>, usize),
   BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
   Assignment(Box<Expr>, Box<Expr>, usize),
   Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
   FunctionCall(String, Vec<Expr>, usize),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
   Complement,
   Negate,
   Not,
   PreIncrement,
   PreDecrement,
   PostIncrement,
   PostDecrement,
}

#[derive(Clone, PartialEq, Eq, Hash)]
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