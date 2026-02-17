use crossbeam::channel::{Sender, Receiver, unbounded};
use engine_math::vector2::Vector2;
use system_interface::common::keyboard_interface::{MyKeyboardEvent};

#[derive(Debug, Clone)]
pub enum ForceField{
    Constant{ // e.g. gravity on a 2D sidescroller
        force : Vector2,
        charge : Box<[f32]>        
    },
    InverseSquare{ // e.g. gravity in a space sim
        source : usize,
        constant : f32,
        charge : Box<[f32]>
    },
    InverseSquareNBody{// e.g. N body gravity sim
        constant : f32,
        charge : Box<[f32]>
    },
    LJPotential{ // Leonard-Jones potential, used for simulating phases of matter
        a : f32,
        b : f32
    }
}

#[derive(Debug, Clone)]
pub enum LogicToPhysicsChannel{
    AddEntityGroup{
        num_entities_pow_2 : u32
    },
    AddEntity{
        group_index : usize,
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
        group_index : usize,
        index : usize
    },
    ApplyForce{
        group_index : usize,
        index : usize,
        forcex : f32,
        forcey : f32
    },
    ApplyCenterlineForce{
        group_index : usize,
        index : usize,
        force : f32
    },
    ApplyTorque{
        group_index : usize,
        index : usize,
        torque : f32
    },
    ApplyField{
        group_index : usize,
        field : ForceField
    }
}

#[derive(Debug, Clone)]
pub enum LogicToRenderSyncChannel{
    BackgroundTexture,
    Camera{
        posx : f64,
        posy : f64
    },
    DotGraphicsUpdate{
        index : usize,
        layer : f64,
        dots  : Vec<f64>
    },
    ChangeEntityColour{
        index: usize,
        colour_id: u16
    }
}
#[derive(Debug, Clone)]
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
    Frame(Vec<u8>)
}

pub enum RenderCommand{
    Pixel {
        x: i32,
        y: i32,
        colour_id: u16,
    },
}

pub enum SysInToGameLogicChannel{
    KeyboardEvents{
        events : Vec<MyKeyboardEvent>
    },
    MouseEvents,
    TouchEvents
}

pub enum SysInToCompositorChannel{
    BufferRecycle{
        data : Vec<u8>,
        width : usize,
        height : usize
    }
}

pub struct Channel<T>{ 
    pub rx : Receiver<T>,
    pub tx : Sender<T>
}
pub struct EngineBus{
    pub logic_to_physics : Channel<LogicToPhysicsChannel>,
    pub logic_to_render_sync : Channel<LogicToRenderSyncChannel>,
    pub physics_to_logic : Channel<PhysicsToLogicChannel>,
    pub physics_to_render_sync : Channel<PhysicsToRenderSyncChannel>,
    pub render_sync_to_compositor : Channel<RenderCommand>,
    pub compositor_to_sysout : Channel<CompositorToSysOutCommand>,
    pub sysin_to_logic : Channel<SysInToGameLogicChannel>,
    pub sysin_to_compositor : Channel<SysInToCompositorChannel>
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
        let (sysin_to_compositor_tx, sysin_to_compositor_rx) = unbounded::<SysInToCompositorChannel>();

        EngineBus
        {
            logic_to_physics          : Channel { tx:logic_to_physics_tx,       rx:logic_to_physics_rx },
            logic_to_render_sync      : Channel { tx:logic_to_renderer_tx,      rx:logic_to_renderer_rx },
            physics_to_logic          : Channel { tx:physics_to_logic_tx,       rx:physics_to_logic_rx },
            physics_to_render_sync    : Channel { tx:physics_to_renderer_tx,    rx:physics_to_renderer_rx },
            render_sync_to_compositor : Channel { tx:renderer_to_compositor_tx, rx:renderer_to_compositor_rx },
            compositor_to_sysout      : Channel { tx:compositor_to_sysout_tx,   rx:compositor_to_sysout_rx },
            sysin_to_logic            : Channel { tx:sysin_to_logic_tx,         rx:sysin_to_logic_rx },
            sysin_to_compositor       : Channel { tx:sysin_to_compositor_tx,    rx:sysin_to_compositor_rx },
        }
    }
}