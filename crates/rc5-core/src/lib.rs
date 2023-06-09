#![no_std]

pub mod algs;
pub mod traits;

pub mod words;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;
