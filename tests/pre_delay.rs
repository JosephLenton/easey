use ::assert_approx_eq::assert_approx_eq;
use ::easey::f32::pre_delay;

#[test]
fn it_should_return_zero_when_before_delay_point() {
    let n = 0.25;
    assert_approx_eq!(0.0, pre_delay(n, 0.3));
}

#[test]
fn it_should_have_when_at_half_during_delay() {
    let n = 0.7;
    assert_approx_eq!(0.5, pre_delay(n, 0.4));
}

#[test]
fn it_should_max_value_when_at_max() {
    let n = 1.0;
    assert_approx_eq!(1.0, pre_delay(n, 0.3));
}
