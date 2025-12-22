use system_interface::{init_system_interface, SystemInterface};
use engine_common::engine_bus::{
    EngineBus,
    CompositorToSysOutCommand,
    SysInToGameLogicChannel
};
use engine_common::engine_module::EngineIoModule;

const WIDTH : usize = 500;
const HEIGHT : usize = 500;

pub struct EngineIO {
    system_interface : SystemInterface
}

impl EngineIO{
    pub fn new() -> EngineIO{
        let mut system_interface: SystemInterface = init_system_interface();
        system_interface.graphical_interface.create_window(WIDTH as u32, HEIGHT as u32, "Physics Demo", 0);
        EngineIO{
            system_interface
        }
    }
}

impl EngineIoModule for EngineIO{
    fn write_output(&mut self, bus : &mut EngineBus){
        match bus.compositor_to_sysout.rx.recv().unwrap(){
            CompositorToSysOutCommand::Frame (data) => {
                self.system_interface.graphical_interface.render_to_window(&data.pixel_data, 0, 0);
            }
        }
            
    }

    fn read_input(&mut self, bus : &mut EngineBus){
        bus.sysin_to_logic.tx.send(
            SysInToGameLogicChannel::KeyboardEvents{
                events : self.system_interface.keyboard_interface.poll_events()
            }
        );
    }
}