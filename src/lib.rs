#![no_std]
#![cfg_attr(feature="exception", feature(lang_items))]
#![cfg_attr(feature="exception", feature(slice_take))]

#[cfg(all(not(test), feature="exception"))]
mod exception;

#[cfg(all(not(test), feature="memory"))]
mod memory;

#[cfg(all(not(test), feature="iostream"))]
mod iostream;
