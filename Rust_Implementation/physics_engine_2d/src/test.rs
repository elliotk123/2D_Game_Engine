use entity::Entity;
use particle::Particle;
use vector2::Vector2;
use shape::Shape;

mod particle;
mod vector2;
mod shape;
pub mod entity;


fn test() {
    let v1 : Vector2 = Vector2::new(2.0,3.0);
    let v2 : Vector2 = Vector2::new(1.0,8.0);

    let mut p1 : Particle = Particle::new(Vector2::new(1.0, 1.0), 
                                  Vector2::new(0.03, 0.04), 
                                  0.0, 0.0);
                                  let delta_t = 1.0;
    let mut points = Vec::new();
    points.push(Vector2::new(1.0, -1.0));
    points.push(Vector2::new(0.0, 1.0));
    points.push(Vector2::new(-1.0, -1.0));

    let shape : Shape = Shape::new(points);
    let mut e1 : Entity = Entity::new(p1,shape,1.0,1.0);
    println!("{:?} + {:?} = {:?}",v1,v2,v1 + v2);
    println!("{:?} - {:?} = {:?}",v1,v2,v1 - v2);
    println!("{:?} * {:?} = {:?}",v1,v2,v1 * v2);
    println!("{:?} / {:?} = {:?}",v1,v2,v1 / v2);
    println!("{:?} * 2.0 = {:?}",v1,v1 * 2.0);
    println!("{:?} . {:?} = {:?}",v1,v2,v1.dot_product(v2));
    println!("{:?} perpendicular is {:?}",v1,v1.perpendicular());
    println!("{:?} Normalised is {:?}",v1,v1.normalise());
    println!("{:?} rotated 0.3 radians is {:?}",v1,v1.rotate(0.3));

    println!("{:?} moves to ",e1);
    e1.apply_force(Vector2::new(1.0,0.43));
    e1.apply_torque(0.35);
    e1.update(delta_t);
    println!("{:?} after {:?} seconds",e1,delta_t);
}


