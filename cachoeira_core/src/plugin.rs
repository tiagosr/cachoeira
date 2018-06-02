extern crate libloading;
use libloading::{Library, Symbol};

use super::engine_system;

pub trait Plugin {

}

pub type PluginLoadFunc = unsafe fn() -> Result<String, String>;

pub fn load_plugin(filename: String) -> Result<String, String> {
    let lib = Library::new(filename).unwrap();
    unsafe {
        let func: Symbol<PluginLoadFunc> = lib.get(b"load_plugin").unwrap();
        func()
    }
}