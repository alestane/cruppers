#![no_std]
#![allow(internal_features)]
#![cfg_attr(feature="exception", feature(lang_items))]
#![cfg_attr(feature="exception", feature(slice_take))]

#[cfg(all(not(test), feature="exception"))]
pub mod exception;

#[cfg(all(not(test), feature="memory"))]
pub mod memory;

#[cfg(all(not(test), feature="iostream"))]
pub mod iostream;
