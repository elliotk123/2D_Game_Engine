pub struct PixelBuffer {
    pub pixel_data: Vec<u8>,
    pub width : usize,
    pub height : usize,
    bpp : usize
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize, bpp : usize, colour: &[u8]) -> PixelBuffer{
        let pixel_data = colour.iter().cycle().take(width*height*bpp).copied().collect();

        Self{
            pixel_data,
            width,
            height,
            bpp
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: &[u8]){
        if y >= self.height || x >= self.width{
            return;
        }
        let index = ((self.height-y)*self.width + x)*self.bpp;
        for i in 0 .. self.bpp - 1{
            self.pixel_data[index+i] = colour[i];
        }
    }

    pub fn clear(&mut self, colour: &[u8]) {
        let bpp = self.bpp;

        // Safety Check (important!)
        if colour.len() != bpp {
            panic!("Color pattern length must match BPP ({})", bpp);
        }

        // Iterate over the pixel_data in chunks equal to the BPP (e.g., 3 bytes for RGB)
        self.pixel_data
            .chunks_mut(bpp) // Gives us &mut [u8] slices of size BPP
            .for_each(|pixel_chunk| {
                // Copy the colour pattern into the current pixel chunk
                pixel_chunk.copy_from_slice(colour);
            });
    }
}