use super::engine_bus::EngineBus;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleId{
    PHYS2D = 1,
    COMP2D = 2,
    SYSIN  = 3,
    SYSOUT = 4,
    LOGIC  = 5,
    RENDER = 6,
}

pub trait EngineModule {
    fn run(&mut self, bus : &mut EngineBus);
}

pub trait EngineIoModule{
    fn write_output(&mut self, bus : &mut EngineBus);
    fn read_input(&mut self, bus : &mut EngineBus);
}