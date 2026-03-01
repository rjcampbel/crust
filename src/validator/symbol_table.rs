use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum DeclType {
   Int,
   Func(usize)
}

#[derive(Copy, Clone)]
pub enum InitialValue {
   Tentative,
   Initialized(i64),
   NoInitializer
}

pub enum Attrs {
   FuncAttr {
      defined: bool,
      global: bool
   },
   StaticAttr {
      initial_value: InitialValue,
      global: bool
   },
   LocalAttr
}
pub struct TypeInfo {
   pub decl_type: DeclType,
   pub attrs: Attrs
}

pub type SymbolTable = HashMap<String, TypeInfo>;