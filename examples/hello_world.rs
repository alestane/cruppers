#![no_std]

use cpp::*;

#[no_mangle]
extern "C" fn greet() {
    println!("Hello, world!");
}
