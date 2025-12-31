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
    frame_buffer : PixelBuffer,
    palette : Vec<[u8;4]>
}

pub const BPP : usize = 4;
pub const CLEAR_COLOUR : [u8;4] = [0, 0, 0, 255];

impl CompositorModule{
    pub fn new(width: usize, height: usize) -> CompositorModule
    {
        CompositorModule
        {
            id : 0,
            frame_buffer: PixelBuffer::new(width, height,CLEAR_COLOUR),
            palette : vec! [
                [  0,   0,   0, 255],
                [255, 255, 255, 255], //  1 white
                [128, 128, 128, 255], //  2 grey
                [192, 192, 192, 255], //  3 light grey

                [255,   0,   0, 255], //  4 red
                [200,   0,   0, 255], //  5 dark red
                [255, 128, 128, 255], //  6 light red

                [  0, 255,   0, 255], //  7 green
                [  0, 180,   0, 255], //  8 dark green
                [128, 255, 128, 255], //  9 light green

                [  0,   0, 255, 255], // 10 blue
                [  0,   0, 180, 255], // 11 dark blue
                [128, 128, 255, 255], // 12 light blue

                [255, 255,   0, 255], // 13 yellow
                [200, 200,   0, 255], // 14 dark yellow

                [255, 128,   0, 255], // 15 orange
                [200, 100,   0, 255], // 16 dark orange

                [128,   0, 255, 255], // 17 purple
                [100,   0, 200, 255], // 18 dark purple

                [255,   0, 255, 255], // 19 magenta
                [200,   0, 200, 255], // 20 dark magenta

                [  0, 255, 255, 255], // 21 cyan
                [  0, 180, 180, 255], // 22 dark cyan

                [150,  75,   0, 255], // 23 brown
                [210, 180, 140, 255], // 24 tan

                [255, 215,   0, 255], // 25 gold
                [192, 192, 192, 255], // 26 silver

                [255, 105, 180, 255], // 27 pink
                [ 75,   0, 130, 255], // 28 indigo
                [  0, 128, 128, 255], // 29 teal
                [128, 128,   0, 255], // 30 olive
                [245, 245, 220, 255], // 31 beige
                ]     
        }
    }
}

impl EngineModule for CompositorModule{
    fn run(&mut self, bus : &mut EngineBus)->bool
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

                    let width = self.frame_buffer.width as usize;
                    let height = self.frame_buffer.height as usize;
                    
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
                    self.frame_buffer.pixel_data[idx..idx+BPP].copy_from_slice(&self.palette[colour_idx]);

                }
            }
        }

        //Extract necessary data before mutably borrowing reference.
        let width  = self.frame_buffer.width as usize;
        let height = self.frame_buffer.height as usize;

        //Next Hand off completed Frame Data.
        let frame = std::mem::replace(&mut self.frame_buffer, PixelBuffer::new(width,height,CLEAR_COLOUR),);

        //Finally Publish Frame.
        bus.compositor_to_sysout.tx.send(CompositorToSysOutCommand::Frame(frame)).unwrap();
        return true;

    }
}

