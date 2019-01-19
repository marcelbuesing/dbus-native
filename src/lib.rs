#![deny(clippy::all, clippy::missing_inline_in_public_items)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod address;
mod message;
mod names;
mod reader;
mod type_system;
mod writer;
