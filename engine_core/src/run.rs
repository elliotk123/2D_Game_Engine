use std::time::Duration;

use engine_common::{
    engine_module::ModuleId,
    game_conf::GameConf
};

use crate::{game_state::GameState, scheduler::Scheduler};

pub fn run<T : GameConf>(schedule : Vec<Vec<ModuleId>>, minor_cycle : Duration)
{
    let scheduler = Scheduler::new(minor_cycle, schedule);

    let mut game_state : GameState<T> = GameState::new(minor_cycle);

    let mut run = true;

    while run == true{
        run = scheduler.run(&mut game_state);
    }
}