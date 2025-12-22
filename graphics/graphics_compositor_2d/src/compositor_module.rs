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
    frameBuffer : PixelBuffer,
    palette : Vec<[u8;4]>
}

pub const BPP : usize = 4;
pub const CLEAR_COLOUR : [u8;4] = [0, 0, 0, 255];

impl CompositorModule{
    pub fn new(width: usize, height: usize, palette: Vec<[u8;4]>) -> CompositorModule
    {
        CompositorModule
        {
            id : 0,
            frameBuffer: PixelBuffer::new(width, height,CLEAR_COLOUR),
            palette,
        }
    }
}

impl EngineModule for CompositorModule{
    fn run(&mut self, bus : &mut EngineBus)
    {
        // First Drain all Queued RenderCommands
        while let Ok(cmd) = bus.render_sync_to_compositor.rx.try_recv()
        {
            match cmd {
                RenderCommand::Pixel {x,y,colour_id}=> 
                {
                    //exclude off screen pixels
                    if x< 0 || y < 0{
                        continue;
                    }

                    //normalise types
                    let x = x as usize;
                    let y = y as usize;

                    let width = self.frameBuffer.width as usize;
                    let height = self.frameBuffer.height as usize;
                    
                    //exclude off screen pixels
                    if x >= width || y >= height{
                        continue;
                    }
                    
                    //look up colour in palette and exclude if colour does not exist
                    let colour_idx = colour_id as usize;
                    if colour_idx < self.palette.len() {
                        continue;
                    }

                    //insert pixel into frame buffer
                    let idx = (y * width + x) * BPP;
                    self.frameBuffer.pixel_data[idx..idx+BPP].copy_from_slice(&self.palette[colour_idx]);

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

