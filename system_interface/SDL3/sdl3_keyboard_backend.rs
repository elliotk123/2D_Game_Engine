pub struct Sdl3KeyboardBackend
{
    pub initialized : bool,
    pub event_pump  : sdl3::EventPump,
}

impl Sdl3KeyboardBackend
{
    pub fn new(context : sdl3::Sdl) -> Sdl3KeyboardBackend
    {
        let event_pump = context.event_pump().unwrap();
        Sdl3KeyboardBackend
        {
            initialized : true,
            event_pump
        }
    }

    pub fn poll_keys(&mut self) -> Vec<KeyEvent>
    {
        let mut my_event_queue: Vec<MyKeyboardEvent> = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    if !repeat {
                        // Ignore key repeat events
                        let my_key = MyKey::from(keycode);
                        let my_event = MyKeyboardEvent::KeyDown(my_key);
                        my_event_queue.push(my_event);
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    let my_key = MyKey::from(keycode);
                    let my_event = MyKeyboardEvent::KeyUp(my_key);
                    my_event_queue.push(my_event);
                },
                _ => {}
            }
        }
    }
}