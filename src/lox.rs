use crate::scanner::Scanner;

pub struct Lox {}
impl Lox {
    pub fn run(text: &str) {
        let mut scanner = Scanner::new(text.to_string());
        scanner.scan_tokens();
        scanner.print_info();
    }
}
