use std::collections::HashMap;
use std::sync::mpsc;
use std::result;
use std::rc::Rc;

pub type ConsoleVarResult  = Result<Option<String>, String>;

#[derive(Debug)]
struct ConsoleVarItem {
}

trait ConsoleVar {
    fn set(&mut self, String) -> ConsoleVarResult;
    fn get(&self) -> String;
}

impl ConsoleVar for ConsoleVarItem {
    fn set(&mut self, value: String) -> ConsoleVarResult {
        unimplemented!()
    }
    fn get(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ConsoleVarString {
    base: ConsoleVarItem,
    value: String,
} 

impl ConsoleVar for ConsoleVarString {
    fn set(&mut self, value: String) -> ConsoleVarResult {
        self.value = value;
        Ok(None)
    }
    fn get(&self) -> String {
        self.value
    }
}


type ConsoleContextHashmap = HashMap<&'static str, Rc<ConsoleVarItem>>;
#[derive(Debug)]
struct ConsoleContext {
    vars: ConsoleContextHashmap,

}

impl ConsoleContext {
    fn query_var(&self, key: &'static str) -> Option<String> {
        match self.vars.get(key) {
            None => None,
            Some(val) => Some((val as &mut ConsoleVar).get()),
        }
    }
    fn write_var(&mut self, key: &'static str, val: &str) -> Option<ConsoleVarResult> {
        match self.vars.get(key) {
            None => None,
            Some(result) => Some(result.set(val.to_string())),
        }
    }
    fn add_var(&mut self, key: &str, var: &mut ConsoleVar) -> &mut Self {
        self.vars.insert(key, var);
        self
    }
}

impl Default for ConsoleContext {
    fn default() -> Self {
        Self { vars: ConsoleContextHashmap::new(), }
    }
}