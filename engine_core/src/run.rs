

use engine_common::{
    game_conf::{EngineSettings, GameConf}
};

use crate::{game_state::GameState, scheduler::Scheduler};

pub fn run<T : GameConf>(settings : EngineSettings)
{
    let scheduler = Scheduler::new(settings.schedule.clone());

    let mut game_state : GameState<T> = GameState::new(settings.clone());

    let mut run = true;

    while run == true{
        run = scheduler.run(&mut game_state, settings.minor_cycle);
    }
}