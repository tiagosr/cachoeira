use super::window;
use winit;

pub struct WinitWindow {
    base: winit::Window,
    events_loop: mut winit::EventsLoop,

}

impl WinitWindow {

}

impl window::Title for WinitWindow {
    fn set(&mut self, title: String) {
        self.base.set_title(title.as_str())
    }
    fn get(&self) {
        self.base.get_title().to_string()
    }
}
