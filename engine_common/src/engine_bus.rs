use crossbeam::channel::{Sender, Receiver, unbounded};
use system_interface::common::keyboard_interface::{MyKeyboardEvent, MyKey};

pub enum LogicToPhysicsChannel{
    AddEntity{
        mass : f64,
        moi : f64,
        posx : f64,
        posy : f64,
        velx : f64,
        vely : f64,
        orien : f64,
        angvel : f64,
        shape : Vec<f64>
    },
    RemoveEntity{
        index : usize
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
    pub fn new(&mut self)
    {
        (self.logic_to_physics.tx, self.logic_to_physics.rx) = unbounded::<LogicToPhysicsChannel>();
        (self.logic_to_compositor.tx, self.logic_to_compositor.rx) = unbounded::<LogicToCompositorChannel>();
        (self.physics_to_logic.tx, self.physics_to_logic.rx) = unbounded::<PhysicsToLogicChannel>();
        (self.physics_to_compositor.tx, self.physics_to_compositor.rx) = unbounded::<PhysicsToCompositorChannel>();
        (self.compositor_to_sysout.tx, self.compositor_to_sysout.rx) = unbounded::<CompositorToSysOutChannel>();
        (self.sysin_to_logic.tx, self.sysin_to_logic.rx) = unbounded::<SysInToGameLogicChannel>();
    }
}