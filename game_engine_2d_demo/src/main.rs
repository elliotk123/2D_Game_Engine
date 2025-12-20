use physics_engine_2d::entity::Entity;
use physics_engine_2d::particle::Particle;
use physics_engine_2d::vector2::Vector2;
use physics_engine_2d::shape::Shape;
use physics_engine_2d::animate::animate;
use inter_module_comms::pixel_buffer::PixelBuffer;
use engine_core::scheduler::Scheduler;
use engine_common::engine_module::ModuleId;

use system_interface::{
    init_system_interface,
    common::keyboard_interface::{MyKey, MyKeyboardEvent }
};

use std::time::Duration;

fn main() {
    let schedule: Vec<Vec<ModuleId>> = vec![
        vec![ModuleId::SYSIN,ModuleId::LOGIC, ModuleId::PHYS2D, ModuleId::COMP2D, ModuleId::SYSOUT]
    ];

    let minor_cycle : u64 = 16667;
    let scheduler = Scheduler::new(minor_cycle, schedule);
    
    const WIDTH : usize= 1000;
    const HEIGHT : usize = 500;

    let mut entity_list:Vec<Entity> = Vec::new();

    let entity1 = Entity::new(
        Particle::new(
            Vector2 { x: (0.0), y: (0.0) },
            Vector2 { x: (3.0), y: (1.0) },
            0.5,
            0.0
        ),
        Shape::new([
            Vector2 { x: (0.0), y: (1.0) },
            Vector2 { x: (-0.5), y: (-0.5) },
            Vector2 { x: (0.5), y: (-0.5) },
        ].to_vec()),
        1000.0,
        1000.0
    );

    entity_list.push(entity1);

    let mut system_interface = init_system_interface();
    let window = system_interface.graphical_interface.create_window(WIDTH as u32, HEIGHT as u32, "Physics Demo", 0);

    let mut thrust = 0.0;
    let mut torque_clock = 0.0;
    let mut torque_anti_clock = 0.0;
    'running: loop {
        for event in system_interface.keyboard_interface.poll_events(){
            if let MyKeyboardEvent::KeyDown(key) = event{
                match key {
                    MyKey::Escape => break 'running,
                    MyKey::Space => thrust = 10.0,
                    MyKey::A => torque_anti_clock = 10.0,
                    MyKey::D => torque_clock = 10.0,
                    _ => {}
                }
            }else if let MyKeyboardEvent::KeyUp(key) = event{
                match key {
                    MyKey::Space => thrust = 0.0,
                    MyKey::A => torque_anti_clock = 0.0,
                    MyKey::D => torque_clock = 0.0,
                    _ => {}
                }
            }
        }

        entity_list[0].apply_centerline_force(thrust);
        entity_list[0].apply_torque(-1.0*torque_clock);
        entity_list[0].apply_torque(torque_anti_clock);
        

        for ent in entity_list.iter_mut(){
            ent.update(1.0/60.0);
        }

        pixel_buffer.clear(&[0, 0, 0, 255]);

        animate(
            &mut pixel_buffer, 
            &entity_list, 
            WIDTH, HEIGHT, 
            100, 50, 
            Vector2 { x: (-50.0), y: (-25.0) },
            true
        );

        system_interface.graphical_interface.render_to_window(&pixel_buffer.pixel_data, window, 0);
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
