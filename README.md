# Cruppers

This is a lightweight crate to support using `#![no_std]` Rust libraries in C++ programs. It uses C++ facilities (new/delete, exceptions, iostream) to provide the facilities (alloc, panic, print macros) that Rust crates normally lose access to when they give up the standard library.

This makes it fairly easy to create Rust libraries that can be used from either Rust or C++. You should not need to do anything but set a dependency on it, specify which features you will use (you can omit `memory` or `exception` independently if you do not need them or want to supply your own), and link to it using `extern crate cpp;`

**This crate is in _very_ early development. In particular, stdout/stderr is not implemented yet.**

**Currently, this crate requires nightly builds; that dependency should be removed once the appropriate features are stabilized.**