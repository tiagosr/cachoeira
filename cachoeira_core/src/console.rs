use std::collections::HashMap;
use std::sync::mpsc;
use std::result;
use std::cell::RefCell;
use std::ops::Deref;
use std::borrow::Borrow;

pub type ConsoleVarResult  = Result<Option<String>, String>;

pub trait ConsoleVar {
    fn set(&mut self, String) -> ConsoleVarResult;
    fn get(&self) -> String;
}

#[derive(Debug)]
pub struct ConsoleVarString {
    value: String,
} 

impl ConsoleVar for ConsoleVarString {
    fn set(&mut self, value: String) -> ConsoleVarResult {
        self.value = value;
        Ok(None)
    }
    fn get(&self) -> String {
        self.value.clone()
    }
}


type ConsoleContextHashmap = HashMap<String, RefCell<Box<ConsoleVar>>>;


pub struct ConsoleContext {
    vars: ConsoleContextHashmap,
}

impl ConsoleContext {
    fn query_var(&self, key: &str) -> Option<String> {
        match self.vars.get(key) {
            None => None,
            Some(val) => Some((*val.borrow().deref()).get()),
        }
    }
    fn write_var(&mut self, key: &str, val: &str) -> Option<ConsoleVarResult> {
        match self.vars.get(key) {
            None => None,
            Some(result) => Some(result.borrow_mut().set(val.to_string())),
        }
    }
    fn add_var(&mut self, key: &str, var: RefCell<Box<ConsoleVar>>) -> &mut Self {
        self.vars.insert(key.to_string(), var);
        self
    }
}

impl Default for ConsoleContext {
    fn default() -> Self {
        Self { vars: ConsoleContextHashmap::new(), }
    }
}