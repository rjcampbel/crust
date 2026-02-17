use super::stack_allocator::StackAllocator;

pub struct Assembly {
   pub program: AssemblyProgram,
}

pub struct AssemblyProgram {
   pub functions: Vec<Function>,
}


pub struct Function {
   pub name: String,
   pub instructions: Vec<Instruction>,
   pub stack_allocator: StackAllocator,
}

#[derive(Clone)]
pub enum Instruction {
   Mov(Operand, Operand),
   Movb(Operand, Operand),
   Unary(UnaryOp, Operand),
   Binary(BinaryOp, Operand, Operand),
   Cmp(Operand, Operand),
   Shl(Operand, Operand),
   Shr(Operand, Operand),
   Idiv(Operand),
   Cdq,
   Jmp(String),
   JmpCC(ConditionCode, String),
   SetCC(ConditionCode, Operand),
   Label(String),
   AllocateStack(i64),
   DeallocateStack(i64),
   Push(Operand),
   Call(String),
   Return
}

#[derive(Clone)]
pub enum UnaryOp {
   Neg,
   Not,
}

#[derive(Clone)]
pub enum BinaryOp {
   Add,
   Sub,
   Mult,
   BitwiseAnd,
   BitwiseOr,
   BitwiseXor,
}

#[derive(Debug,Clone)]
pub enum Operand {
   Immediate(i64),
   Register(Register),
   Pseudo(String),
   Stack(i64),
}

#[derive(Debug, Clone)]
pub enum ConditionCode {
   E,
   NE,
   G,
   GE,
   L,
   LE
}

#[derive(Debug,Clone)]
pub enum Register {
   AX(usize),
   DX(usize),
   CX(usize),
   DI(usize),
   SI(usize),
   R8(usize),
   R9(usize),
   R10(usize),
   R11(usize),
}
