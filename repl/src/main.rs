extern crate readline;
extern crate rlisp;

use std::os;

const PROMPT: &'static str = "rlisp> ";

fn main() {
    os::setenv("RUST_BACKTRACE", "1");

    println!("RLisp");

    let mut rlisp = rlisp::Rlisp::new();

    loop {
        let line = match readline::read(PROMPT) {
            Some(line) => line, None => return
        };

        println!("{}", rlisp.execute(line[]));
    }
}