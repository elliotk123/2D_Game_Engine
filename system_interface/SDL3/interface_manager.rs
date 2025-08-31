pub struct InterfaceManager
{
    pub initialized : bool,
    pub context : sdl3::Sdl
}

impl InterfaceManager
{
    pub fn new() -> InterfaceManager
    {
        let context = sdl3::init().unwrap();
        InterfaceManager
        {
            initialized : true,
            context
        }
    }
}