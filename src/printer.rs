use super::constants;
use super::network;
use super::qr;
use std::fs::File;
use std::io::Write;

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


    ///Prints the given qr code. 
    ///The qr code is to be constructed with the qr submodule..
    pub fn add_qr(&mut self, qr: qr::Qr) {
        self.add(qr.export());
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

    ///Call this to set text alignment.
    ///Use constants::ALIGN_LEFT, constants::ALIGN_CENTER, or constants::ALIGN_RIGHT.
    ///Values greater than ALIGN_RIGHT are clamped to ALIGN_RIGHT.
    ///Default alignment is left.
    pub fn set_alignment(&mut self, mut value: u8) {
        if value > constants::ALIGN_RIGHT {
            value = constants::ALIGN_RIGHT;
        }
        self.message.push(constants::ESC);
        self.message.push(0x61);
        self.message.push(value);
    }

    ///Call this to select the character font.
    ///Use constants::FONT_A or constants::FONT_B.
    ///Font A is the default larger font; Font B is the smaller font.
    ///Values greater than FONT_B are clamped to FONT_B.
    pub fn set_font(&mut self, mut value: u8) {
        if value > constants::FONT_B {
            value = constants::FONT_B;
        }
        self.message.push(constants::ESC);
        self.message.push(0x4D);
        self.message.push(value);
    }

    ///Call this to set the character size (width and height multipliers).
    ///width_multiplier and height_multiplier can each be between 1 and 8.
    ///Values outside this range are clamped to 1 or 8.
    ///A multiplier of 1 means normal size; 2 means double size, etc.
    pub fn set_character_size(&mut self, mut width_multiplier: u8, mut height_multiplier: u8) {
        if width_multiplier < 1 { width_multiplier = 1; }
        if width_multiplier > 8 { width_multiplier = 8; }
        if height_multiplier < 1 { height_multiplier = 1; }
        if height_multiplier > 8 { height_multiplier = 8; }
        let n = ((width_multiplier - 1) << 4) | (height_multiplier - 1);
        self.message.push(constants::GS);
        self.message.push(0x21);
        self.message.push(n);
    }

    ///Call this to set extra space added to the right of each character.
    ///Value can be between 0 and 255 (in dots).
    ///Default is 0.
    pub fn set_character_spacing(&mut self, value: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x20);
        self.message.push(value);
    }

    ///Call this to set horizontal tab stop positions.
    ///Each value in the slice is a column number (1–255) for a tab stop.
    ///The positions must be given in ascending order.
    ///Pass an empty slice to clear all tab stops.
    pub fn set_tab_positions(&mut self, positions: &[u8]) {
        self.message.push(constants::ESC);
        self.message.push(0x44);
        for &pos in positions {
            self.message.push(pos);
        }
        self.message.push(0x00);
    }

    ///Call this to feed n lines.
    ///This is equivalent to printing n newlines without printing any data.
    pub fn feed(&mut self, lines: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x64);
        self.message.push(lines);
    }

    ///Call this to print and feed n lines.
    ///This is an explicit ESC/POS-named alias for feed().
    pub fn print_and_feed_lines(&mut self, lines: u8) {
        self.feed(lines);
    }

    ///Call this to print and reverse feed n lines.
    ///Uses the ESC e command.
    pub fn print_and_reverse_feed_lines(&mut self, lines: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x65);
        self.message.push(lines);
    }

    ///Call this to print and feed paper by n dots.
    ///Uses the ESC J command.
    pub fn print_and_feed_paper(&mut self, dots: u8) {
        self.message.push(constants::ESC);
        self.message.push(0x4A);
        self.message.push(dots);
    }

    fn _set_print_control_method(&mut self, function: u8, value: u8) {
        self.message.push(constants::GS);
        self.message.push(0x28);
        self.message.push(0x4B);
        self.message.push(0x02);
        self.message.push(0x00);
        self.message.push(function);
        self.message.push(value);
    }

    ///Draft implementation for selecting print density.
    ///Uses GS ( K function 49.
    pub fn select_print_density(&mut self, density: u8) {
        self._set_print_control_method(0x31, density);
    }

    ///Draft implementation for selecting print speed.
    ///Uses GS ( K function 50.
    pub fn select_print_speed(&mut self, speed: u8) {
        self._set_print_control_method(0x32, speed);
    }

    ///Call this to set the position of the HRI (Human Readable Interpretation) characters
    ///relative to the barcode.
    ///Use constants::HRI_NONE, HRI_ABOVE, HRI_BELOW, or HRI_ABOVE_BELOW.
    ///Values greater than HRI_ABOVE_BELOW are clamped to HRI_ABOVE_BELOW.
    pub fn set_barcode_hri_position(&mut self, mut value: u8) {
        if value > constants::HRI_ABOVE_BELOW {
            value = constants::HRI_ABOVE_BELOW;
        }
        self.message.push(constants::GS);
        self.message.push(0x48);
        self.message.push(value);
    }

    ///Call this to select the font used for barcode HRI characters.
    ///Use constants::FONT_A or constants::FONT_B.
    ///Values greater than FONT_B are clamped to FONT_B.
    pub fn set_barcode_hri_font(&mut self, mut value: u8) {
        if value > constants::FONT_B {
            value = constants::FONT_B;
        }
        self.message.push(constants::GS);
        self.message.push(0x66);
        self.message.push(value);
    }

    ///Call this to perform a partial cut (leaves a small uncut section).
    ///Use cut() for a full cut.
    pub fn partial_cut(&mut self) {
        self.message.push(constants::LF);
        self.message.push(constants::GS);
        self.message.push(0x56);
        self.message.push(0x42);
        self.message.push(0x01);
    }

    ///Call this to turn 90-degree clockwise rotation on or off.
    ///Use constants::ROTATION_OFF or constants::ROTATION_90.
    ///Values greater than ROTATION_90 are clamped to ROTATION_90.
    pub fn set_rotation(&mut self, mut value: u8) {
        if value > constants::ROTATION_90 {
            value = constants::ROTATION_90;
        }
        self.message.push(constants::ESC);
        self.message.push(0x56);
        self.message.push(value);
    }

    ///Call this to set the left margin.
    ///The margin is specified in dots (0–65535).
    ///This command is only effective at the beginning of a line.
    pub fn set_left_margin(&mut self, margin: u16) {
        self.message.push(constants::GS);
        self.message.push(0x4C);
        self.message.push((margin & 0xFF) as u8);
        self.message.push((margin >> 8) as u8);
    }

    ///Call this to set the print area width in dots (0–65535).
    ///This command is only effective at the beginning of a line.
    pub fn set_print_area_width(&mut self, width: u16) {
        self.message.push(constants::GS);
        self.message.push(0x57);
        self.message.push((width & 0xFF) as u8);
        self.message.push((width >> 8) as u8);
    }

    pub fn cut(&mut self) {
        self.message.push(constants::LF);
        self.message.push(constants::GS);
        self.message.push(0x56);
        self.message.push(0x41);
        self.message.push(0x08);
        self.message.push(constants::LF);
    }

    ///Returns the raw byte buffer that will be sent to the printer.
    ///Useful for testing or inspecting the command sequence.
    pub fn get_bytes(&self) -> &[u8] {
        &self.message
    }

    ///Prints the message straight to stdout
    pub fn print_stdout(&self) {
        for i in &self.message {
            print!("{}", *i as char);
        }
    }


    ///Prints the message to a file
    ///The path is given as a str
    pub fn print_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        for i in &self.message {
            file.write(&[*i]).unwrap();
        }
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
