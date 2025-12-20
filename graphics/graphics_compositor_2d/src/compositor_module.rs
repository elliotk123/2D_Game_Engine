use inter_module_comms::pixel_buffer::PixelBuffer;
use engine_common::
{
    engine_module::EngineModule,
    engine_bus::
    {
        EngineBus,
        CompositorToSysOutCommand,
        RenderCommand
    },
};

pub struct CompositorModule{
    id : u64,
    frameBuffer : PixelBuffer
}

pub const BPP : usize = 4;
pub const CLEAR_COLOUR : [u8;4] = [0, 0, 0, 255];

impl CompositorModule{
    pub fn new(width: usize, height: usize) -> CompositorModule
    {
        CompositorModule
        {
            id : 0,
            frameBuffer: PixelBuffer::new(width, height,CLEAR_COLOUR),
        }
    }
}

impl EngineModule for CompositorModule{
    fn run(&mut self, bus : &mut EngineBus)
    {
        // First Drain all Queued RenderCommands
        while let Ok(cmd) = bus.renderer_to_compositor.rx.try_recv()
        {
            match cmd {
                RenderCommand::Pixel {x,y,rgba}=> 
                {
                    if x>= 0 && y >= 0
                    {
                        let width = self.frameBuffer.width as usize;
                        let x = x as usize;
                        let y = y as usize;
                        
                        if x < self.frameBuffer.width && y < self.frameBuffer.height
                        {
                            let idx = ((y * width + x) * BPP) as usize;
                            self.frameBuffer.pixel_data[idx..idx+4].copy_from_slice(&rgba);
                        }
                    }
                }
            }
        }

        //Extract necessary data before mutably borrowing reference.
        let width  = self.frameBuffer.width as usize;
        let height = self.frameBuffer.height as usize;

        //Next Hand off completed Frame Data.
        let frame = std::mem::replace(&mut self.frameBuffer, PixelBuffer::new(width,height,CLEAR_COLOUR),);

        //Finally Publish Frame.
        bus.compositor_to_sysout.tx.send(CompositorToSysOutCommand::Frame(frame)).unwrap();

    }
}

