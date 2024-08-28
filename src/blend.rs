use image::{ImageBuffer, Rgb};
use crate::{operations::BlendOperation, SourceData};

impl SourceData {
    pub fn blend_images(&self, operation: impl BlendOperation)  -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let SourceData {
            width,
            height,
            image1,
            image2,
        } = self;

        // Create a new buffer that has the same size as input images, which will serve as our output data
        let mut buffer = ImageBuffer::new(*width as u32, *height as u32);

        // Iterate over all pixels in the output buffer, along with their coordinates
        for (x, y, output_pixel) in buffer.enumerate_pixels_mut() {
            // Compute linear index form x & y coordinates. In other words, you have the
            // row and column indexes here, and you want to compute the array index based
            // on these two positions.
            let index = (y * *width as u32 + x) as usize;

            // Store pixel values in the given position into variables
            let pixel1 = image1[index];
            let pixel2 = image2[index];

            // Compute the blended pixel and convert it into the `Rgb` type, which is then
            // assigned to the output pixel in the buffer.
            *output_pixel = Rgb::from(operation.perform_operation(pixel1, pixel2));
        }

        buffer
    }
}
