macro_rules! generate_ease_in {
    ( $fxx:ident ) => {
        pub fn ease_in(n: $fxx) -> $fxx {
            use ::std::$fxx::consts::PI;
            1.0 - ((n * PI) / 2.0).cos()
        }

        pub fn ease_in_quad(n: $fxx) -> $fxx {
            n * n
        }

        pub fn ease_in_cubic(n: $fxx) -> $fxx {
            n * n * n
        }

        pub fn ease_in_quint(n: $fxx) -> $fxx {
            n * n * n * n * n
        }

        pub fn ease_in_circ(n: $fxx) -> $fxx {
            (1.0 - n.powi(2)).sqrt()
        }

        pub fn ease_in_expo(n: $fxx) -> $fxx {
            if n <= 0.0 {
                0.0
            } else {
                (2.0 as $fxx).powf(10.0 * n - 10.0)
            }
        }
    }
}
