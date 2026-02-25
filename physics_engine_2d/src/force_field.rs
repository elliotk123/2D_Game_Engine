use engine_common::engine_bus::ForceField;
use engine_math::vector2::Vector2;

pub fn calculate_constant_force(force : Vector2, charge : f64)-> Vector2
{
    force*charge
}

pub fn calculate_inverse_square
(
    source_pos : Vector2, 
    constant : f64, 
    charge : f64,
    position : Vector2
)->Vector2{
    if charge == 0.0{
        return Vector2{x:0.0,y:0.0};
    }
    let disp = position - source_pos;
    let mag = disp.magnitude();
    disp.scale(constant*charge*mag.powi(-3))
}

pub fn reconfigure_deleted_entity(deleted_entity_id : usize, field : &mut ForceField){
        match field{
            ForceField::Constant 
            {
                force, charge 
            }  => {
                for((fx, fy),c) in 
                self.force_x_accumulators.iter_mut()
                .zip(&mut self.force_y_accumulators)
                .zip(&charge)
                .take(self.num_active_entities){
                    let calculated_force = calculate_constant_force(force, *c);
                    *fx += calculated_force.x;
                    *fy += calculated_force.y;
                }
            },
            ForceField::InverseSquare 
            { 
                source, 
                constant, 
                charge 
            } => {
                let source_position = Vector2
                {
                    x : self.particles.positions_x[source],
                    y : self.particles.positions_y[source],
                };
                for(((p, fx),fy),c) in 
                self.particles.into_iter()
                .zip(&mut self.force_x_accumulators)
                .zip(&mut self.force_y_accumulators)
                .zip(charge)
                .take(self.num_active_entities){
                    let force = calculate_inverse_square
                    (
                        source_position,
                        constant, 
                        c, 
                        Vector2{x: *p.x, y : *p.y} 
                    );
                    *fx += force.x;
                    *fy += force.y;
                }
            }
            _ => {
            }
        }
}

