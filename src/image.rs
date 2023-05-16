use super::constants;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<bool>,
}


impl Image {

    pub fn new(width : u32, height: u32, image: Vec<bool>) -> Image {
        Image {
            width,
            height,
            pixels: image,
        }
    }

    pub fn export(&self ) -> Vec<u8> {
        let mut to_return = Vec::new();
        to_return.push(constants::ESC);
        to_return.push(0x2A);
        to_return.push(33);
        to_return.push(self.width as u8);
        to_return.push(self.height as u8);
        for i in 0..self.pixels.len() {
            if self.pixels[i] {
                to_return.push(0x01);
            } else {
                to_return.push(0x00);
            }
        }
        to_return.push('\n' as u8);
        to_return
    }
}
