#![crate_type = "dylib"]

extern crate libc;

use libc::{c_char,c_int};
use std::ffi::CStr;
use std::str;
//use std::Vec;

fn parse_c_str(c_buf:*const c_char) -> Option<String> {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    match str::from_utf8(buf) {
        Ok(str_slice) => Some(str_slice.to_string()),
        Err(_) => None
    }
}

#[no_mangle]
pub extern fn main(argc:c_int, argv:*const *const c_char) -> i32 {
    let mut result:Vec<String> = Vec::new();
    for idx in 0..(argc) {
        let arg = unsafe {parse_c_str(*argv.offset(idx as isize))};
        match arg {
            Some(x) => result.push(x),
            _ => {}
        }
    }
    println!("Hello world from Rust (argc={})\n", argc);
    let mut idx = 0;
    for x in result {
        println!("argv[{}]={}\n", idx, x);
        idx+= 1;
    }
    1
}
