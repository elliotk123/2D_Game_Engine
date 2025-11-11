use sdl3::event::Event;
// use crate::common;
use crate::common::keyboard_interface::{KeyboardBackend, MyKeyboardEvent, MyKey};

use super::sdl3_types::SdlStateShared;

pub struct Sdl3KeyboardBackend
{
    shared_sdl : SdlStateShared
}

impl Sdl3KeyboardBackend
{
    pub fn new(shared_sdl : SdlStateShared) -> Sdl3KeyboardBackend
    {
        Sdl3KeyboardBackend
        {
            shared_sdl
        }
    }
}

impl KeyboardBackend for Sdl3KeyboardBackend
{
    fn poll_events(&mut self) -> Vec<MyKeyboardEvent>
    {
        let mut shared: std::cell::RefMut<'_, super::sdl3_types::SdlState> = self.shared_sdl.borrow_mut();
        let mut my_event_queue: Vec<MyKeyboardEvent> = Vec::new();
        for event in shared.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    if !repeat {
                        // Ignore key repeat events
                        let my_key: MyKey = MyKey::try_from(keycode as i32).unwrap();
                        let my_event: MyKeyboardEvent = MyKeyboardEvent::KeyDown(my_key);
                        my_event_queue.push(my_event);
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    let my_key: MyKey = MyKey::try_from(keycode as i32).unwrap();
                    let my_event: MyKeyboardEvent = MyKeyboardEvent::KeyUp(my_key);
                    my_event_queue.push(my_event);
                },
                _ => {}
            }
        }
        my_event_queue
    }
}