// This struct is responsible for rendering a pixel buffer to the screen.
trait GraphicsBackend
{
    fn render(&self, pixel_buffer: &Vec<u8>, window_id: u32);
    fn create_window(&self, width: u32, height: u32, name : String) -> u32;

}

pub struct GraphicalInterface
{
    priv graphics_backends : Vec<GraphicsBackend>
}

impl GraphicalInterface
{
    pub fn new(graphics_backends) -> GraphicalInterface
    {
        GraphicalInterface
        {
            graphics_backends
        }
    }

    pub fn create_window(&mut self, width:u32, height:u32, name : String, backend : u32) -> u32
    {
        if backend as usize >= graphics_backends.len()
        {
            // Error, backend does not exist
            return 0;
        }
        graphics_backends[backend].create_window(width, height, name);

    }

    pub fn render_to_window(&self, pixel_buffer: &Vec<u8>, window_id:u32, backend : u32)
    {
        graphics_backends[backend].render(pixel_buffer, window_id);
    }
}