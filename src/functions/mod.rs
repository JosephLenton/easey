//!
//! Many of the timings in this file are based on those from here: https://easings.net/#
//!
#[macro_use]
mod ease_in;
#[macro_use]
mod ease_in_out;
#[macro_use]
mod ease_out;
#[macro_use]
mod inverse;
#[macro_use]
mod minmax;
#[macro_use]
mod post_delay;
#[macro_use]
mod pre_delay;
#[macro_use]
mod square_wave;
#[macro_use]
mod trigonometry_wave;

macro_rules! generate_functions {
    ( $fxx:ident ) => {
        generate_ease_in!($fxx);
        generate_ease_in_out!($fxx);
        generate_ease_out!($fxx);
        generate_inverse!($fxx);
        generate_minmax!($fxx);
        generate_post_delay!($fxx);
        generate_pre_delay!($fxx);
        generate_square_wave!($fxx);
        generate_trigonometry_wave!($fxx);
    };
}
