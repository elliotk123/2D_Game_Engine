pub trait GraphicsBackend
{
    fn render(&mut self, pixel_buffer: &Vec<u8>, window_id: usize);
    fn create_window(&mut self, width: u32, height: u32, name : &str) -> usize;
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

    pub fn create_window(&mut self, width:u32, height:u32, name : &str, backend : usize) -> usize
    {
        if backend  >= self.graphics_backends.len()
        {
            // Error, backend does not exist
            return 0 as usize;
        }
        return self.graphics_backends[backend].create_window(width, height, name);

    }

    pub fn render_to_window(&mut self, pixel_buffer: &Vec<u8>, window_id:usize, backend : usize)
    {
        self.graphics_backends[backend].render(pixel_buffer, window_id);
    }
}