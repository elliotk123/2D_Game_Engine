use engine_math::vector2::Vector2;

pub fn calculate_constant_force(force : Vector2, charge : f32)-> Vector2
{
    force*charge
}

pub fn calculate_inverse_square
(
    source_pos : Vector2, 
    constant : f32, 
    charge : f32,
    position : Vector2
)->Vector2{
    if charge == 0.0{
        return Vector2{x:0.0,y:0.0};
    }
    let disp = position - source_pos;
    let mag = disp.magnitude();
    disp.scale(constant*charge*mag.powi(-3))
}

