use crossbeam::channel::{Sender, Receiver, unbounded};
use inter_module_comms::pixel_buffer::PixelBuffer;
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

pub enum LogicToRenderSyncChannel{
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

pub enum PhysicsToRenderSyncChannel{
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

pub enum CompositorToSysOutCommand{
    Frame(PixelBuffer)
}

pub enum RenderCommand{
    Pixel {
        x: i32,
        y: i32,
        rgba: [u8; 4],
    },
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
    pub logic_to_renderer : Channel<LogicToRenderSyncChannel>,
    pub physics_to_logic : Channel<PhysicsToLogicChannel>,
    pub physics_to_renderer : Channel<PhysicsToRenderSyncChannel>,
    pub renderer_to_compositor : Channel<RenderCommand>,
    pub compositor_to_sysout : Channel<CompositorToSysOutCommand>,
    pub sysin_to_logic : Channel<SysInToGameLogicChannel>
}

impl EngineBus{
    pub fn new() -> EngineBus
    {
        let (logic_to_physics_tx, logic_to_physics_rx) = unbounded::<LogicToPhysicsChannel>();
        let (logic_to_renderer_tx, logic_to_renderer_rx) = unbounded::<LogicToRenderSyncChannel>();
        let (physics_to_logic_tx, physics_to_logic_rx) = unbounded::<PhysicsToLogicChannel>();
        let (physics_to_renderer_tx, physics_to_renderer_rx) = unbounded::<PhysicsToRenderSyncChannel>();
        let (renderer_to_compositor_tx, renderer_to_compositor_rx) = unbounded::<RenderCommand>();
        let (compositor_to_sysout_tx, compositor_to_sysout_rx) = unbounded::<CompositorToSysOutCommand>();
        let (sysin_to_logic_tx, sysin_to_logic_rx) = unbounded::<SysInToGameLogicChannel>();

        EngineBus
        {
            logic_to_physics        : Channel { tx:logic_to_physics_tx,       rx:logic_to_physics_rx },
            logic_to_renderer       : Channel { tx:logic_to_renderer_tx,      rx:logic_to_renderer_rx },
            physics_to_logic        : Channel { tx:physics_to_logic_tx,       rx:physics_to_logic_rx },
            physics_to_renderer     : Channel { tx:physics_to_renderer_tx,    rx:physics_to_renderer_rx },
            renderer_to_compositor  : Channel { tx:renderer_to_compositor_tx, rx:renderer_to_compositor_rx },
            compositor_to_sysout    : Channel { tx:compositor_to_sysout_tx,   rx:compositor_to_sysout_rx },
            sysin_to_logic          : Channel { tx:sysin_to_logic_tx,         rx:sysin_to_logic_rx },
        }
    }
}