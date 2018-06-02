use super::window;
use super::window::{Decorations, Frame, Fullscreen, MouseCursorControl, WindowManager};
use std::sync::{Arc, RwLock};
use std::cell::Cell;
use winit;

pub struct WinitWindow {
    base: winit::Window,
    window_manager: Arc<RwLock<window::WindowManager>>,
    events_loop: Cell<winit::EventsLoop>,
    window_shown: Cell<bool>,
    decorations_shown: Cell<bool>,
    window_fullscreen: Cell<bool>,
}

impl WinitWindow {

}

impl Decorations for WinitWindow {
    fn set_title(&mut self, title: String) { self.base.set_title(title.as_str()); }
    fn can_display(&self) -> bool { true }
    fn can_set_display(&self) -> bool { true }
    fn is_on(&self) -> bool { self.decorations_shown }
    fn set_display(&mut self, display: bool) -> Result<bool, String> {
        self.base.set_decorations(display);
        Ok(display)
    }
}


impl Frame for WinitWindow {
    fn set_position(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.base.set_position(x, y);
        Ok(())
    }
    fn set_size(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.base.set_inner_size(width, height);
        Ok(())
    }
    
    fn is_shown(&self) -> bool { self.window_shown }
    fn show(&mut self, do_show: bool) {
        if do_show { self.base.show(); }
        else { self.base.hide(); }
        self.window_shown = do_show;
    }
}

impl Fullscreen for WinitWindow {

    fn set(&mut self, fullscreen: bool) -> Result<bool, String> {
        if fullscreen {
            self.base.set_fullscreen(Some(self.base.get_current_monitor()));
        } else {
            self.base.set_fullscreen(None);
        }
        Ok(fullscreen)
    }
    fn is_fullscreen(&self) -> bool { self.window_fullscreen }

}

impl window::Window for WinitWindow {
    fn as_decorations(self: Box<Self>) -> Option<Box<Decorations>> { Some(Box::new(self)) }
    fn as_fullscreen(self: Box<Self>) -> Option<Box<Fullscreen>> { Some(Box::new(self)) }
    fn as_frame(self: Box<Self>) -> Option<Box<Frame>> { Some(Box::new(self)) }
}

