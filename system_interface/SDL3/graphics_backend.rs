use sdl3::render::Canvas;

pub struct GraphicsBackend
{
    pub initialized : bool,
    pub canvas : sdl3::render::Canvas<sdl3::video::Window>
}