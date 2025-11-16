use std::thread;
use std::time::Duration;
use system_interface::init_system_interface;

fn set_argb_pixel(
    buffer: &mut Vec<u8>, 
    x: usize, 
    y: usize, 
    width: usize, 
    // The color array is [A, R, G, B]
    color: [u8; 4] 
) {
    const BPP: usize = 4;
    let index = (y * width + x) * BPP;

    // Boundary check
    if index + BPP <= buffer.len() {
        buffer[index]     = color[0]; // Alpha (A)
        buffer[index + 1] = color[1]; // Red (R)
        buffer[index + 2] = color[2]; // Green (G)
        buffer[index + 3] = color[3]; // Blue (B)
    }
}

const OPAQUE_RED: [u8; 4] = [255, 255, 0, 0]; //

#[test]
fn create_window(){
    const WINDOW_WIDTH : u32 = 800;
    const WINDOW_HEIGHT : u32 = 800;
    const BPP : u32 = 4;
    const BUFFER_SIZE : u32 = WINDOW_WIDTH*WINDOW_HEIGHT*BPP;

    // Creates a vector of size 20,000 filled with 0s (A=0, R=0, G=0, B=0)
    let pixel_buffer: &mut Vec<u8> = &mut vec![0; BUFFER_SIZE as usize];

    let mut interface: system_interface::SystemInterface = init_system_interface();
    let window_id: usize = interface.graphical_interface.create_window(800, 800, "Test", 0);
    interface.graphical_interface.render_to_window(&pixel_buffer, window_id, 0);
    thread::sleep(Duration::from_secs(5));
    set_argb_pixel(pixel_buffer, 400, 400, 800, OPAQUE_RED);
    set_argb_pixel(pixel_buffer, 400, 401, 800, OPAQUE_RED);
    set_argb_pixel(pixel_buffer, 400, 402, 800, OPAQUE_RED);
    interface.graphical_interface.render_to_window(&pixel_buffer, window_id, 0);
    thread::sleep(Duration::from_secs(5));
}