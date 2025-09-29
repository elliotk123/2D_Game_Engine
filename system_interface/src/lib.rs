mod SDL3;
mod common;

use common::graphical_interface::{GraphicalInterface, GraphicsBackend};
use common::keyboard_interface::{KeyboardInterface, KeyboardBackend};
use SDL3::sdl3_backend::Sdl3Backend;

struct SystemInterface{
    graphical_interface : GraphicalInterface,
    keyboard_interface : KeyboardInterface
}


// This method needs to be auto generated from a config file as it is dependent on the target platform
pub fn init_system_interface() -> SystemInterface
{
    // initilise any backends here that control multiple interfaces
    let sdl3_backend = Sdl3Backend::new();

    // Create a graphical interface with the SDL3 graphics backend
    let graphics_backends: Vec<Box<dyn GraphicsBackend>> = vec![Box::new(sdl3_backend)];
    
    // Create a keyboard interface with the SDL3 keyboard backend
    let keyboard_backends: Vec<Box<dyn KeyboardBackend>> = vec![Box::new(sdl3_backend)];

    SystemInterface
    {
        graphical_interface : GraphicalInterface::new(graphics_backends),
        keyboard_interface : KeyboardInterface::new(keyboard_backends),
    }
}