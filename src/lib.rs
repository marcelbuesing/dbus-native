#![deny(clippy::all, clippy::missing_inline_in_public_items)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

mod dbus_reader;
mod dbus_writer;
mod message;
mod names;
mod type_system;
