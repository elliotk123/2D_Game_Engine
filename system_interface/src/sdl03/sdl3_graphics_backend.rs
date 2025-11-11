use crate::common;

use sdl3::video::Window;
use sdl3::surface::Surface;
use sdl3::pixels::PixelFormat;
use sdl3::pixels::PixelFormatEnum;

use super::sdl3_types::SdlStateShared;

use common::graphical_interface::GraphicsBackend;

pub struct Sdl3GraphicsBackend
{
    shared_sdl : SdlStateShared,
    windows : Vec<Window>,
    surfaces : Vec<Surface<'static>>,
}
impl Sdl3GraphicsBackend{
    pub fn new(shared_sdl : SdlStateShared) -> Sdl3GraphicsBackend{
        Sdl3GraphicsBackend
        {
            shared_sdl,
            windows : Vec::new(),
            surfaces : Vec::new()
        }
    }
}


impl GraphicsBackend for Sdl3GraphicsBackend
{
    unsafe fn create_window(&mut self, width : u32, height : u32, name : &str) -> usize{
        // let video_subsystem: VideoSubsystem = self.shared_sdl.borrow_mut().context.video().unwrap();
        let shared: std::cell::RefMut<'_, super::sdl3_types::SdlState> = self.shared_sdl.borrow_mut();
        let window : Window = shared.video_subsystem.window(name, width, height)
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
        let shared: std::cell::RefMut<'_, super::sdl3_types::SdlState> = self.shared_sdl.borrow_mut();
        // Copy pixel_buffer to surface
        let surface: &mut Surface<'static> = &mut self.surfaces[window_id];
        surface.with_lock_mut(|buf: &mut [u8]| buf.copy_from_slice(pixel_buffer));

        // Blit surface to window and update
        let window_surface: &mut sdl3::video::WindowSurfaceRef<'_> = &mut self.windows[window_id].surface(&shared.event_pump).unwrap();
        window_surface.blit(None, surface, None).unwrap();
        window_surface.update_window().unwrap();
    }
}