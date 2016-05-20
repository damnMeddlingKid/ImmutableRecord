use std::collections::HashMap;
use std::vec::Vec;

pub struct Record {
  get_func: fn(&Record, &str) -> PythonObject;
  fields: Hashmap<&str, PythonObject>;
  retained_fields: Hashmap<&str, PythonObject>;
  new_keys: Vec<&str>;
}

impl Record {

  fn get_value(&self, key: &str) -> PythonObject {

  }

  fn indirect_get_value(&self, key: &str) -> PythonObject {

  }

  fn persist_value_for_key(&self, key: &str, value: PythonObject) {
    if self.fields.contains_key(key) {
      self.retained_fields.insert(key, value);
    } else {
      self.new_keys.push(key);
    }
  }

  fn set_indirect_access(&self) {
    self.get_func = self.indirect_get_value;
    self.new_keys = Vec<&str>::new();
  }

  pub fn get(&self, key: &str, value: &PythonObject) -> Record {
    self.get_func(key)
  }

  pub fn set(&self, key: &str, value: &PythonObject) -> Record {
    self.set_indirect_access()
    self.persist_value_for_key(key, value)
    
  }
}
