#[macro_export]
macro_rules! generate_square_wave {
    ( $fxx:ident ) => {
        pub fn square_wave(n: $fxx, wave_duration: $fxx) -> $fxx {
            if square_wave_as_bool(n, wave_duration) {
                1.0
            } else {
                0.0
            }
        }

        pub fn square_wave_as_bool(n: $fxx, wave_duration: $fxx) -> bool {
            if (n % wave_duration) < (wave_duration / 2.0) {
                false
            } else {
                true
            }
        }
    }
}
