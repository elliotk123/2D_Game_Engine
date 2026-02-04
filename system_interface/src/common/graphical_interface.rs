pub trait GraphicsBackend
{
    fn render(&mut self, pixel_buffer: &Vec<u8>, window_id: usize);
    fn create_window(&mut self, width: u32, height: u32, name : &str) -> WindowData;
    fn create_fullscreen_window(&mut self, name : &str) -> WindowData;
}

pub struct WindowData{
    pub id : usize,
    pub width : usize,
    pub height : usize
}

pub struct GraphicalInterface
{
    graphics_backends : Vec<Box<dyn GraphicsBackend>>
}

impl GraphicalInterface
{
    pub fn new(graphics_backends : Vec<Box<dyn GraphicsBackend>>) -> GraphicalInterface
    {
        GraphicalInterface
        {
            graphics_backends
        }
    }

    pub fn create_window(&mut self, width:u32, height:u32, name : &str, backend : usize) -> WindowData
    {
        if backend  >= self.graphics_backends.len()
        {
            // Error, backend does not exist
             eprintln!(
                "Error: Backend ({}) doesn't exist!", 
                       backend);
            return WindowData{
                id : 0,
                width : 0,
                height : 0
            }
        }
        return self.graphics_backends[backend].create_window(width, height, name);

    }

    pub fn create_fullscreen_window(&mut self, name : &str, backend : usize) -> WindowData
    {
        if backend  >= self.graphics_backends.len()
        {
            // Error, backend does not exist
             eprintln!(
                "Error: Backend ({}) doesn't exist!", 
                       backend);
            return WindowData{
                id : 0,
                width : 0,
                height : 0
            }
        }
        return self.graphics_backends[backend].create_fullscreen_window(name);
    }

    pub fn render_to_window(&mut self, pixel_buffer: &Vec<u8>, window_id:usize, backend : usize)
    {
        self.graphics_backends[backend].render(pixel_buffer, window_id);
    }
}