//! This contains the timing functions. They are implemented as macros,
//! so they can be used as f32 or f64.
mod functions;
pub use self::functions::*;

#[macro_export]
macro_rules! generate_functions {
  ( $fxx:ident ) => {
    ::f_generic::generate_ease_in!(f32);
    ::f_generic::generate_ease_in_out!(f32);
    ::f_generic::generate_ease_out!(f32);
    ::f_generic::generate_inverse!(f32);
    ::f_generic::generate_post_delay!(f32);
    ::f_generic::generate_pre_delay!(f32);
    ::f_generic::generate_square_wave!(f32);
  }
}
