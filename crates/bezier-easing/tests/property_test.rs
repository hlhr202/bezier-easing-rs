use bezier_easing::bezier_easing;
use proptest::prelude::*;

const ENDPOINT_EPSILON: f64 = 0.0;
const SYMMETRY_EPSILON: f64 = 1e-8;
const F32_EPSILON: f64 = 1e-3;

fn unit() -> impl Strategy<Value = f64> {
    0.0_f64..=1.0
}

fn finite_y() -> impl Strategy<Value = f64> {
    -2.0_f64..=3.0
}

fn sample_x() -> impl Strategy<Value = f64> {
    0.0_f64..=1.0
}

fn common_curve() -> impl Strategy<Value = [f64; 4]> {
    prop::sample::select(vec![
        [0.0, 0.0, 1.0, 1.0],
        [0.25, 0.1, 0.25, 1.0],
        [0.42, 0.0, 1.0, 1.0],
        [0.0, 0.0, 0.58, 1.0],
        [0.42, 0.0, 0.58, 1.0],
        [0.25, 0.1, 0.0, 1.0],
    ])
}

proptest! {
    #[test]
    fn valid_curves_construct_and_preserve_endpoints(
        x1 in unit(),
        y1 in finite_y(),
        x2 in unit(),
        y2 in finite_y(),
    ) {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();

        prop_assert!((easing(0.0) - 0.0).abs() <= ENDPOINT_EPSILON);
        prop_assert!((easing(1.0) - 1.0).abs() <= ENDPOINT_EPSILON);
    }

    #[test]
    fn valid_curves_return_finite_values_for_unit_x(
        x1 in unit(),
        y1 in finite_y(),
        x2 in unit(),
        y2 in finite_y(),
        x in sample_x(),
    ) {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();

        prop_assert!(easing(x).is_finite());
    }

    #[test]
    fn same_parameters_are_deterministic(
        x1 in unit(),
        y1 in finite_y(),
        x2 in unit(),
        y2 in finite_y(),
        x in sample_x(),
    ) {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();
        let same_easing = bezier_easing(x1, y1, x2, y2).unwrap();

        prop_assert_eq!(easing(x), same_easing(x));
    }

    #[test]
    fn linear_curves_are_identity(a in unit(), b in unit(), x in sample_x()) {
        let easing = bezier_easing(a, a, b, b).unwrap();

        prop_assert_eq!(easing(x), x);
    }

    #[test]
    fn symmetric_curves_are_symmetric(a in unit(), b in finite_y(), x in sample_x()) {
        let easing = bezier_easing(a, b, 1.0 - a, 1.0 - b).unwrap();
        let expected = 1.0 - easing(1.0 - x);

        prop_assert!(
            (easing(x) - expected).abs() < SYMMETRY_EPSILON,
            "a={a}, b={b}, x={x}, easing(x)={}, expected={expected}",
            easing(x)
        );
    }

    #[test]
    fn f32_and_f64_results_are_close_for_common_curves(
        [x1, y1, x2, y2] in common_curve(),
        x in sample_x(),
    ) {
        let easing_f64 = bezier_easing(x1, y1, x2, y2).unwrap();
        let easing_f32 = bezier_easing(x1 as f32, y1 as f32, x2 as f32, y2 as f32).unwrap();
        let f64_value = easing_f64(x);
        let f32_value = easing_f32(x as f32) as f64;

        prop_assert!(
            (f64_value - f32_value).abs() < F32_EPSILON,
            "x1={x1}, y1={y1}, x2={x2}, y2={y2}, x={x}, f64={f64_value}, f32={f32_value}"
        );
    }
}
