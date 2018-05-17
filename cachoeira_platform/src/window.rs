extern crate cachoeira_core;

use cachoeira_core::engine_system::EngineSystem;
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;

pub trait ShowHide {
    fn is_shown(&self) -> bool;
    fn show(&mut self, do_show: bool);
}

pub trait Title {
    fn set(&mut self, title: String);
    fn get(&self) -> String;
}

pub trait Titlebar {
    fn can_display(&self) -> bool;
    fn can_set_display(&self) -> bool;
    fn is_on(&self) -> bool;
    fn set_display(&mut self, display: bool) -> Result<bool, String>;
    fn toggle_display(&mut self) -> Result<bool, String> {
        self.set_display(!self.is_on())
    }
}

pub trait Frame {
    fn set_geometry(&mut self, width: u32, height: u32, x: i32, y: i32) -> Result<String, String>;
    fn get_position(&self) -> Option<(i32, i32)>;
    fn get_size(&self) -> Option<(u32, u32)>;
    fn is_resizable(&self) -> bool;
}

pub trait Fullscreen {
    fn set(&mut self, fullscreen: bool) -> Result<bool, String>;
    fn is_fullscreen(&self) -> bool;
    fn toggle(&mut self) -> Result<bool, String> {
        self.set(!self.is_fullscreen())
    }
}



pub trait Window {
    
}

struct WindowManager {
    windows: Vec<Arc<RwLock<Window>>>,
}

impl EngineSystem for WindowManager {
    fn get_name(&self) -> String {
        "WindowManager".to_string()
    }
    fn setup(&mut self) -> Result<String, String> {
        self.windows = Vec::new();
        Ok("WindowManager setup successful".to_string())
    }
    fn teardown(&mut self) -> Result<String, String> {
        self.windows.clear();
        Ok("WindowManager teardown successful".to_string())
    }
}