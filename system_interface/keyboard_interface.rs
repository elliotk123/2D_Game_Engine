#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MyKeyboardEvent {
    KeyDown(MyKey),
    KeyUp(MyKey),
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MyKey {
    A,
    B,
    // Add other keys you want to handle
    Space,
    Escape,
    Unknown,
}

trait KeyboardBackend
{
    fn poll_events(&mut self) -> Vec<MyKeyboardEvent>;

}


pub struct KeyboardInterface
{
    pub backends : Vec<Box<dyn KeyboardBackend>>,
}