use physics_engine_2d::entity::Entity;
use physics_engine_2d::particle::Particle;
use physics_engine_2d::vector2::Vector2;
use physics_engine_2d::shape::Shape;
use physics_engine_2d::animate::animate;
use inter_module_comms::pixel_buffer::PixelBuffer;
use system_interface::{
    init_system_interface,
    common::keyboard_interface::{MyKey, MyKeyboardEvent }
};

use std::time::Duration;

fn main() {

    const WIDTH : usize= 1000;
    const HEIGHT : usize = 500;


    let mut pixel_buffer = PixelBuffer::new(
        1000, 500, 4, &[0, 0, 0, 255]
    );

    let mut entity_list:Vec<Entity> = Vec::new();

    let entity1 = Entity::new(
        Particle::new(
            Vector2 { x: (0.0), y: (0.0) },
            Vector2 { x: (3.0), y: (1.0) },
            0.0,
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

    'running: loop {
        for event in system_interface.keyboard_interface.poll_events(){
            match event {
                MyKeyboardEvent::KeyDown(MyKey::Escape) => break 'running,
                _ => {}
            }
        }

        for ent in entity_list.iter_mut(){
            ent.update(1.0/60.0);
        }

        pixel_buffer.clear(&[0, 0, 0, 255]);

        animate(
            &mut pixel_buffer, 
            &entity_list, 
            WIDTH, HEIGHT, 
            100, 50, 
            Vector2 { x: (-50.0), y: (-25.0) }
        );

        system_interface.graphical_interface.render_to_window(&pixel_buffer.pixel_data, window, 0);
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
