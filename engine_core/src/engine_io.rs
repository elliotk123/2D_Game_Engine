use system_interface::{init_system_interface, SystemInterface};
use engine_common::engine_bus::{
    EngineBus,
    CompositorToSysOutChannel
};
use engine_common::engine_module::EngineIoModule;

const WIDTH : usize = 500;
const HEIGHT : usize = 500;

struct EngineIO {
    system_interface : SystemInterface
}

impl EngineIO{
    pub fn new(&mut self) -> &mut EngineIO{
        self.system_interface = init_system_interface();
        self.system_interface.graphical_interface.create_window(WIDTH as u32, HEIGHT as u32, "Physics Demo", 0);
        self
    }
}

impl EngineIoModule for EngineIO{


    fn write_output(&mut self, bus : &mut EngineBus){
        match bus.compositor_to_sysout.rx.recv().unwrap(){
            CompositorToSysOutChannel::PixelBuffer {data} => {
                self.system_interface.graphical_interface.render_to_window(&data, 0, 0);
            }
        }
            
    }

    fn read_input(&mut self, bus : &mut EngineBus){
        let keyboard_in = bus.sysin_to_logic.tx.send(
            SysInToGameLogicChannel::KeyboardEvents{
                events : self.system_interface.keyboard_interface.poll_events()
            }
        );
    }
}