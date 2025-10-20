use crate::common;

use sdl3::video::Window;
use sdl3::surface::Surface;
use sdl3::VideoSubsystem;
use sdl3::pixels::PixelFormat;
use sdl3::pixels::PixelFormatEnum;

use common::graphical_interface::GraphicsBackend;

pub struct Sdl3GraphicsBackend
{
    sdl_context : &sdl3::Sdl,
    windows : Vec<Window>,
    surfaces : Vec<Surface<'static>>,
}
impl Sdl3GraphicsBackend{
    pub fn new(sdl_context : &sdl3::Sdl) -> Sdl3GraphicsBackend{
        Sdl3GraphicsBackend
        {
            sdl_context,
            windows : Vec::new(),
            surfaces : Vec::new()
        }
    }
}


impl GraphicsBackend for Sdl3GraphicsBackend
{
    unsafe fn create_window(&mut self, width : u32, height : u32, name : &str) -> usize{
        let video_subsystem: VideoSubsystem = self.sdl_context.video().unwrap();
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