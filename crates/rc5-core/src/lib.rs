#![no_std]

pub mod algs;
pub mod traits;

pub mod std_words;

#[macro_use]
pub mod strange_words;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;
