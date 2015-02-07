#![feature(env)]

extern crate readline;
extern crate rlisp;

use std::env;
use std::str::Str;

const PROMPT: &'static str = "rlisp> ";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    println!("RLisp");

    let mut rlisp = rlisp::Rlisp::new();

    loop {
        let line = match readline::read(PROMPT) {
            Some(line) => line, None => return
        };

        println!("{}", rlisp.execute(line.as_slice()));
    }
}