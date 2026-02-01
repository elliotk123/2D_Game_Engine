use sdl3::video::{Window, WindowContext};
use sdl3::pixels::PixelFormat;
use sdl3::render::{Texture, TextureCreator, WindowCanvas};

use super::sdl3_types::SdlStateShared;

use super::super::common::graphical_interface::{
    GraphicsBackend,
    WindowData
};

pub struct Sdl3GraphicsBackend
{
    shared_sdl : SdlStateShared,
    canvases : Vec<WindowCanvas>
}

impl Sdl3GraphicsBackend{
    pub fn new(shared_sdl : SdlStateShared) -> Sdl3GraphicsBackend{
        Sdl3GraphicsBackend
        {
            shared_sdl,
            canvases : Vec::new()
        }
    }
}


impl GraphicsBackend for Sdl3GraphicsBackend
{
    fn create_window(&mut self, width : u32, height : u32, name : &str) -> WindowData{
        let shared: std::cell::RefMut<'_, super::sdl3_types::SdlState> = self.shared_sdl.borrow_mut();
        let window : Window = shared.video_subsystem.window(name, width, height)
            .position_centered()
            .build()
            .unwrap();
        let (width, height) = window.size_in_pixels();
        self.canvases.push(window.into_canvas());

        WindowData{
            id : self.canvases.len() - 1,
            height : height as usize,
            width : width as usize
        }
    }

    fn create_fullscreen_window(&mut self, name : &str) -> WindowData {
        let shared: std::cell::RefMut<'_, super::sdl3_types::SdlState> = self.shared_sdl.borrow_mut();
        let window : Window = shared.video_subsystem.window(name, 0, 0)
            .fullscreen()
            .position_centered()
            .build()
            .unwrap();
        let (width, height) = window.size_in_pixels();
        self.canvases.push(window.into_canvas());
  
        WindowData{
            id : self.canvases.len() - 1,
            height : height as usize,
            width : width as usize
        }     
    }

    fn render(&mut self, pixel_buffer: &Vec<u8>, window_id : usize)
    {
        let (width, height) = self.canvases[window_id].window().size();
        let bytes_per_pixel = 4; // If using ARGB8888
        let pitch = width as usize * bytes_per_pixel;

        let texture_creator: TextureCreator<WindowContext> = self.canvases.last().unwrap().texture_creator();

        let mut texture: Texture<'_> = texture_creator
            .create_texture_streaming(
                PixelFormat::ARGB8888,
                // Use a 32-bit format like ARGB8888 (4 bytes per pixel)
                width, 
                height
            ).unwrap();

        // 'None' for the rect updates the entire texture
        texture.update(None, pixel_buffer, pitch)
            .expect("Failed to update texture");


        // Copy the texture to the entire rendering target
        self.canvases[window_id].copy(&texture, None, None)
            .expect("Failed to copy texture to canvas");

        // Display the result
        self.canvases[window_id].present();
    }
}