use super::constants;


pub struct Qr {
    model: u8,
    error_correction: u8,
    size: u8,
    data: Vec<u8>,
}

impl Qr {

    ///Initialize a new Qr code with the given data.
    ///The data must be a vector of bytes.
    pub fn new(data: Vec<u8>) -> Qr {
        Qr {
            model: constants::QR_MODEL_1,
            error_correction: constants::ERROR_L,
            size: 3,
            data: data,
        }
    }
    
    ///Set the model of QR code to print
    ///Can be either QR_MODEL_1, QR_MODEL_2 or QR_MODEL_MICRO
    ///However QR_MODEL_MICRO only works on TM-L90 4** models
    ///Default is QR_MODEL_1
    pub fn set_model(&mut self, model: u8) {
        self.model = model;
    }

    ///Set the error correction level of QR code to print
    ///Can be either ERROR_L, ERROR_M, ERROR_Q or ERROR_H
    ///Default is ERROR_L
    ///L: 7% M: 15% Q: 25% H: 30%
    pub fn set_error_correction(&mut self, error_correction: u8) {
        self.error_correction = error_correction;
    }

    ///Set the size of QR code to print
    ///Can be between 1 and 16
    ///Default is 3
    ///However some models only support 3-16
    ///If a value is set that is not supported by the model, it will be set to the closest value
    pub fn set_size(&mut self, size: u8) {
        if size < 1 {
            self.size = 1;
        } else if size > 16 {
            self.size = 16;
        } else {
            self.size = size;
        }
    }

    fn get_model(&self, data: &mut Vec<u8>) {
        data.push(constants::GS);
        data.push(0x28);
        data.push(0x6b);
        data.push(0x04);
        data.push(0x00);
        data.push(0x31);
        data.push(0x41);
        data.push(self.model);
        data.push(0x00);
    }

    fn get_error_correction(&self, data: &mut Vec<u8>) {
        data.push(constants::GS);
        data.push(0x28);
        data.push(0x6b);
        data.push(0x03);
        data.push(0x00);
        data.push(0x31);
        data.push(0x45);
        data.push(self.error_correction);
    }

    fn get_size(&self, data: &mut Vec<u8>) {
        data.push(constants::GS);
        data.push(0x28);
        data.push(0x6b);
        data.push(0x03);
        data.push(0x00);
        data.push(0x31);
        data.push(0x43);
        data.push(self.size);
    }

    fn get_data(&self, data: &mut Vec<u8>) {
        data.push(constants::GS);
        data.push(0x28);
        data.push(0x6b);
        data.push((self.data.len() + 3)  as u8);
        data.push(((self.data.len() + 3) >> 8) as u8);
        data.push(0x31);
        data.push(0x50);
        data.push(0x30);
        data.extend(self.data.clone());
    }

    fn add_print(&self, data: &mut Vec<u8>) {
        data.push(constants::GS);
        data.push(0x28);
        data.push(0x6b);
        data.push(0x03);
        data.push(0x00);
        data.push(0x31);
        data.push(0x51);
        data.push(48);
    }

    ///Call this to get the bytes to print the QR code
    pub fn export(&self) -> Vec<u8> {
        let mut to_return: Vec<u8> = Vec::new();
        
        //Set the model
        self.get_model(&mut to_return);

        //Set the error correction level
        self.get_error_correction(&mut to_return);

        //Set the size
        self.get_size(&mut to_return);

        //Add the data
        self.get_data(&mut to_return);

        //Print the QR code
        self.add_print(&mut to_return);

        to_return
    }
}


