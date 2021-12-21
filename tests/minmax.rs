use ::assert_approx_eq::assert_approx_eq;
use ::easey::f32::minmax;

#[test]
fn it_should_return_min_when_greater_than_self() {
    let n = 0.1;
    assert_approx_eq!(0.2, minmax(n, 0.2, 0.7));
}

#[test]
fn it_should_return_self_when_between_min_max() {
    let n = 0.5;
    assert_approx_eq!(0.5, minmax(n, 0.2, 0.7));
}

#[test]
fn it_should_return_max_when_less_than_self() {
    let n = 0.9;
    assert_approx_eq!(0.7, minmax(n, 0.2, 0.7));
}
