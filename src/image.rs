use super::constants;
use image::imageops;
use image::GrayImage;
use image::DynamicImage;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<bool>,
}

fn resize_image(img: &DynamicImage) -> DynamicImage {
    img.resize(constants::MAX_X_WIDTH, std::u32::MAX, imageops::FilterType::Nearest)
}

fn get_image(path: &str) ->
    Result<DynamicImage, String> {
    return match image::open(path) {
        Ok(img) => Ok(img),
        Err(_) => Err(String::from("Could not open image")),
    }
}

fn buffer_to_bool_vec(buffer: &GrayImage) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    for pixel in buffer.pixels() {
        vec.push(pixel[0] == 0);
    }
    return vec;
}

/// Create a new Image object from a DynamicImage in the image crate
pub fn image_from_dynamic(buffer: &image::DynamicImage) -> Result<Image, String> {

    let mut buffer = resize_image(buffer).into_luma8();
    imageops::dither(&mut buffer, &image::imageops::colorops::BiLevel);

    let vec = buffer_to_bool_vec(&buffer);

    return match Image::new(buffer.width(), buffer.height(), vec) {
        Ok(img) => Ok(img),
        Err(_) => Err("Could not create image from buffer".to_string()),
    }
}

///Give a path to an image and get an Image object back
///It's based on the crate image in rust. 
///Filetype is checked by parsing the file extension
///If they mismatch then an error will be thrown so be aware
pub fn image_from_path(path: &str) -> Result<Image, String> {
    
    let img = match get_image(path) {
        Ok(img) => img,
        Err(err) => return Err(err),
    };

    match image_from_dynamic(&img) {
        Ok(e) => return Ok(e),
        Err(err) => return Err(err),
    }
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
