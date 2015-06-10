#![crate_type = "dylib"]

#[no_mangle]
pub extern fn main(argc:i32, argv:&[str]) -> i32 {
    println!("Hello world from Rust\n");
    1
}
