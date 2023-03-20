use super::constants;
use super::network;

pub struct Printer {
    message: Vec<u8>,
}


impl Printer {
    ///Call this function to create a new printer object.
    ///It will start with the initial values ESC and @ which initializes the printer.
    ///This is done in order to reset the printer to initial state
    pub fn new() -> Printer {
        let mut p = Printer {
            message: Vec::new(),
        };
        p.message.push(constants::ESC);
        p.message.push(constants::AT);
        p
    }


    ///Add a list of u8 characters to be printed. 
    ///This is intended to mainly be used to print plaintext.
    ///It could however be used for commands as well.
    pub fn add(&mut self, text: Vec<u8>) {
        for i in text {
            self.message.push(i);
        }
    }

    ///Add a list of str to be printed. 
    ///keep in mind that the printer only works on ASCII characters.
    ///So it's the responsibility of the callee to make sure that the string is ASCII.
    ///If not then it will most likely be malformed.
    pub fn add_str(&mut self, text: &str) {
        for i in text.bytes() {
            self.message.push(i);
        }
    }


    ///This function will turn on or off double strike mode
    ///This is done by giving either the value constants::ON or constants::OFF
    pub fn set_double_strike(&mut self, value: u8) {
        self.message.push(constants::ESC);
        self.message.push(b'G');
        if value == constants::ON {
            self.message.push(constants::ON);
        } else {
            self.message.push(constants::OFF);
        }
    }

    ///Call this to either turn on or off emphisized text.
    ///This is done by giving either the value constants::ON or constants::OFF
    pub fn set_emph(&mut self, value: u8) {
        self.message.push(constants::ESC);
        self.message.push(b'E');
        if value == constants::ON {
            self.message.push(constants::ON);
        } else {
            self.message.push(constants::OFF);
        }
    }

    ///Call this to turn smoothing on or off
    ///This is done by giving either the value constants::ON or constants::OFF
    pub fn set_smoothing(&mut self, value: u8) {
        self.message.push(constants::GS);
        self.message.push(0x62);
        if value == constants::ON {
            self.message.push(constants::ON);
        } else {
            self.message.push(constants::OFF);
        }
    }


    fn _barcode_mode_a(&mut self, text:&str, barcode_type: u8) {
        self.message.push(constants::GS);
        self.message.push(0x6b);
        self.message.push(barcode_type);
        for i in text.bytes() {
            self.message.push(i);
        }
        self.message.push(0x00);
    }

    fn _barcode_mode_b(&mut self, text:&str, barcode_type: u8) {
        self.message.push(constants::GS);
        self.message.push(0x6b);
        self.message.push(barcode_type + 65);
        self.message.push(text.len() as u8);
        for i in text.bytes() {
            self.message.push(i);
        }
    }


    ///This function will set the height of the barcode.
    ///The actual height depends on the printer, try and find what works for you.
    ///Can be given values between 0-255
    pub fn set_barcode_height(&mut self, height: u8) {
        self.message.push(constants::GS);
        self.message.push(0x68);
        self.message.push(height);
    }

    ///This function will set the width of the barcode
    ///The actual width depends on the printer, try and find what works for you.
    ///Can be given values between 2-6
    ///Not sure why this is the case, but it is. 
    ///Will set to either highest or lowest if out of range.
    pub fn set_barcode_width(&mut self, mut width: u8) {
        if width >6 {
            width = 6;
        }
        else if width < 2 {
            width = 2;
        }
        self.message.push(constants::GS);
        self.message.push(0x77);
        self.message.push(width);
    }

    ///Prints a given barcode
    ///The barcode to print is given as a str
    ///There can be different types of barcodes. Check which you want to print https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=128
    ///There are two kinds of ways to print barcode, either constant::MODE_A or constant::MODE_B
    ///You have to know which mode is right for you.
    pub fn add_barcode(&mut self, text: &str, barcode_type: u8, mode: u8) {
        
        if mode == constants::MODE_A {
            self._barcode_mode_a(text, barcode_type);
        }
        else {
            self._barcode_mode_b( text, barcode_type);
        }
    }

    ///Call this to reverse feed paper.
    ///This is done by giving the number of lines to reverse feed.
    ///The maximum number of lines is 255.
    ///WARNING: I don't think this works on most printers
    pub fn reverse_feed(&mut self, lines: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x4b);
        self.message.push(lines);
    }

    ///Call this to either turn on or off upside down text
    ///This is done by giving either the value constants::ON or constants::OFF
    pub fn set_upside_down(&mut self, value: u8) {
        self.message.push(constants::ESC);
        self.message.push(b'{');
        if value == constants::ON {
            self.message.push(constants::ON);
        } else {
            self.message.push(constants::OFF);
        }
    }

    ///Call this to either set underline text on or off
    ///This is done by giving either the value constants::SINGLE, constant::DOUBLE or constants::OFF
    pub fn set_underline(&mut self, value: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x2D);
        self.message.push(value);
    }
    
    ///Call this to change the spacing between lines
    ///This is done by giving a value between 0 and 255
    pub fn set_line_spacing(&mut self, value: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x33);
        self.message.push(value);
    }

    ///Call this to set back to the default line spacing
    pub fn set_default_line_spacing(&mut self) {
        self.message.push(constants::ESC);
        self.message.push(0x32);
    }

    pub fn cut(&mut self) {
        self.message.push(constants::LF);
        self.message.push(constants::GS);
        self.message.push(0x56);
        self.message.push(0x41);
        self.message.push(0x08);
        self.message.push(constants::LF);
    }

    ///Sends a printjob to the correct address and port.
    ///This requires that other functions adding actions have been called before this.
    pub fn print(&self, address: String, port: u32) -> Result<(), String> {
        if self.message.len() == 2 {
            return Err("No message to print".to_string());
        }
        network::send_message(&self.message, address, port)
    }

}

