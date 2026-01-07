pub struct AssemblyAST {
   pub program: AssemblyProgram,
}

pub enum AssemblyProgram {
   Function {
      name: String,
      instructions: Vec<Instruction>,
   },
}

pub enum Instruction {
   Mov(Operand, Operand),
   Return
}

#[derive(Debug)]
pub enum Operand {
   Immediate(u64),
   Register,
}