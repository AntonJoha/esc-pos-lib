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

///Used for setting different values
pub const ON: u8 = 0x01;
pub const OFF: u8 = 0x00;

///Used for setting underline
pub const SINGLE: u8 = 49;
pub const DOUBLE: u8 = 50;


///Other basic characters
///At symbol '@'
pub const AT: u8 = 0x40;


///Different modes
pub const MODE_A: u8 = 0x01;
pub const MODE_B: u8 = 0x00;


///Text alignment values for set_alignment
///Left alignment (default)
pub const ALIGN_LEFT: u8 = 0;
///Center alignment
pub const ALIGN_CENTER: u8 = 1;
///Right alignment
pub const ALIGN_RIGHT: u8 = 2;


///Font selection values for set_font
///Font A (default, larger)
pub const FONT_A: u8 = 0;
///Font B (smaller)
pub const FONT_B: u8 = 1;


///Barcode HRI (Human Readable Interpretation) position values
///Do not print HRI characters (default)
pub const HRI_NONE: u8 = 0;
///Print HRI characters above the barcode
pub const HRI_ABOVE: u8 = 1;
///Print HRI characters below the barcode
pub const HRI_BELOW: u8 = 2;
///Print HRI characters both above and below the barcode
pub const HRI_ABOVE_BELOW: u8 = 3;


///90-degree clockwise rotation values for set_rotation
///No rotation (default)
pub const ROTATION_OFF: u8 = 0;
///90-degree clockwise rotation
pub const ROTATION_90: u8 = 1;


///Different barcode types
pub const UPC_A: u8 = 0x00;
pub const UPC_E: u8 = 0x01;
pub const JAN13: u8 = 0x02;
pub const JAN8: u8 = 0x03;
pub const CODE39: u8 = 0x04;
pub const ITF: u8 = 0x05;
pub const CODABAR: u8 = 0x06;
pub const CODE93: u8 = 0x07;
pub const CODE128: u8 = 0x08;
pub const GS1_128: u8 = 0x09;
pub const DATABAR_OMNIDIRECTIONAL: u8 = 0x0A;
pub const DATABAR_TRUNCATED: u8 = 0x0B;
pub const DATABAR_LIMITED: u8 = 0x0C;
pub const DATABAR_EXPANDED: u8 = 0x0D;
pub const CODE128_AUTO: u8 = 0x0E;


///QR related constants
pub const QR_MODEL_1: u8 = 49;
pub const QR_MODEL_2: u8 = 50;
///Micro QR only works on TM-L90 4** models
pub const QR_MODEL_MICRO: u8 = 51;
///Different percentage of error correction
///L: 7% M: 15% Q: 25% H: 30%
pub const ERROR_L: u8 = 48;
pub const ERROR_M: u8 = 49;
pub const ERROR_Q: u8 = 50;
pub const ERROR_H: u8 = 51;


pub const MAX_X_WIDTH: u32 = 1024;
