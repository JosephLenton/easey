macro_rules! generate_pre_delay {
    ( $fxx:ident ) => {
        /**
         * This compresses the 0.0 to 1.0 range,
         * so that there is a delay at the start.
         *
         * i.e. From this graph ...
         *
         *   |      /
         *   |     /
         *   |    /
         *   |   /
         *   | /
         *   --------
         *
         * To this ...
         *
         *   |       /
         *   |      |
         *   |     /
         *   |    |
         *   |   /
         *   --------
         *
         * ^ Note how the graph is more compressed on the right.
         */
        pub fn pre_delay(n: $fxx, delay_point: $fxx) -> $fxx {
            if n < delay_point {
                0.0
            } else {
                (n - delay_point) / (1.0 - delay_point)
            }
        }
    }
}
