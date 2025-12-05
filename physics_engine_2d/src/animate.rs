use super::entity::Entity;
use super::vector2::Vector2;
use inter_module_comms::pixel_buffer::PixelBuffer;


pub fn animate(
    buffer: &mut PixelBuffer, 
    entity_list: &Vec<Entity>, 
    pix_x: usize, pix_y: usize, 
    phys_x: usize, phys_y: usize, 
    phys_origin: Vector2
)
{
    let conversion = Vector2::new(
        pix_x as f32/phys_x as f32,
        pix_y as f32/phys_y as f32
    );

    for e in entity_list.iter(){
        for vertex in e.shape.vertices.iter(){
            let particle_vec: Vector2 = (
                vertex.rotate(e.particle.orientation) + 
                e.particle.position - phys_origin)*conversion;
            buffer.set_pixel(
                particle_vec.x.round() as usize, 
                particle_vec.y.round() as usize, 
                &[255,0,0,255]
            );
        }
    }
}