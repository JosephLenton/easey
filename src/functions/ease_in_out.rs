macro_rules! generate_ease_in_out {
    ( $fxx:ident ) => {
        pub fn ease_in_out(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            -((n * PI).cos() - 1.0) / 2.0
        }

        pub fn ease_in_out_sine(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            -((PI * n) - 1.0).cos() / 2.0
        }

        #[inline(always)]
        pub fn ease_in_out_quad(n: $fxx) -> $fxx {
            ease_in_out_power(n, 2)
        }

        #[inline(always)]
        pub fn ease_in_out_cubic(n: $fxx) -> $fxx {
            ease_in_out_power(n, 3)
        }

        #[inline(always)]
        pub fn ease_in_out_quart(n: $fxx) -> $fxx {
            ease_in_out_power(n, 4)
        }

        #[inline(always)]
        pub fn ease_in_out_quint(n: $fxx) -> $fxx {
            ease_in_out_power(n, 5)
        }

        fn ease_in_out_power(n: $fxx, pow: i32) -> $fxx {
            if n < 0.5 {
                (2.0 as $fxx).powi(pow - 1) * n.powi(pow)
            } else {
                1.0 - (-2.0 * n + 2.0).powi(pow) / 2.0
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

        pub fn ease_in_out_circ(n: $fxx) -> $fxx {
            if n < 0.5 {
                (1.0 - (1.0 - (2.0 * n).powi(2)).sqrt()) / 2.0
            } else {
                ((1.0 - (-2.0 * n + 2.0).powi(2)).sqrt() + 1.0) / 2.0
            }
        }

        pub fn ease_in_out_back(n: $fxx) -> $fxx {
            const C1: $fxx = 1.70158;
            const C2: $fxx = C1 + 1.525;

            if n < 0.5 {
                ((2.0 * n).powi(2) * ((C2 + 1.0) * 2.0 * n - C2)) / 2.0
            } else {
                ((2.0 * n - 2.0).powi(2) * ((C2 + 1.0) * (n * 2.0 - 2.0) + C2) + 2.0) / 2.0
            }
        }
    };
}
