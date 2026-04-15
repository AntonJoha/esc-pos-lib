use esc_pos_lib::constants;
use esc_pos_lib::printer;

// Helper: create the initial bytes emitted by Printer::new() (ESC @)
fn init_bytes() -> Vec<u8> {
    vec![constants::ESC, constants::AT]
}

#[test]
fn test_set_alignment_left() {
    let mut p = printer::Printer::new();
    p.set_alignment(constants::ALIGN_LEFT);
    let expected = [init_bytes(), vec![constants::ESC, 0x61, 0]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_alignment_center() {
    let mut p = printer::Printer::new();
    p.set_alignment(constants::ALIGN_CENTER);
    let expected = [init_bytes(), vec![constants::ESC, 0x61, 1]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_alignment_right() {
    let mut p = printer::Printer::new();
    p.set_alignment(constants::ALIGN_RIGHT);
    let expected = [init_bytes(), vec![constants::ESC, 0x61, 2]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_font_a() {
    let mut p = printer::Printer::new();
    p.set_font(constants::FONT_A);
    let expected = [init_bytes(), vec![constants::ESC, 0x4D, 0]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_font_b() {
    let mut p = printer::Printer::new();
    p.set_font(constants::FONT_B);
    let expected = [init_bytes(), vec![constants::ESC, 0x4D, 1]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_character_size_normal() {
    let mut p = printer::Printer::new();
    p.set_character_size(1, 1);
    // (1-1)<<4 | (1-1) = 0
    let expected = [init_bytes(), vec![constants::GS, 0x21, 0]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_character_size_double_both() {
    let mut p = printer::Printer::new();
    p.set_character_size(2, 2);
    // (2-1)<<4 | (2-1) = 0x11
    let expected = [init_bytes(), vec![constants::GS, 0x21, 0x11]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_character_size_clamp_max() {
    let mut p = printer::Printer::new();
    p.set_character_size(9, 9);
    // clamped to 8: (8-1)<<4 | (8-1) = 0x77
    let expected = [init_bytes(), vec![constants::GS, 0x21, 0x77]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_character_size_clamp_min() {
    let mut p = printer::Printer::new();
    p.set_character_size(0, 0);
    // clamped to 1: (1-1)<<4 | (1-1) = 0
    let expected = [init_bytes(), vec![constants::GS, 0x21, 0]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_character_spacing() {
    let mut p = printer::Printer::new();
    p.set_character_spacing(5);
    let expected = [init_bytes(), vec![constants::ESC, 0x20, 5]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_tab_positions() {
    let mut p = printer::Printer::new();
    p.set_tab_positions(&[8, 16, 24]);
    let expected = [init_bytes(), vec![constants::ESC, 0x44, 8, 16, 24, 0x00]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_tab_positions_clear() {
    let mut p = printer::Printer::new();
    p.set_tab_positions(&[]);
    let expected = [init_bytes(), vec![constants::ESC, 0x44, 0x00]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_feed() {
    let mut p = printer::Printer::new();
    p.feed(3);
    let expected = [init_bytes(), vec![constants::ESC, 0x64, 3]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_barcode_hri_position_below() {
    let mut p = printer::Printer::new();
    p.set_barcode_hri_position(constants::HRI_BELOW);
    let expected = [init_bytes(), vec![constants::GS, 0x48, constants::HRI_BELOW]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_barcode_hri_font() {
    let mut p = printer::Printer::new();
    p.set_barcode_hri_font(constants::FONT_B);
    let expected = [init_bytes(), vec![constants::GS, 0x66, constants::FONT_B]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_partial_cut() {
    let mut p = printer::Printer::new();
    p.partial_cut();
    let expected = [init_bytes(), vec![constants::LF, constants::GS, 0x56, 0x42, 0x01]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_rotation_on() {
    let mut p = printer::Printer::new();
    p.set_rotation(constants::ROTATION_90);
    let expected = [init_bytes(), vec![constants::ESC, 0x56, constants::ROTATION_90]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_rotation_off() {
    let mut p = printer::Printer::new();
    p.set_rotation(constants::ROTATION_OFF);
    let expected = [init_bytes(), vec![constants::ESC, 0x56, constants::ROTATION_OFF]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_left_margin() {
    let mut p = printer::Printer::new();
    p.set_left_margin(100);
    let expected = [init_bytes(), vec![constants::GS, 0x4C, 100, 0]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_left_margin_large() {
    let mut p = printer::Printer::new();
    p.set_left_margin(512);
    // 512 = 0x200 => low=0x00, high=0x02
    let expected = [init_bytes(), vec![constants::GS, 0x4C, 0x00, 0x02]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_alignment_clamp() {
    let mut p = printer::Printer::new();
    p.set_alignment(99);
    // clamped to ALIGN_RIGHT (2)
    let expected = [init_bytes(), vec![constants::ESC, 0x61, constants::ALIGN_RIGHT]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_font_clamp() {
    let mut p = printer::Printer::new();
    p.set_font(99);
    // clamped to FONT_B (1)
    let expected = [init_bytes(), vec![constants::ESC, 0x4D, constants::FONT_B]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_barcode_hri_position_clamp() {
    let mut p = printer::Printer::new();
    p.set_barcode_hri_position(99);
    // clamped to HRI_ABOVE_BELOW (3)
    let expected = [init_bytes(), vec![constants::GS, 0x48, constants::HRI_ABOVE_BELOW]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_barcode_hri_font_clamp() {
    let mut p = printer::Printer::new();
    p.set_barcode_hri_font(99);
    // clamped to FONT_B (1)
    let expected = [init_bytes(), vec![constants::GS, 0x66, constants::FONT_B]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_rotation_clamp() {
    let mut p = printer::Printer::new();
    p.set_rotation(99);
    // clamped to ROTATION_90 (1)
    let expected = [init_bytes(), vec![constants::ESC, 0x56, constants::ROTATION_90]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}

#[test]
fn test_set_print_area_width() {
    let mut p = printer::Printer::new();
    p.set_print_area_width(576);
    // 576 = 0x240 => low=0x40, high=0x02
    let expected = [init_bytes(), vec![constants::GS, 0x57, 0x40, 0x02]].concat();
    assert_eq!(p.get_bytes(), expected.as_slice());
}