use super::game_logic_io::{GameLogicInputs, GameLogicOutputs};

pub trait GameConf{
    fn new() -> Self;
    fn process(&mut self, input: &GameLogicInputs, output : &mut GameLogicOutputs) -> bool;
}

