///This is the list of all constants which are used.
///They are derived from the list of specifications given from
///[this](https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=2) page.

///Horizontal tab. <https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=52>
pub const HT: u8 = 0x09;
///Print and line feed. <https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=10>
pub const LF: u8 = 0x0A;
///Print and return to standard mode. <https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=12>
pub const FF: u8 = 0x0C; 
///Print and carriage return. <https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=13>
pub const CR: u8 = 0x0D;
///Cancel print data in Page mode.
///<https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=21>
pub const CAN: u8 = 0x18;
///ESC character 
pub const ESC: u8 = 0x1B;
///DLE character    
pub const DLE: u8 = 0x10;
///GS character
pub const GS: u8 = 0x1D;
///FS character
pub const FS: u8 = 0x1C;



///Other basic characters
///At symbol '@'
pub const AT: u8 = 0x40;
