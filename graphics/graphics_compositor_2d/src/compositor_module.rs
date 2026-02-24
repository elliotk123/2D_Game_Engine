use inter_module_comms::pixel_buffer::PixelBuffer;
// use std::time::{
//     //Duration, 
//     Instant
// };

use asset_manager::AssetManager;

use engine_common::
{
    engine_bus::
    {
        CompositorToSysOutCommand, EngineBus, RenderCommand, SysInToCompositorChannel
    }, engine_module::EngineModule, game_conf::ScreenSettings
};

pub struct CompositorModule{
    frame_buffer : PixelBuffer,
    clear_buffer : PixelBuffer,
    palette: [[u8; 4]; 8],   // debug only
    assets: AssetManager,
    initialised : bool
}

pub const BPP : usize = 4;
pub const CLEAR_COLOUR : [u8;4] = [0, 0, 0, 255];
pub const DEBUG_PALETTE: [[u8; 4]; 8] = [
    [255,   0,   0, 255], // 0: Red
    [  0, 255,   0, 255], // 1: Green
    [  0,   0, 255, 255], // 2: Blue
    [255, 255,   0, 255], // 3: Yellow
    [255,   0, 255, 255], // 4: Magenta
    [  0, 255, 255, 255], // 5: Cyan
    [255, 255, 255, 255], // 6: White
    [  0,   0,   0, 255], // 7: Black
];

impl CompositorModule{
    pub fn new(_screen_settings: ScreenSettings, assets: AssetManager) -> CompositorModule
    {
        CompositorModule
        {
            frame_buffer: PixelBuffer
            {
                pixel_data : Vec::new(),
                width : 0,
                height : 0
            },
            clear_buffer : PixelBuffer 
            { 
                pixel_data: Vec::new(), 
                width : 0, 
                height : 0 
            },
            assets: assets,
            palette: DEBUG_PALETTE,
            initialised : false
        }
    }
}

impl EngineModule for CompositorModule{
    fn run(&mut self, bus : &mut EngineBus)->bool
    {
        // let compositor_start : Instant = Instant::now();
        let mut message_num = 0;
        // Get the buffer from the system input
        while let Ok(msg) = bus.sysin_to_compositor.rx.try_recv()
        {
            match msg {
                SysInToCompositorChannel::BufferRecycle{
                    data,
                    width,
                    height
                }=> 
                {
                    self.frame_buffer.pixel_data = data;
                    self.frame_buffer.width = width;
                    self.frame_buffer.height = height;
                    if !self.initialised{
                        self.clear_buffer = PixelBuffer::new(width, height, CLEAR_COLOUR);
                        self.initialised = true;
                    }
                    self.frame_buffer.pixel_data.copy_from_slice(&self.clear_buffer.pixel_data);
                }
            }
        }
        // Then Drain all Queued RenderCommands
        while let Ok(cmd) = bus.render_sync_to_compositor.rx.try_recv()
        {
            match cmd {
                RenderCommand::Pixel {x,y,colour_id}=> 
                {
                    if self.initialised{
                        //exclude off screen pixels
                        if x< 0 || y < 0{
                            // println!("Offscreen, less than 0,0");
                            continue;
                        }

                        //normalise types
                        let x = x as usize;
                        let y = y as usize;

                        let width = self.frame_buffer.width as usize;
                        let height = self.frame_buffer.height as usize;

                        //exclude off screen pixels
                        if x >= width || y >= height{
                            // println!("Offscreen, bounds are 0,0 .. {},{}",width,height);
                            continue;
                        }
        
                        //look up colour in palette and exclude if colour does not exist
                        let colour_idx = colour_id as usize;
                        if colour_idx >= self.palette.len() {
                            println!("Colour out of bounds {}  {}",colour_idx, self.palette.len());
                            continue;
                        }

                        //insert pixel into frame buffer
                        let idx = (y * width + x) * BPP;
                        self.frame_buffer.pixel_data[idx..idx+BPP].copy_from_slice(&self.palette[colour_idx]);
                        // let duration = compositor_start.elapsed();
                        message_num = message_num + 1;
                        // println!("Update pixel buffer {}, {}, {}, {}", self.frame_buffer.pixel_data[idx], self.frame_buffer.pixel_data[idx+1], self.frame_buffer.pixel_data[idx+2], self.frame_buffer.pixel_data[idx+3], );
                        // println!("Message {} finished at {} us", message_num, duration.as_micros());
                    }
                }

                RenderCommand::Sprite { x, y, sprite_key } =>
                {
                    println!("Compositor got Sprite '{}' at {},{}", sprite_key, x, y);
                    if !self.initialised { continue; }

                    let sprite = match self.assets.get_sprite(&sprite_key) {
                        Ok(s) => s,
                        Err(e) => {
                            println!("Sprite load failed '{}': {:?}", sprite_key, e);
                            continue;
                        }
                    };

                    let dst_w = self.frame_buffer.width as usize;
                    let dst_h = self.frame_buffer.height as usize;

                    blit_rgba(
                        &mut self.frame_buffer.pixel_data,
                        dst_w,
                        dst_h,
                        x,
                        y,
                        sprite.width as usize,
                        sprite.height as usize,
                        &sprite.rgba,
                    );
                }
            }
        }
        // After clear_buffer copy, before publishing frame:
        let w = self.frame_buffer.width as usize;
        let idx = (10 * w + 10) * BPP;
        self.frame_buffer.pixel_data[idx..idx+4].copy_from_slice(&[255, 0, 0, 255]); // bright red

        //Extract necessary data before mutably borrowing reference.
        // println!("{} us", compositor_start.elapsed().as_micros());
        //Next Hand off completed Frame Data.
        // println!("{} us", compositor_start.elapsed().as_micros());
        // let frame = std::mem::replace(&mut self.frame_buffer, clear_buffer);
        let frame_pixel_data = std::mem::take(&mut self.frame_buffer.pixel_data);

        // println!("{} us", compositor_start.elapsed().as_micros());
        //Finally Publish Frame.
        bus.compositor_to_sysout.tx.send(CompositorToSysOutCommand::Frame(frame_pixel_data)).unwrap();

        // println!("{} us", compositor_start.elapsed().as_micros());
        return true;

    }
}

fn blit_rgba(
    dst: &mut [u8],
    dst_w: usize,
    dst_h: usize,
    x: i32,
    y: i32,
    src_w: usize,
    src_h: usize,
    src: &[u8],
) {
    let start_x = x.max(0) as usize;
    let start_y = y.max(0) as usize;

    let end_x = (x + src_w as i32).min(dst_w as i32).max(0) as usize;
    let end_y = (y + src_h as i32).min(dst_h as i32).max(0) as usize;

    if start_x >= end_x || start_y >= end_y {
        return;
    }

    let dst_stride = dst_w * BPP;
    let src_stride = src_w * BPP;

    for dy in start_y..end_y {
        let sy = (dy as i32 - y) as usize;

        let dst_row = dy * dst_stride;
        let src_row = sy * src_stride;

        for dx in start_x..end_x {
            let sx = (dx as i32 - x) as usize;

            let dst_idx = dst_row + dx * BPP;
            let src_idx = src_row + sx * BPP;

            // alpha test (skip fully transparent)
            if src[src_idx + 3] == 0 {
                continue;
            }

            dst[dst_idx..dst_idx + BPP].copy_from_slice(&src[src_idx..src_idx + BPP]);
        }
    }
}