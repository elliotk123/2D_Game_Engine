use crossbeam::channel::{Sender, Receiver, unbounded};

enum LogicToPhysicsChannel{
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

enum LogicToCompositorChannel{
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
enum PhysicsToLogicChannel{
    Collision{
        index_a : usize,
        index_b : usize,
        pocx : f64,
        pocy : f64,
        angle : f64,
        depth : f64
    }
}

enum PhysicsToCompositorChannel{
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

enum CompositorToSysOutChannel{
    PixelBuffer{
        data : Vec<u8>,
    }
}

enum SysInToGameLogicChannel{
    KeyboardEvents,
    MouseEvents,
    TouchEvents
}

struct Channel<T>{ 
    rx : Receiver<T>,
    tx : Sender<T>
}
pub struct EngineBus{
    logic_to_physics : Channel<LogicToPhysicsChannel>,
    logic_to_compositor : Channel<LogicToCompositorChannel>,
    physics_to_logic : Channel<PhysicsToLogicChannel>,
    physics_to_compositor : Channel<PhysicsToCompositorChannel>,
    compositor_to_sysout : Channel<CompositorToSysOutChannel>,
    sysin_to_logic : Channel<SysInToGameLogicChannel>
}

impl EngineBus{
    fn new(&mut self)
    {
        (self.logic_to_physics.tx, self.logic_to_physics.rx) = unbounded::<LogicToPhysicsChannel>();
        (self.logic_to_compositor.tx, self.logic_to_compositor.rx) = unbounded::<LogicToCompositorChannel>();
        (self.physics_to_logic.tx, self.physics_to_logic.rx) = unbounded::<PhysicsToLogicChannel>();
        (self.physics_to_compositor.tx, self.physics_to_compositor.rx) = unbounded::<PhysicsToCompositorChannel>();
        (self.compositor_to_sysout.tx, self.compositor_to_sysout.rx) = unbounded::<CompositorToSysOutChannel>();
        (self.sysin_to_logic.tx, self.sysin_to_logic.rx) = unbounded::<SysInToGameLogicChannel>();
    }
}