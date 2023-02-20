use esc_pos_lib::printer;



#[test]
#[ignore]
fn print_and_cut() {
    let mut p = printer::Printer::new();
    p.add_str("Hello world");
    p.cut();
    p.print("192.168.0.157".to_string(), 9100).unwrap();
}
