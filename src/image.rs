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
        to_return.push(constants::GS); //GS
        to_return.push(0x76); // v
        to_return.push(0x30); // O
        to_return.push(0); // m
        to_return.push((self.width % 256) as u8);   // xL
        to_return.push((self.width / 256) as u8);   // xH
        to_return.push((self.height % 256) as u8);  // yL
        to_return.push((self.height / 256) as u8);  // yH

        for i in 0..self.pixels.len() {
            if self.pixels[i] {
                to_return.push(0xff);
            } else {
                to_return.push(0x00);
            }
        }
        to_return
    }
}
