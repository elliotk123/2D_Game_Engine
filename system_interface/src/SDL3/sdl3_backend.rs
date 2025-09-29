use sdl3::Sdl;

use crate::common;
use crate::common::keyboard_interface::{KeyboardBackend, MyKeyboardEvent, MyKey};
use common::graphical_interface::GraphicsBackend;

use sdl3::video::Window;
use sdl3::surface::Surface;
use sdl3::VideoSubsystem;
use sdl3::pixels::PixelFormat;
use sdl3::pixels::PixelFormatEnum;
use sdl3::event::Event;



// use super::sdl3_graphics_backend::Sdl3GraphicsBackend;
// use super::sdl3_keyboard_backend::Sdl3KeyboardBackend;
// pub struct Sdl3Backend<'a>
// {
//     pub initialized : bool,
//     pub context : sdl3::Sdl,
//     pub graphics : Sdl3GraphicsBackend,
//     pub keyboard : Sdl3KeyboardBackend,
// }

// impl<'a> Sdl3Backend<'a>
// {
//     pub fn new() -> Sdl3Backend<'a>
//     {
//         let context : Sdl  = sdl3::init().unwrap();
//         Sdl3Backend
//         {
//             initialized : true,
//             context : context,
//             graphics : Sdl3GraphicsBackend::new(&context),
//             keyboard : Sdl3KeyboardBackend::new(&context),
//         }
//     }
// }

fn sdl_keycode_to_my_key(keycode: &sdl3::keyboard::Keycode) -> Option<MyKey>
{
    let sdl_keycode_int: i32 =  *keycode as i32;
    MyKey::try_from(sdl_keycode_int).ok()

}


pub struct Sdl3Backend
{
    pub initialized : bool,
    pub context : sdl3::Sdl,
    windows : Vec<Window>,
    surfaces : Vec<Surface<'static>>,
    pub event_pump  : sdl3::EventPump,
}

impl Sdl3Backend
{
    pub fn new() -> Sdl3Backend
    {
        let context : Sdl  = sdl3::init().unwrap();
        let event_pump: sdl3::EventPump = context.event_pump().unwrap();
        Sdl3Backend
        {
            initialized : true,
            context : context,
            event_pump : event_pump,
            windows : Vec::new(),
            surfaces : Vec::new(),

        }
    }
}

impl GraphicsBackend for Sdl3Backend
{
    unsafe fn create_window(&mut self, width : u32, height : u32, name : &str) -> usize{
        let video_subsystem: VideoSubsystem = self.context.video().unwrap();
        let window : Window = video_subsystem.window(name, width, height)
            .position_centered()
            .build()
            .unwrap();

        let surface: Surface<'static> = Surface::new(width, height, 
            PixelFormat::from_ll(PixelFormatEnum::ARGB8888.to_ll())).unwrap();

        self.windows.push(window);
        self.surfaces.push(surface);

        self.windows.len() - 1
    }

    fn render(&mut self, pixel_buffer: &Vec<u8>, window_id : usize)
    {
        // Copy pixel_buffer to surface
        let surface = &mut self.surfaces[window_id];
        surface.with_lock_mut(|buf| buf.copy_from_slice(pixel_buffer));

        // Blit surface to window and update
        let window = &self.windows[window_id];
        window.surface().unwrap().blit(&surface, None, None).unwrap();
        window.update_surface().unwrap();
    }
}

impl KeyboardBackend for Sdl3Backend{
    fn poll_events(&mut self) -> Vec<MyKeyboardEvent> {
        let mut my_event_queue: Vec<MyKeyboardEvent> = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    if !repeat {
                        // Ignore key repeat events
                        let my_key = sdl_keycode_to_my_key(&keycode);
                        let my_event = MyKeyboardEvent::KeyDown(my_key);
                        my_event_queue.push(my_event);
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    let my_key = sdl_keycode_to_my_key(&keycode);
                    let my_event = MyKeyboardEvent::KeyUp(my_key);
                    my_event_queue.push(my_event);
                },
                _ => {}
            }
        }
        my_event_queue
    }
}