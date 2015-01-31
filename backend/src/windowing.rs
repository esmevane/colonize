use utility::Bounds;
use rendering::TextAlign;

pub trait WindowTrait {
    fn clear(&mut self);
    fn get_bounds(&self) -> Bounds;
    fn print_message(&mut self, x: i32, y: i32,
        align: TextAlign, text: &str);
    fn buffer_message(&mut self, text: &str);
    fn flush_message_buffer(&mut self);
    fn get_messages(&self) -> Vec<Box<String>>;
}
