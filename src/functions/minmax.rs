macro_rules! generate_minmax {
    ( $fxx:ident ) => {
        pub fn minmaxp(n: $fxx) -> $fxx {
            minmax(n, 0.0, 1.0)
        }

        pub fn minmax(n: $fxx, min: $fxx, max: $fxx) -> $fxx {
            #[cfg(debug_assertions)]
            {
                if max < min {
                    panic!("minmax called where max is less than min ... minmax({}, {}, {})", n, min, max);
                }
            }

            n.min(max).max(min)
        }
    };
}
