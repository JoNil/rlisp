#![allow(unstable)]

extern crate gcc;

use std::default::Default;
use std::os;

fn main() {
    os::setenv("CFLAGS", "-ansi -std=c99 -pedantic -pedantic-errors -Wall -Werror");
    gcc::compile_library("libmpc.a", &Default::default(), &["src/mpc.c"]);
}