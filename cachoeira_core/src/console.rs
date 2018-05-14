use std::collections::HashMap;
use std::sync::mpsc;

trait ConsoleVar {
    fn set(&mut self, &str);
    fn get(&self) -> &str;
}

#[derive(Debug)]
struct ConsoleContext {
    vars: HashMap<&'static str, &mut ConsoleVar>,

}

impl ConsoleContext {
    fn query_var(&self, key: &'static str) -> Option<&str> {
        match self.vars.get(key) {
            None => None,
            Some(val) => Option(val.get()),
        }
    }
    fn write_var(&mut self, key: &str, val: &str) -> &mut Self {
        
        self
    }
    fn add_var(&mut self, key: &str, var: &mut ConsoleVar) -> &mut Self {
        self.vars.insert(key, var);
        self
    }
}

impl Default for ConsoleContext {
    fn default() -> Self {
        Self { vars = HashMap<&str, str>::new(), }
    }
}