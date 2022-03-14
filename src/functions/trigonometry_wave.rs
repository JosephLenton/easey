macro_rules! generate_trigonometry_wave {
    ( $fxx:ident ) => {
        pub fn sin_wave(n: $fxx) -> $fxx {
            (::std::$fxx::consts::TAU * n).sin()
        }

        pub fn cos_wave(n: $fxx) -> $fxx {
            (::std::$fxx::consts::TAU * n).cos()
        }

        pub fn tan_wave(n: $fxx) -> $fxx {
            (::std::$fxx::consts::TAU * n).tan()
        }
    };
}
