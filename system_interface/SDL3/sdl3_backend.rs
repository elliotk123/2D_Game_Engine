pub struct Sdl3Backend
{
    pub initialized : bool,
    pub context : sdl3::Sdl,
    pub graphics : Sdl3_GraphicsBackend,
    pub keyboard : KeyboardBackend,
}

impl Sdl3Backend
{
    pub fn new() -> Sdl3Backend
    {
        let context = sdl3::init().unwrap();
        Sdl3Backend
        {
            initialized : true,
            context,
            graphics : Sdl3_GraphicsBackend::new(context.clone()),
            keyboard : KeyboardBackend::new(context.clone()),
        }
    }
}