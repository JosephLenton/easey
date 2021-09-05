macro_rules! generate_ease_out {
    ( $fxx:ident ) => {
        pub fn ease_out(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            ((n * PI) / 2.0).sin()
        }

        pub fn ease_out_sine(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            ((n * PI) / 2.0).sin()
        }

        #[inline(always)]
        pub fn ease_out_quad(n: $fxx) -> $fxx {
            ease_out_power(n, 2)
        }

        #[inline(always)]
        pub fn ease_out_cubic(n: $fxx) -> $fxx {
            ease_out_power(n, 3)
        }

        #[inline(always)]
        pub fn ease_out_quart(n: $fxx) -> $fxx {
            ease_out_power(n, 4)
        }

        #[inline(always)]
        pub fn ease_out_quint(n: $fxx) -> $fxx {
            ease_out_power(n, 5)
        }

        fn ease_out_power(n: $fxx, pow: i32) -> $fxx {
            1.0 - (1.0 - n).powi(pow)
        }

        pub fn ease_out_expo(n: $fxx) -> $fxx {
            if n >= 1.0 {
                1.0
            } else {
                1.0 - (2.0 as $fxx).powf(-10.0 * n)
            }
        }

        pub fn ease_out_circ(n: $fxx) -> $fxx {
            (1.0 - (n - 1.0).powi(2)).sqrt()
        }

        pub fn ease_out_back(n: $fxx) -> $fxx {
            const C1: $fxx = 1.70158;
            const C3: $fxx = C1 + 1.0;

            1.0 + C3 * (n - 1.0).powi(3) + C1 * (n - 1.0).powi(2)
        }
    };
}
