macro_rules! generate_ease_in_out {
    ( $fxx:ident ) => {
        pub fn ease_in_out(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            -((n * PI).cos() - 1.0) / 2.0
        }

        pub fn ease_in_out_quad(n: $fxx) -> $fxx {
            if n < 0.5 {
                2.0 * n.powi(2)
            } else {
                1.0 - (-2.0 * n + 2.0).powi(2) / 2.0
            }
        }

        pub fn ease_in_out_cubic(n: $fxx) -> $fxx {
            if n < 0.5 {
                4.0 * n.powi(3)
            } else {
                1.0 - (-2.0 * n + 2.0).powi(3) / 2.0
            }
        }

        pub fn ease_in_out_quint(n: $fxx) -> $fxx {
            if n < 0.5 {
                16.0 * n.powi(5)
            } else {
                1.0 - (-2.0 * n + 2.0).powi(5) / 2.0
            }
        }

        pub fn ease_in_out_circ(n: $fxx) -> $fxx {
            if n < 0.5 {
                (1.0 - (1.0 - (2.0 * n).powi(2)).sqrt()) / 2.0
            } else {
                ((1.0 - (-2.0 * n + 2.0).powi(2)).sqrt() + 1.0) / 2.0
            }
        }

        pub fn ease_in_out_expo(n: $fxx) -> $fxx {
            if n <= 0.0 {
                0.0
            } else if n >= 1.0 {
                1.0
            } else if n < 0.5 {
                (2.0 as $fxx).powf(20.0 * n - 10.0) / 2.0
            } else {
                (2.0 - (2.0 as $fxx).powf(-20.0 * n + 10.0)) / 2.0
            }
        }
    };
}
