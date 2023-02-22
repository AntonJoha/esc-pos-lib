use esc_pos_lib::printer;
use esc_pos_lib::constants;
use std::process::Command;

#[test]
#[ignore]
fn print_and_cut() {
    let mut p = printer::Printer::new();
    p.add_str("Hello world");
    p.cut();
    p.print("192.168.0.157".to_string(), 9100).unwrap();
}


#[test]
#[ignore]
fn emphisized() {
    let mut p = printer::Printer::new();
    p.set_emph(constants::ON);
    p.add_str("This is with emphisized text. ");
    p.set_emph(constants::OFF);
    p.add_str("This is with it off\n\n");
    p.cut();
    p.print("192.168.0.157".to_string(), 9100).unwrap();
}


#[test]
#[ignore]
fn double_strike() {
    let mut p = printer::Printer::new();
    p.set_double_strike(constants::ON);
    p.add_str("This is with double strike on. ");
    p.set_double_strike(constants::OFF);
    p.add_str("This is with it off\n");
    p.cut();
    p.print("192.168.0.157".to_string(), 9100).unwrap();
}


#[test]
#[ignore]
fn print_fortune() {
    let mut p = printer::Printer::new();
    let fortune = Command::new("fortune").output().unwrap().stdout;
    p.add(fortune);
    p.cut();
    p.print("192.168.0.157".to_string(), 9100).unwrap();
}


