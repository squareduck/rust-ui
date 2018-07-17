#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate uuid;

// mod component;
mod engine;
mod event;
mod template;
mod types;
pub mod vnode;

pub use template::template;
pub use vnode::builder::Builder;
