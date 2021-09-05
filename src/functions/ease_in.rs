macro_rules! generate_ease_in {
    ( $fxx:ident ) => {
        pub fn ease_in(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            1.0 - ((n * PI) / 2.0).cos()
        }

        pub fn ease_in_sine(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            1.0 - ((n * PI) / 2.0).cos()
        }

        pub fn ease_in_quad(n: $fxx) -> $fxx {
            n.powi(2)
        }

        pub fn ease_in_cubic(n: $fxx) -> $fxx {
            n.powi(3)
        }

        pub fn ease_in_quart(n: $fxx) -> $fxx {
            n.powi(4)
        }

        pub fn ease_in_quint(n: $fxx) -> $fxx {
            n.powi(5)
        }

        pub fn ease_in_expo(n: $fxx) -> $fxx {
            if n <= 0.0 {
                0.0
            } else {
                (2.0 as $fxx).powf(10.0 * n - 10.0)
            }
        }

        pub fn ease_in_circ(n: $fxx) -> $fxx {
            1.0 - (1.0 - n.powi(2)).sqrt()
        }

        pub fn ease_in_back(n: $fxx) -> $fxx {
            const C1: $fxx = 1.70158;
            const C3: $fxx = C1 + 1.0;

            C3 * n * n * n - C1 * n * n
        }
    };
}
