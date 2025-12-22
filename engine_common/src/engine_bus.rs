use crossbeam::channel::{Sender, Receiver, unbounded};
use system_interface::common::keyboard_interface::{MyKeyboardEvent};

pub enum LogicToPhysicsChannel{
    AddEntity{
        mass : f32,
        moi : f32,
        posx : f32,
        posy : f32,
        velx : f32,
        vely : f32,
        orien : f32,
        angvel : f32,
        shape : Vec<f32>
    },
    RemoveEntity{
        index : usize
    },
    ApplyForce{
        index : usize,
        forcex : f32,
        forcey : f32
    },
    ApplyTorque{
        index : usize,
        torque : f32
    }
}

pub enum LogicToCompositorChannel{
    BackgroundTexture,
    Camera{
        posx : f64,
        posy : f64
    },
    DotGraphicsUpdate{
        index : usize,
        dots : Vec<f64>
    }
}
pub enum PhysicsToLogicChannel{
    Collision{
        index_a : usize,
        index_b : usize,
        pocx : f64,
        pocy : f64,
        angle : f64,
        depth : f64
    }
}

pub enum PhysicsToCompositorChannel{
    Position{
        index : usize,
        x : f64,
        y : f64
    },
    Orientation{
        index : usize,
        orientation : f64
    }
}

pub enum CompositorToSysOutChannel{
    PixelBuffer{
        data : Vec<u8>,
    }
}

pub enum SysInToGameLogicChannel{
    KeyboardEvents{
        events : Vec<MyKeyboardEvent>
    },
    MouseEvents,
    TouchEvents
}

pub struct Channel<T>{ 
    pub rx : Receiver<T>,
    pub tx : Sender<T>
}
pub struct EngineBus{
    pub logic_to_physics : Channel<LogicToPhysicsChannel>,
    pub logic_to_compositor : Channel<LogicToCompositorChannel>,
    pub physics_to_logic : Channel<PhysicsToLogicChannel>,
    pub physics_to_compositor : Channel<PhysicsToCompositorChannel>,
    pub compositor_to_sysout : Channel<CompositorToSysOutChannel>,
    pub sysin_to_logic : Channel<SysInToGameLogicChannel>
}

impl EngineBus{
    pub fn new() -> Self
    {
        let (lp_tx, lp_rx) = unbounded::<LogicToPhysicsChannel>();
        let (lc_tx, lc_rx) = unbounded::<LogicToCompositorChannel>();
        let (pl_tx, pl_rx) = unbounded::<PhysicsToLogicChannel>();
        let (pc_tx, pc_rx) = unbounded::<PhysicsToCompositorChannel>();
        let (cs_tx, cs_rx) = unbounded::<CompositorToSysOutChannel>();
        let (sl_tx, sl_rx) = unbounded::<SysInToGameLogicChannel>();
        EngineBus {
            logic_to_physics: Channel { tx: lp_tx, rx: lp_rx },
            logic_to_compositor: Channel { tx: lc_tx, rx: lc_rx },
            physics_to_logic: Channel { tx: pl_tx, rx: pl_rx },
            physics_to_compositor: Channel { tx: pc_tx, rx: pc_rx },
            compositor_to_sysout: Channel { tx: cs_tx, rx: cs_rx },
            sysin_to_logic: Channel { tx: sl_tx, rx: sl_rx },
        }
    }
}