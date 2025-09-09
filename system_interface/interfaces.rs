struct Interfaces{
    graphical_interface : GraphicalInterface,
    keyboard_interface : KeyboardInterface
}

impl Interfaces
{
    // This method needs to be auto generated from a config file as it is dependent on the target platform
    pub fn new() -> Interfaces
    {
        // initilise any backends here that control multiple interfaces
        let sdl3_backend = Sdl3Backend::new();

        // Create a graphical interface with the SDL3 graphics backend
        let graphics_backends: Vec<Box<dyn GraphicsBackend>> = vec![Box::new(sdl3_backend.graphics)];
        
        // Create a keyboard interface with the SDL3 keyboard backend
        let keyboard_backends: Vec<Box<dyn KeyboardBackend>> = vec![Box::new(sdl3_backend.keyboard)];

        Interfaces
        {
            graphical_interface : GraphicalInterface::new(graphics_backends);
            keyboard_interface : KeyboardInterface::new(keyboard_backends);
        }
    }
}