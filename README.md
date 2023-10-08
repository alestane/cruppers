# Cruppers

This is a lightweight crate to support using `#![no_std]` Rust libraries in C++ programs. It uses C++ facilities (new/delete, exceptions, iostream) to provide the facilities (alloc, panic, print macros) that Rust crates normally lose access to when they give up the standard library.

This makes it fairly easy to create Rust libraries that can be used from either Rust or C++. The assumed use case is that you create a Rust crate with a C API and a `crate-type` that includes `staticlib`, then add a dependency on this crate. If you don't want a specific feature (like stdout), you can disable building it by disabling default features on the dependency and enabling them individually.

**This crate is in pretty early development. It passes initial use cases for most features, but it hasn't been pushed very far yet.**

**Currently, this crate requires nightly builds; that dependency should be removed once the appropriate features are stabilized.**
