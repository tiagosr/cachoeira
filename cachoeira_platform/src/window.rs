use winit;

#[derive(Debug)]
pub struct Window {}

trait HasTitle {
    fn set_title(&mut self, title: str) -> Self;
    fn get_title(&self) -> str;
}
