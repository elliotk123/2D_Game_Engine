use sdl3;

use std::rc::Rc;
use std::cell::RefCell;

pub struct SdlState
{
    pub video_subsystem : sdl3::VideoSubsystem,
    pub event_pump : sdl3::EventPump
}

pub type SdlStateShared = Rc<RefCell<SdlState>>;