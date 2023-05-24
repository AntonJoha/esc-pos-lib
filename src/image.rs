use super::constants;
use image::imageops;
use image::GrayImage;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<bool>,
}


fn dithered_buffer(path: &str) ->
    Result<GrayImage, String> {

    let img = match image::open(path) {
        Ok(img) => img,
        Err(_) => return Err(String::from("Could not open image")),
    };
    let img = img.resize(constants::MAX_X_WIDTH, std::u32::MAX, imageops::FilterType::Nearest);

    let mut buffer = img.into_luma8();
    imageops::dither(&mut buffer, &image::imageops::colorops::BiLevel);
    
    return Ok(buffer);
}

fn buffer_to_bool_vec(buffer: &GrayImage) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    for pixel in buffer.pixels() {
        vec.push(pixel[0] == 0);
    }
    return vec;
}

pub fn image_from_file(path: &str) -> Result<Image, String> {
    
    let buffer = match dithered_buffer(path) {
        Ok(buffer) => buffer,
        Err(err) => return Err(err),
    };

    let (width, height) = buffer.dimensions();
    let pixels: Vec<bool> = buffer_to_bool_vec(&buffer);

    return Ok(Image {
        width,
        height,
        pixels,
    });
}

impl Image {

    ///Requires the width and height of the image in terms of pixels
    ///The height has no max while the width is limited to constant:MAX_X_WIDTH which is 1024
    ///The vector contains pixels from top left to bottom right in a left to right fashion
    pub fn new(width : u32, height: u32, image: Vec<bool>) -> Result<Image, String> {
        if width > constants::MAX_X_WIDTH {
            return Err(format!("Width {} exceeds maximum width of 1024",
                               width));
        }
        if image.len() != (width * height) as usize {
            return Err(format!("Image size {} does not match width {} and height {} area {}",
                               image.len(),
                               width,
                               height,
                               width * height));
        }
        Ok(
        Image {
            width: width/8,
            height,
            pixels: image,
        }
        )
    }

    ///Returns a vector of bytes which can be sent to the printer add function.
    pub fn export(&self ) -> Vec<u8> {
        let mut to_return = Vec::new();
        to_return.push(constants::GS); //GS
        to_return.push(0x76); // v
        to_return.push(0x30); // O
        to_return.push(0); // m
        to_return.push((self.width % 256) as u8);   // xL
        to_return.push((self.width / 256) as u8);   // xH
        to_return.push((self.height % 256) as u8);  // yL
        to_return.push((self.height / 256) as u8);  // yH

        let mut curr: u8 = 0;
        let mut count = 0;
        for i in 0..self.pixels.len() {
            if self.pixels[i] {
                curr |= 1 << (7 - count);
            } 
            count += 1;
            if count == 8 {
                to_return.push(curr);
                curr = 0;
                count = 0;
            }
        }
        to_return
    }
}
