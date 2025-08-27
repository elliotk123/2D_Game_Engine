use physics_engine_2d::vector2::Vector2;
use graphics_engine_2d::init_graphics;
use graphics_engine_2d::run_graphics;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let points = [
        100.0, 100.0,
        200.0, 200.0,
        300.0, 100.0,
        400.0, 200.0,
        500.0, 100.0,
        600.0, 200.0,
    ];

    let mut canvas = init_graphics()?;

    loop {
        run_graphics(&mut canvas, &points);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
