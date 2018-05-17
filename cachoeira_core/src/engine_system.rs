use std::sync::{Arc, RwLock, Once, ONCE_INIT};
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

pub type EngineSystemRef = Arc<RwLock<EngineSystem>>;
type EngineSystemMap = HashMap<String, EngineSystemRef>;

struct SystemManagerData {
    pub systems: RwLock<VecDeque<EngineSystemRef>>,
    pub initialized_systems: RwLock<HashMap<String, EngineSystemRef>>,
}

impl SystemManagerData {
    fn new() -> Self {
        Self {
            systems: RwLock::new(VecDeque::new()),
            initialized_systems: RwLock::new(HashMap::new()),
        }
    }
    fn add_system(&mut self, system: EngineSystemRef) -> Result<String, String> {
        let name = system.read().unwrap().get_name();
        match system.write().unwrap().setup() {
            Ok(message) => {
                self.initialized_systems.write().unwrap().insert(name.clone(), system.clone());
                Ok(format!("{} added: {}", name, message))
            },
            Err(message) => Err(format!("{} add FAILED! {}", name, message))
        }
    }

    fn remove_system_by_name(&mut self, name: String) -> Result<String, String> {
        match self.initialized_systems.write().unwrap().get(&name) {
            None => Err(format!("{} not registered", name)),
            Some(found) => {
                match found.write().unwrap().teardown() {
                    Ok(message) => {
                        self.initialized_systems.write().unwrap().remove(&name);
                        Ok(format!("{} removed: {}", name, message))
                    },
                    Err(message) => Err(format!("{} removal FAILED! {}", name, message))
                }
            }
        }
    }

    fn remove_system(&mut self, system: EngineSystemRef) -> Result<String, String> {
        self.remove_system_by_name(system.read().unwrap().get_name())
    }
}

#[derive(Clone)]
pub struct SystemManager {
    inner: Arc<RwLock<SystemManagerData>>,
}


impl SystemManager {
    fn get() -> Self {

        static mut SINGLETON: *const SystemManager = 0 as *const SystemManager;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                let manager = Self {
                    inner: Arc::new(RwLock::new(SystemManagerData::new()))
                };

                SINGLETON = mem::transmute(Box::new(manager));
            });
            
            (*SINGLETON).clone()
        }
    }

    fn register_system(&self, system: EngineSystemRef) -> Result<String, String> {
        self.inner.write().unwrap().add_system(system)
    }

    fn unregister_system(&self, system: EngineSystemRef) -> Result<String, String> {
        self.inner.write().unwrap().remove_system(system)
    }
}