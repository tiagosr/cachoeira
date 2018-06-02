extern crate cachoeira_core;

use cachoeira_core::engine_system::EngineSystem;
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;


pub trait Decorations {
    fn set_title(self: Box<Self>, title: String);
    fn can_display(self: Box<Self>) -> bool;
    fn can_set_display(self: Box<Self>) -> bool;
    fn is_on(self: Box<Self>f) -> bool;
    fn set_display(self: Box<Self>, display: bool) -> Result<bool, String>;
    fn toggle_display(self: Box<Self>) -> Result<bool, String> {
        self.set_display(!self.is_on())
    }
}

pub trait Frame {
    fn set_size(self: Box<Self>, width: u32, height: u32) -> Result<(), String>;
    fn set_position(self: Box<Self>, x: i32, y: i32) -> Result<(), String>;
    fn get_position(self: Box<Self>) -> Option<(i32, i32)> { None }
    fn get_inner_position(self: Box<Self>) -> Option<(i32, i32)> { None }
    fn get_size(self: Box<Self>) -> Option<(u32, u32)> { None }
    fn get_inner_size(self: Box<Self>) -> Option<(u32, u32)> { None }
    fn set_geometry(self: Box<Self>, width: u32, height: u32, x: i32, y: i32) -> Result<(), String> {
        match self.set_size(width, height) {
            Ok(_) => self.set_position(x, y),
            Err(size_failed) => Err(size_failed)
        }
    }
    fn is_resizable(self: Box<Self>) -> bool { false }
    fn set_maximized(self: Box<Self>, maximized: bool) -> Result<(), String> { unimplemented!() }
    fn is_shown(self: Box<Self>) -> bool { false }
    fn show(self: Box<Self>, do_show: bool);
}

pub trait Fullscreen {
    fn set(self: Box<Self>, fullscreen: bool) -> Result<(), String>;
    fn is_fullscreen(self: Box<Self>) -> bool;
    fn toggle(self: Box<Self>) -> Result<bool, String> {
        self.set(!self.is_fullscreen())
    }
    fn get_monitors_count(self: Box<Self>) -> isize { 1 }
    fn get_monitor_name(self: Box<Self>, id: isize) -> Option<String> {
        if id == 1 {
            Some("Default".to_string())
        } else {
            None
        }
    }
    fn set_monitor(self: Box<Self>, id: isize) -> Result<(), String> {
        if id == 1 {
            Ok(())
        } else {
            Err("Invalid monitor".to_string())
        }
    }

}

pub trait MouseCursorControl {
    fn get_cursor_pos(self: Box<Self>) -> Option<(i32, i32)> { None }
    fn is_cursor_visible(self: Box<Self>) -> bool { false }
    fn set_cursor_visible(self: Box<Self>, visible: bool) -> Result<(), String>;
}



pub trait Window {
    fn as_decorations(self: Box<Self>) -> Option<Box<Decorations>> { None }
    fn as_fullscreen(self: Box<Self>) -> Option<Box<Fullscreen>> { None }
    fn as_frame(self: Box<Self>) -> Option<Box<Frame>> { None }
    fn as_mouse_cursor_control(self: Box<Self>) -> Option<Box<MouseCursorControl>> { None }
}

pub trait WindowBuilder {
    fn build(self: Box<Self>) -> Result<Box<Window>, String>;

}

pub struct WindowManager {

}

impl WindowManager {
    fn new_window(&mut self) -> Box<WindowBuilder> {

    }
}

impl EngineSystem for WindowManager {
    fn get_name(&self) -> String {
        "WindowManager".to_string()
    }
    fn setup(&mut self) -> Result<(), String> {
        self.windows = Vec::new();
        Ok(())
    }
    fn teardown(&mut self) -> Result<(), String> {
        self.windows.clear();
        Ok(())
    }
}