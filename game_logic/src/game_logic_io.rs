use engine_common::engine_bus::{LogicToPhysicsChannel, LogicToRenderSyncChannel, PhysicsToLogicChannel};
use system_interface::common::keyboard_interface::{MyKeyboardEvent};

#[derive(Debug, Clone)]
pub struct GameLogicInputs
{
    pub keyboard_events : Vec<MyKeyboardEvent>,
    pub physics_events : Vec<PhysicsToLogicChannel>,
}

impl GameLogicInputs
{
    pub fn new() -> GameLogicInputs{
        GameLogicInputs{
            keyboard_events: Vec::new(),
            physics_events : Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameLogicOutputs
{
    pub physics_commands : Vec<LogicToPhysicsChannel>,
    pub render_sync_commands : Vec<LogicToRenderSyncChannel>
}

impl GameLogicOutputs
{
    pub fn new() -> GameLogicOutputs{
        GameLogicOutputs { 
            physics_commands: Vec::new(), 
            render_sync_commands: Vec::new()
        }
    }
}