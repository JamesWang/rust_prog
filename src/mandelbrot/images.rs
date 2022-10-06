use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use std::fs::File;

pub(crate) fn write_image(
    filename: &str,
    pixel: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(pixel, bounds.0 as u32, bounds.1 as u32, ColorType::L8)
        .expect("TODO: panic message");
    Ok(())
}
