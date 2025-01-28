use physics_engine_2d::vector2::Vector2;

fn main() {
    println!("Hello, world!");
    let v1 : Vector2 = Vector2::new(2.0,3.0);
    let v2 : Vector2 = Vector2::new(1.0,8.0);
    println!("{:?} + {:?} = {:?}",v1,v2,v1 + v2);
}
