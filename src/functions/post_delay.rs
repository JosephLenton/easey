macro_rules! generate_post_delay {
    ( $fxx:ident ) => {
        ///
        /// This compresses the 0.0 to 1.0 range,
        /// so that there is a delay at the end.
        ///
        /// i.e. From this graph ...
        ///
        /// ```text
        ///   |     /
        ///   |    /
        ///   |   /
        ///   |  /
        ///   | /
        ///   --------
        /// ```
        ///
        /// To this ...
        ///
        /// ```text
        ///   |     |--
        ///   |    |
        ///   |   |
        ///   |  |
        ///   | |
        ///   ---------
        /// ```
        ///
        /// The result starts more compressed to the left,
        /// followed by a flat line for the upper values.
        ///
        pub fn post_delay(n: $fxx, delay_point: $fxx) -> $fxx {
            if delay_point < n {
                1.0
            } else {
                n / delay_point
            }
        }
    };
}
