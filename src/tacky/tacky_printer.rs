use crate::tacky::tacky::*;

pub fn print_tacky_ast(tacky_ast: &TackyAST) {
   println!("Tacky AST:");
   for func in &tacky_ast.program.funcs {
      println!("Tacky Function: {}", func.name);
      for instr in &func.instrs {
         match instr {
            Instr::Return(val) => {
               match val {
                  Val::Integer(i) => println!("  RETURN {}", i),
                  Val::Var(v) => println!("  RETURN {}", v),
               }
            },
            Instr::Unary(operator, src, dest) => {
               println!("  {:?} {:?} -> {:?}", operator, src, dest);
            },
            Instr::Binary(operator, left, right, dest) => {
               println!("  {:?} {:?}, {:?} -> {:?}", operator, left, right, dest);
            },
            Instr::Copy(src, dest) => {
               println!("  COPY {:?} -> {:?}", src, dest);
            },
            Instr::Jump(label) => {
               println!("  JUMP {:?}", label);
            },
            Instr::JumpIfZero(condition, target) => {
               println!("  JZ {:?} -> {:?}", condition, target);
            },
            Instr::JumpIfNotZero(condition, target) => {
               println!("  JNZ {:?} -> {:?} ", condition, target);
            },
            Instr::Label(label) => {
               println!("  LABEL {:?}", label);
            },
            Instr::FuncCall(func_name, args, dest) => {
               let arg_strs: Vec<String> = args.iter().map(|arg| {
                  match arg {
                     Val::Integer(i) => i.to_string(),
                     Val::Var(v) => v.clone(),
                  }
               }).collect();
               println!("  CALL {}({}) -> {:?}", func_name, arg_strs.join(", "), dest);
            },
         }
      }
   }
}