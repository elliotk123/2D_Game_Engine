use system_interface::{init_system_interface, SystemInterface};
use engine_common::engine_bus::EngineBus;
use engine_common::engine_module::EngineIoModule;

struct EngineIO {
    system_interface : SystemInterface
}

impl EngineIO{
    pub fn new() -> EngineIO{
        EngineIO{
            system_interface : init_system_interface()
        }
    }
}

impl EngineIoModule for EngineIO{


    fn write_output(&mut self, bus : &mut EngineBus){
        match bus.compositor_to_sysout.rx.recv().unwrap(){

        }
            
    }

    fn read_input(bus : &mut EngineBus){
        let keyboard_in = bus.sysin_to_logic.send(system_interface.poll_events());
    }
}