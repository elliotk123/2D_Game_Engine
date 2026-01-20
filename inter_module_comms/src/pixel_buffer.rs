pub struct PixelBuffer {
    pub pixel_data: Vec<u8>,
    pub width : usize,
    pub height : usize,
}

const BPP: usize = 4;

// origin of pixel buffer is bottom Left
impl PixelBuffer {
    pub fn new(width: usize, height: usize, colour: [u8;4]) -> PixelBuffer{
        let pixel_data = colour.iter().cycle().take(width*height*BPP).copied().collect();

        Self{
            pixel_data,
            width,
            height
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: [u8;4])
    {
        if y >= self.height || x >= self.width
        {
            return;
        }
        let index = (y*self.width + x)*BPP;
        for i in 0 .. BPP
        {
            self.pixel_data[index+i] = colour[i];
        }
    }

    pub fn clear(&mut self, colour: [u8;4]) {
        // Iterate over the pixel_data in chunks equal to the BPP (e.g., 3 bytes for RGB)
        self.pixel_data
            .chunks_mut(BPP) // Gives us &mut [u8] slices of size BPP
            .for_each(|pixel_chunk| {
                // Copy the colour pattern into the current pixel chunk
                pixel_chunk.copy_from_slice(&colour);
            });
    }
}