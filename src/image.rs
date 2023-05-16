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
        to_return.push(0x28); // (
        to_return.push(0x4C); // L
        to_return.push(self.width as u8); // pL
        to_return.push(self.height as u8); // pH
        to_return.push(0x30); // m
        to_return.push(0x70); // fn
        to_return.push(48); // a
        to_return.push(1); // bx
        to_return.push(1); // by
        to_return.push(49); // c
        to_return.push(1);// xL
                            // xH
                            // yL
                            // yH
        
        panic!("Not implemented yet");
        to_return
    }
}
