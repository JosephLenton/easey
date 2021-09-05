use ::assert_approx_eq::assert_approx_eq;
use ::easey::f32::post_delay;

#[test]
fn it_should_return_half_when_at_half_delay_point() {
    let n = 0.35;
    assert_approx_eq!(0.5, post_delay(n, 0.7));
}

#[test]
fn it_should_max_value_when_beyond_delay_point() {
    let n = 0.9;
    assert_approx_eq!(1.0, post_delay(n, 0.7));
}
