//! Easey is for interpolation and easing for numbers between 0.0 and 1.0.
//!
//! It can be used as either stand alone functions, or a trait that adds
//! these functions onto f32 and f64 types.
//!
//! ```
//!   use ::easey::f32::ease_in;
//!   let n : f32 = ease_in(0.3);
//! ```
//!
//! ```
//!   use ::easey::Easey;
//!   let n : f32 = 0.3.ease_in();
//! ```
//!
//! The trait allows you to then easily chain functions, allowing one to combine them.
//! For example ...
//!
//! ```
//!   use ::easey::f32::Easey;
//!
//!   let n : f32 = 0.3.pre_delay(0.2).ease_in()
//! ```
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
