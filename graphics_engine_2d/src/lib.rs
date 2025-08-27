extern crate sdl3;

use sdl3::render::FPoint;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::video::Window;
use sdl3::render::Canvas;
use std::time::Duration;

pub fn init_graphics() ->  Result<Canvas<Window>, String>{
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    Ok(canvas)
}

pub fn run_graphics(canvas : &mut Canvas<Window>, points: &[f32] ){
    let points_converted: Vec<FPoint> = points
        .chunks(2)
        .map(|chunk| FPoint::new(chunk[0], chunk[1]))
        .collect();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.draw_points(&points_converted[..]).unwrap();

    canvas.present();    
}


    // let mut event_pump = sdl_context.event_pump().unwrap();

        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit {..} |
        //         Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        //             break 'running
        //         },
        //         _ => {}
        //     }
        // }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
