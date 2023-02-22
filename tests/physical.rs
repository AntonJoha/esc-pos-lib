use esc_pos_lib::printer;
use esc_pos_lib::constants;
use std::process::Command;

/*

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

*/

//Just a basic print
fn basic(p: &mut printer::Printer) {
    
    p.add_str("Hello world");

}
//Emphisized text
fn emphisized(p: &mut printer::Printer) {
    
    p.set_emph(constants::ON);
    p.add_str("This is with emphisized text. ");
    p.set_emph(constants::OFF);
    p.add_str("This is with it off\n\n");
    

}
//Double strike
fn double_strike(p: &mut printer::Printer) {
    
    p.set_double_strike(constants::ON);
    p.add_str("This is with double strike on. ");
    p.set_double_strike(constants::OFF);
    p.add_str("This is with it off\n\n");


}
//Print a fortune
fn fortune(p: &mut printer::Printer) {
    
    let fortune = Command::new("fortune").output().unwrap().stdout;
    p.add(fortune);
    

}
//Underline text
fn underline(p: &mut printer::Printer) {
    
    p.set_underline(constants::SINGLE);
    p.add_str("This is with single underline on. \n");
    p.set_underline(constants::DOUBLE);
    p.add_str("This is with double underline on. \n");
    p.set_underline(constants::OFF);
    p.add_str("This is with it off\n\n");
}
//Horizontal tab
fn horizontal_tab(p: &mut printer::Printer) {
    p.add_str("This is a horizontal tab test. \t This is the second column. \n\n");
}
//Line spacing test
fn line_spacing(p: &mut printer::Printer) {
    p.set_line_spacing(100);
    p.add_str("This is with a spacing of 100. \n Test\n");
    p.set_line_spacing(50);
    p.add_str("This is with a spacing of 50. \n Test\n");
    p.set_line_spacing(10);
    p.add_str("This is with a spacing of 10. \nTest\n");
    p.set_default_line_spacing();
    p.add_str("This is with default line spacing. \n\n");
}

//Upside down test
fn upside_down(p: &mut printer::Printer) {
    p.set_upside_down(constants::ON);
    p.add_str("This is upside down. \n");
    p.set_upside_down(constants::OFF);
    p.add_str("This is not upside down. \n\n");
}

#[test]
#[ignore]
fn mass_test() {
    let mut p = printer::Printer::new();

    //Just a basic print
    basic(&mut p);

    //Emphisized text
    emphisized(&mut p);


    //Double strike text    //Print a fortune
    double_strike(&mut p);

    //Underline text
    underline(&mut p);

    //Print a fortune
    fortune(&mut p);

    //Horizontal tab test
    horizontal_tab(&mut p);

    //Line spacing test
    line_spacing(&mut p);

    //Upside down test
    upside_down(&mut p);

    p.cut();

    p.print("192.168.0.157".to_string(), 9100).unwrap();
}
