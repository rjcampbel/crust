pub enum Program {
   Function(Function),
}

pub struct Function {
   pub name: String,
   pub instructions: Vec<Instruction>,
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