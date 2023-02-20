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
    pub fn add_text(&mut self, text: Vec<u8>) {
        for i in text {
            self.message.push(i);
        }
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

