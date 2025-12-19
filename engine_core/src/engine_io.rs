use system_interface::{init_system_interface, SystemInterface};
use engine_common::engine_bus::EngineBus;

struct EngineIO {
    system_interface : SystemInterface
}

impl EngineIO{
    pub fn new(){
        EngineIO{
            system_interface : init_system_interface()
        }
    }

    pub fn write_output(&mut bus : EngineBus){
        match bus.compositor_to_sysout.rx.recv().unwrap(){

        }
            
    }

    pub fn read_input(&mut bus : EngineBus){
        let keyboard_in = bus.sysin_to_logic.send(system_interface.poll_events());
    }
}