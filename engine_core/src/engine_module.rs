#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleId{
    PHYS2D = 1,
    COMP2D = 2,
    SYSIN  = 3,
    SYSOUT = 4,
    LOGIC = 5
}

pub trait EngineModule {
    fn run(&mut self);
}