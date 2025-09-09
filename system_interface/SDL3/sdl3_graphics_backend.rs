use sdl3::render::Canvas;

pub struct Sdl3_GraphicsBackend
{
    sdl_context : sdl3::Sdl,
    canvases : Vec<sdl3::render::Canvas<sdl3::video::Window>>
}

impl Sdl3_GraphicsBackend
{
    pub fn new(sdl_context : sdl3::Sdl) -> Sdl3_GraphicsBackend
    {
        Sdl3_GraphicsBackend
        {
            sdl_context,
            canvases : Vec::new()
        }
    }

    pub fn create_window(width : u32, height : u32, name "demo") -> u32
    {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(name, width, height)
        .position_centered()
        .build()
        .unwrap();

        canvases.push(window.into_canvas());
        (canvases.len() - 1) as u32
        let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::RGBA8888,
            TextureAccess::Streaming,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    }

    pub fn render(&mut self, pixel_buffer: &Vec<u8>, window_id : u32)
    {
        // Convert pixel_buffer to a texture and render it using the canvas
        // This is a placeholder implementation
        self.canvas.set_draw_color(sdl3::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}