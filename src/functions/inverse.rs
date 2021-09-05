macro_rules! generate_inverse {
    ( $fxx:ident ) => {
        pub fn inverse(n: $fxx) -> $fxx {
            1.0 - n
        }
    };
}
