use system_interface::{init_system_interface, SystemInterface};
use engine_common::engine_bus::{
    CompositorToSysOutCommand, EngineBus, SysInToCompositorChannel, SysInToGameLogicChannel
};
use engine_common::engine_module::EngineIoModule;

const WIDTH : usize = 1000;
const HEIGHT : usize = 500;
// const FULLSCREEN : bool = true;

pub struct EngineIO {
    system_interface : SystemInterface,
    pixel_buffer : Vec<u8>
}

impl EngineIO{
    pub fn new() -> EngineIO{
        let mut system_interface: SystemInterface = init_system_interface();
        system_interface.graphical_interface.create_window(WIDTH as u32, HEIGHT as u32, "Physics Demo", 0);
        EngineIO{
            system_interface,
            pixel_buffer: vec![0u8;WIDTH*HEIGHT*4]
        }
    }
}

impl EngineIoModule for EngineIO{
    fn write_output(&mut self, bus : &mut EngineBus)->bool{
        match bus.compositor_to_sysout.rx.recv().unwrap(){
            CompositorToSysOutCommand::Frame (data) => {
                self.system_interface.graphical_interface.render_to_window(&data, 0, 0);
                self.pixel_buffer = data;
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
            SysInToCompositorChannel::BufferRecycle(std::mem::take(&mut self.pixel_buffer))
        ).unwrap();
        return true;
    }
}