use std::time::Duration;

use crate::{engine_bus::PhysicsEventToLogicChannel, engine_bus::PhysicsStateToLogicChannel, engine_module::ModuleId};

use super::engine_bus::{LogicToPhysicsChannel, LogicToRenderSyncChannel};
use system_interface::common::keyboard_interface::{MyKeyboardEvent};

#[derive(Debug, Clone)]
pub struct GameLogicInputs
{
    pub keyboard_events : Vec<MyKeyboardEvent>,
    pub physics_events  : Vec<PhysicsEventToLogicChannel>,
    pub physics_states  : Vec<PhysicsStateToLogicChannel>,
}

impl GameLogicInputs
{
    pub fn new() -> GameLogicInputs{
        GameLogicInputs{
            keyboard_events: Vec::new(),
            physics_events : Vec::new(),
            physics_states : Vec::new(),
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

pub trait GameConf{
    fn new() -> Self;
    fn process(&mut self, input: &GameLogicInputs, output : &mut GameLogicOutputs) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenSettings{
    pub full_screen : bool,
    pub screen_width : usize,
    pub screen_height : usize
}

pub struct EngineSettings{
    pub schedule : Vec<Vec<ModuleId>>,
    pub minor_cycle : Duration,
    pub screen_settings : ScreenSettings,
    pub asset_manifest_path: String,
}

impl Clone for EngineSettings{
    fn clone(&self)->EngineSettings{
        EngineSettings{
            schedule: self.schedule.clone(),
            minor_cycle : self.minor_cycle,
            screen_settings : self.screen_settings,
            asset_manifest_path: self.asset_manifest_path.clone(),
        }
    }
}

