use std::collections::HashMap;

pub struct StackAllocator {
   offset: i64,
   map: HashMap<String, i64>
}

impl StackAllocator {
   pub fn new() -> Self {
      StackAllocator {
         offset: 0,
         map: HashMap::new()
      }
   }

   pub fn allocate(&mut self, name: String, bytes: i64) -> i64 {
      *self.map.entry(name).or_insert_with(|| {
         self.offset = self.offset + bytes;
         self.offset
      })
   }

   pub fn get(&self) -> i64 {
      self.offset
   }
}