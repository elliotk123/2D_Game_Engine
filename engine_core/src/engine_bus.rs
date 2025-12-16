use crossbeam::channel::{Sender, Receiver, bounded};

const BUS_BUFFER_LENGTH : usize = 2;

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

enum LogicToCompositor{
    BackgroundTexture,
    Camera{
        posx : f64,
        posy : f64
    },
    SpriteEntityMap
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

enum CompositorToSysOut{
    PixelBuffer{
        data : Vec<u8>
    }
}

enum SysInToGameLogic{
    KeyboardEvents,
    MouseEvents,
    TouchEvents
}

struct Channel<T>{ 
    rx : Receiver<T>,
    tx : Sender<T>
}
struct EngineBus{
    logic_to_physics : Channel<LogicToPhysicsChannel>,
    physics_to_logic : Channel<PhysicsToLogicChannel>,
    physics_to_compositor : Channel<PhysicsToCompositorChannel>

}

impl EngineBus{
    fn new(&mut self)
    {
        (self.logic_to_physics.tx, self.logic_to_physics.rx) = bounded::<LogicToPhysicsChannel>(BUS_BUFFER_LENGTH);
        (self.physics_to_logic.tx, self.physics_to_logic.rx) = bounded::<PhysicsToLogicChannel>(BUS_BUFFER_LENGTH);
        (self.physics_to_compositor.tx, self.physics_to_compositor.rx) = bounded::<PhysicsToCompositorChannel>(BUS_BUFFER_LENGTH);
    }
}