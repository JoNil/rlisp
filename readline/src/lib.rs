#![feature(slicing_syntax)]

extern crate libc;

use std::c_str;

mod ext_readline {
    use super::libc::c_char;
    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(line.to_c_str().as_ptr());
    }
}

fn readline(prompt: &str) -> Option<String> {
    let cprmt = prompt.to_c_str();
    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {
            None
        }
        else {
            c_str::CString::new(ret, true).as_str().map(|ret| ret.to_string())
        }
    }
}

pub fn read(prompt: &str) -> Option<String> {
    let input = match readline(prompt) { Some(line) => line, None => return None };
    add_history(input[]);
    Some(input)
}