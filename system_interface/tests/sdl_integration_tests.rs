use system_interface::init_system_interface;
use system_interface::common::keyboard_interface::{MyKey, MyKeyboardEvent};

fn set_argb_pixel(
    buffer: &mut Vec<u8>, 
    x: usize, 
    y: usize, 
    width: usize, 
    // The color array is [B, G, R, A]
    color: [u8; 4] 
) {
    const BPP: usize = 4;
    let index = (y * width + x) * BPP;

    // Boundary check
    if index + BPP <= buffer.len() {
        buffer[index]     = color[0]; // BLue (B)
        buffer[index + 1] = color[1]; // Green (G)
        buffer[index + 2] = color[2]; // Red (R)
        buffer[index + 3] = color[3]; // Alpha (A)
    }
}

const OPAQUE_RED: [u8; 4] = [0, 0, 255, 255]; //
#[test]
// Creates a grey window which fills up red pixel by pixel.  
fn create_window_test(){
    const WINDOW_WIDTH : u32 = 500;
    const WINDOW_HEIGHT : u32 = 500;
    const BPP : u32 = 4;
    const BUFFER_SIZE : u32 = WINDOW_WIDTH*WINDOW_HEIGHT*BPP;

    // Creates a vector of size 20,000 filled with 0s (A=0, R=0, G=0, B=0)
    let pixel_buffer: &mut Vec<u8> = &mut vec![125; BUFFER_SIZE as usize];

    let mut interface: system_interface::SystemInterface = init_system_interface();
    let window_id: usize = interface.graphical_interface.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Test", 0);
    let mut counter = 0;
    'running: loop {
        // 1. You MUST poll events here!
        // This is necessary for SDL to recognize and refresh the window
        // (Assuming event_pump is accessible through shared_sdl)
        for event in interface.keyboard_interface.poll_events(){
            match event {
                MyKeyboardEvent::KeyDown(MyKey::Escape) => break 'running,
                _ => {}
            }
        }
        set_argb_pixel(pixel_buffer, counter % WINDOW_WIDTH as usize, counter / WINDOW_WIDTH as usize, WINDOW_WIDTH as usize, OPAQUE_RED);
        interface.graphical_interface.render_to_window(pixel_buffer, window_id, 0);
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        counter += 1;
    }
}