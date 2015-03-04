extern crate gcc;

use std::env;

fn main() {
    env::set_var("CFLAGS", "-ansi -std=c99 -O3 -pedantic -pedantic-errors -Wall -Werror");
    gcc::compile_library("libmpc.a", &["src/mpc.c"]);
}