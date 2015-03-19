#![feature(libc)]

extern crate libc;

use self::libc::c_void;
use self::libc::free;
use std::ffi::CStr;
use std::ffi::CString;

mod ext_readline {
    use super::libc::c_char;
    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn read(prompt: &str) -> Option<String> {
    let cprmt = CString::new(prompt).unwrap();
    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {
            None
        }
        else {
            ext_readline::add_history(ret);
            let res = String::from_utf8_lossy(CStr::from_ptr(ret).to_bytes()).into_owned();
            free(ret as *mut c_void);
            Some(res)
        }
    }
}