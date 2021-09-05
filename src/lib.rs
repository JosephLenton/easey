//! Easey is for interpolation and easing for numbers between 0.0 and 1.0.
//!

#![deny(private_in_public)]
#![deny(unused_must_use)]
#![deny(unused_mut)]
#![deny(unused_variables)]
#![deny(dead_code)]
#![deny(large_assignments)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#[macro_use]
mod functions;
mod easey;

pub mod f32;
pub mod f64;
pub use self::easey::*;
