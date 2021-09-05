pub trait Easey {
    fn inverse(self) -> Self;

    fn pre_delay(self, point: Self) -> Self;
    fn post_delay(self, point: Self) -> Self;

    fn ease_in(self) -> Self;
    fn ease_in_quad(self) -> Self;
    fn ease_in_cubic(self) -> Self;
    fn ease_in_quint(self) -> Self;
    fn ease_in_circ(self) -> Self;
    fn ease_in_expo(self) -> Self;

    fn ease_out(self) -> Self;
    fn ease_out_quad(self) -> Self;
    fn ease_out_cubic(self) -> Self;
    fn ease_out_quint(self) -> Self;
    fn ease_out_circ(self) -> Self;
    fn ease_out_expo(self) -> Self;

    fn ease_in_out(self) -> Self;
    fn ease_in_out_quad(self) -> Self;
    fn ease_in_out_cubic(self) -> Self;
    fn ease_in_out_quint(self) -> Self;
    fn ease_in_out_circ(self) -> Self;
    fn ease_in_out_expo(self) -> Self;

    fn square_wave(self, wave_duration: Self) -> Self;
    fn square_wave_as_bool(self, wave_duration: Self) -> bool;
}

macro_rules! impl_timing_trait {
    ( $fxx:ident ) => {
        impl Easey for $fxx {
            #[inline(always)]
            fn inverse(self) -> Self {
                crate::$fxx::inverse(self)
            }

            #[inline(always)]
            fn pre_delay(self, delay_point: Self) -> Self {
                crate::$fxx::pre_delay(self, delay_point)
            }

            #[inline(always)]
            fn post_delay(self, delay_point: Self) -> Self {
                crate::$fxx::post_delay(self, delay_point)
            }

            #[inline(always)]
            fn ease_in(self) -> Self {
                crate::$fxx::ease_in(self)
            }

            #[inline(always)]
            fn ease_in_quad(self) -> Self {
                crate::$fxx::ease_in_quad(self)
            }

            #[inline(always)]
            fn ease_in_cubic(self) -> Self {
                crate::$fxx::ease_in_cubic(self)
            }

            #[inline(always)]
            fn ease_in_quint(self) -> Self {
                crate::$fxx::ease_in_quint(self)
            }

            #[inline(always)]
            fn ease_in_circ(self) -> Self {
                crate::$fxx::ease_in_circ(self)
            }

            #[inline(always)]
            fn ease_in_expo(self) -> Self {
                crate::$fxx::ease_in_expo(self)
            }

            #[inline(always)]
            fn ease_out(self) -> Self {
                crate::$fxx::ease_out(self)
            }

            #[inline(always)]
            fn ease_out_quad(self) -> Self {
                crate::$fxx::ease_out_quad(self)
            }

            #[inline(always)]
            fn ease_out_cubic(self) -> Self {
                crate::$fxx::ease_out_cubic(self)
            }

            #[inline(always)]
            fn ease_out_quint(self) -> Self {
                crate::$fxx::ease_out_quint(self)
            }

            #[inline(always)]
            fn ease_out_circ(self) -> Self {
                crate::$fxx::ease_out_circ(self)
            }

            #[inline(always)]
            fn ease_out_expo(self) -> Self {
                crate::$fxx::ease_out_expo(self)
            }

            #[inline(always)]
            fn ease_in_out(self) -> Self {
                crate::$fxx::ease_in_out(self)
            }

            #[inline(always)]
            fn ease_in_out_quad(self) -> Self {
                crate::$fxx::ease_in_out_quad(self)
            }

            #[inline(always)]
            fn ease_in_out_cubic(self) -> Self {
                crate::$fxx::ease_in_out_cubic(self)
            }

            #[inline(always)]
            fn ease_in_out_quint(self) -> Self {
                crate::$fxx::ease_in_out_quint(self)
            }

            #[inline(always)]
            fn ease_in_out_circ(self) -> Self {
                crate::$fxx::ease_in_out_circ(self)
            }

            #[inline(always)]
            fn ease_in_out_expo(self) -> Self {
                crate::$fxx::ease_in_out_expo(self)
            }

            #[inline(always)]
            fn square_wave(self, wave_duration: $fxx) -> Self {
                crate::$fxx::square_wave(self, wave_duration)
            }

            #[inline(always)]
            fn square_wave_as_bool(self, wave_duration: $fxx) -> bool {
                crate::$fxx::square_wave_as_bool(self, wave_duration)
            }
        }
    };
}

impl_timing_trait!(f32);
impl_timing_trait!(f64);
