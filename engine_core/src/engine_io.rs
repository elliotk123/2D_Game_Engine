use engine_common::game_conf::ScreenSettings;
use system_interface::common::graphical_interface::WindowData;
use system_interface::{init_system_interface, SystemInterface};
use engine_common::engine_bus::{
    CompositorToSysOutCommand, EngineBus, SysInToCompositorChannel, SysInToGameLogicChannel
};
use engine_common::engine_module::EngineIoModule;
use inter_module_comms::pixel_buffer::PixelBuffer;

pub struct EngineIO {
    system_interface : SystemInterface,
    pixel_buffer : PixelBuffer
}

impl EngineIO{
    pub fn new(screen_settings : ScreenSettings) -> EngineIO{
        let mut system_interface: SystemInterface = init_system_interface();
        let window_data: WindowData;
        if !screen_settings.full_screen{
            window_data = system_interface.graphical_interface.create_window(
                screen_settings.screen_width as u32, 
                screen_settings.screen_height as u32, 
                "Physics Demo", 
                0
            );
        }else{
            window_data = system_interface.graphical_interface.create_fullscreen_window(
                "Physics Demo", 
                0
            );
            // get screen dimensions from window
        }
        EngineIO{
            system_interface,
            pixel_buffer: PixelBuffer{
                pixel_data : vec![0u8;window_data.width*window_data.height*4],
                width : window_data.width,
                height : window_data.height
            }
        }
    }
}

impl EngineIoModule for EngineIO{
    fn write_output(&mut self, bus : &mut EngineBus)->bool{
        match bus.compositor_to_sysout.rx.recv().unwrap(){
            CompositorToSysOutCommand::Frame (data) => {
                self.system_interface.graphical_interface.render_to_window(&data, 0, 0);
                self.pixel_buffer.pixel_data = data;
            }
        }
        return true;
            
    }

    fn read_input(&mut self, bus : &mut EngineBus)->bool{
        bus.sysin_to_logic.tx.send(
            SysInToGameLogicChannel::KeyboardEvents{
                events : self.system_interface.keyboard_interface.poll_events()
            }
        ).unwrap();
        bus.sysin_to_compositor.tx.send(
            SysInToCompositorChannel::BufferRecycle{
                data : std::mem::take(&mut self.pixel_buffer.pixel_data),
                width : self.pixel_buffer.width,
                height : self.pixel_buffer.height
            }
        ).unwrap();
        return true;
    }
}