use system_interface::common::keyboard_interface::{MyKeyboardEvent};

#[derive(Debug, Clone)]
pub struct GameLogicInputs{
    pub keyboard_events : Vec<MyKeyboardEvent>,
    pub collision_events : u64,
}

impl GameLogicInputs{
    pub fn new() -> GameLogicInputs{
        GameLogicInputs{
            keyboard_events: Vec::new(),
            collision_events : 0
        }
    }
}

pub struct GameLogicOutputs{
    pub physics_commands : u64,
    pub render_sync_commands : u64
}

impl GameLogicOutputs{
    pub fn new() -> GameLogicOutputs{
        GameLogicOutputs { 
            physics_commands: (0), 
            render_sync_commands: (0) 
        }
    }
}