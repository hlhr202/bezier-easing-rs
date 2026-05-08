use bezier_easing::{BezierEasing, BezierEasingError, bezier_easing};

const ERROR_MESSAGE: &str = "bezier x values must be in [0, 1] range";

#[test]
fn accepts_x_control_points_on_closed_unit_interval() {
    for (x1, x2) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)] {
        assert!(bezier_easing(x1, -2.0, x2, 3.0).is_ok(), "x1={x1}, x2={x2}");
    }
}

#[test]
fn rejects_x_control_points_outside_closed_unit_interval() {
    for result in [
        bezier_easing(-f64::EPSILON, 0.0, 0.5, 1.0),
        bezier_easing(1.0 + f64::EPSILON, 0.0, 0.5, 1.0),
        bezier_easing(0.5, 0.0, -f64::EPSILON, 1.0),
        bezier_easing(0.5, 0.0, 1.0 + f64::EPSILON, 1.0),
        bezier_easing(f64::NEG_INFINITY, 0.0, 0.5, 1.0),
        bezier_easing(f64::INFINITY, 0.0, 0.5, 1.0),
        bezier_easing(0.5, 0.0, f64::NEG_INFINITY, 1.0),
        bezier_easing(0.5, 0.0, f64::INFINITY, 1.0),
    ] {
        assert_error(result);
    }
}

#[test]
fn rejects_nan_x_control_points() {
    assert_error(bezier_easing(f64::NAN, 0.0, 0.5, 1.0));
    assert_error(bezier_easing(0.5, 0.0, f64::NAN, 1.0));
}

#[test]
fn allows_y_control_points_outside_unit_interval() {
    let easing = bezier_easing(0.25_f64, -10.0, 0.75, 10.0).unwrap();

    assert_eq!(easing.sample(0.0), 0.0);
    assert_eq!(easing.sample(1.0), 1.0);
    assert!(easing.sample(0.25).is_finite());
    assert!(easing.sample(0.75).is_finite());
}

#[test]
fn preserves_exact_endpoints_even_with_overshooting_y_values() {
    let easing = bezier_easing(0.4_f64, -100.0, 0.6, 100.0).unwrap();

    assert_eq!(easing.sample(0.0), 0.0);
    assert_eq!(easing.sample(1.0), 1.0);
}

#[test]
fn does_not_clamp_x_inputs_outside_unit_interval() {
    let ease_in = bezier_easing(0.42_f64, 0.0, 1.0, 1.0).unwrap();
    let ease_out = bezier_easing(0.0_f64, 0.0, 0.58, 1.0).unwrap();

    assert!(ease_in.sample(1.25) > 1.0);
    assert!(ease_out.sample(-0.25) < 0.0);
}

#[test]
fn propagates_nan_x_inputs() {
    let easing = bezier_easing(0.25_f64, 0.1, 0.25, 1.0).unwrap();

    assert!(easing.sample(f64::NAN).is_nan());
}

#[test]
fn f32_boundaries_match_f64_shape_with_reasonable_precision() {
    let easing_f32 = bezier_easing(0.25_f32, 0.1, 0.25, 1.0).unwrap();
    let easing_f64 = bezier_easing(0.25_f64, 0.1, 0.25, 1.0).unwrap();

    for x in [0.0_f32, 0.000001, 0.1, 0.25, 0.5, 0.75, 0.999999, 1.0] {
        let f32_value = easing_f32.sample(x) as f64;
        let f64_value = easing_f64.sample(x as f64);
        assert!(
            (f32_value - f64_value).abs() < 1e-5,
            "x={x}: f32={f32_value}, f64={f64_value}"
        );
    }
}

fn assert_error(result: Result<BezierEasing, BezierEasingError>) {
    match result {
        Ok(_) => panic!("expected invalid x control point to be rejected"),
        Err(error) => assert_eq!(error.to_string(), ERROR_MESSAGE),
    }
}
