#![no_std]

use cpp::println;

#[no_mangle]
extern "C" fn greet() {
    println!("Hello, world!");
}