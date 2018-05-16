use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};
use std::collections::{VecDeque, HashMap};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt::{Debug, Formatter, Result as FResult};
use super::console::{ConsoleContext};

/*
 * EngineSystem is the base trait for all systems that get attached to the engine
 * the lifetime of each system should start with setup() and end with teardown()
 */
pub trait EngineSystem {
    fn get_name(&self) -> String;
    fn setup(&mut self) -> Result<String, String>;
    fn setup_console_vars(&mut self, &mut ConsoleContext) -> Result<String, String> {
        Ok("no console variables to set".to_string())
    }
    fn teardown(&mut self) -> Result<String, String>;
}

impl EngineSystem for ConsoleContext {
    fn get_name(&self) -> String { "ConsoleContext".to_string() }
    fn setup(&mut self) -> Result<String, String> {
        Ok("ConsoleContext setup finished".to_string())
    }
    fn teardown(&mut self) -> Result<String, String> {
        Ok("ConsoleContext teardown finished".to_string())
    }
}

pub type EngineSystemRef = Arc<RefCell<EngineSystem>>;

struct SystemManagerData {
    pub systems: Cell<VecDeque<EngineSystemRef>>,
    pub initialized_systems: Cell<HashMap<String, EngineSystemRef>>,
}

impl SystemManagerData {
    fn new() -> Self {
        Self {
            systems: Cell::new(VecDeque::new()),
            initialized_systems: Cell::new(HashMap::new())
        }
    }
}

#[derive(Clone)]
pub struct SystemManager {
    inner: Rc<RefCell<SystemManagerData>>,
}


impl SystemManager {
    fn get() -> Self {

        static mut SINGLETON: *const SystemManager = 0 as *const SystemManager;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                let manager = Self {
                    inner: Rc::new(RefCell::new(SystemManagerData::new()))
                };

                SINGLETON = mem::transmute(Box::new(manager));
            });
            
            (*SINGLETON).clone()
        }
    }

    fn register_system(&self, system: Box<EngineSystem>) -> &Self {
        let systems = self.inner.borrow_mut();
        
        self
    }
}