use bezier_easing::{BezierEasingError, bezier_easing};

const DEFAULT_PRECISION: f64 = 0.000001;

fn identity(x: f64) -> f64 {
    x
}

fn assert_close(a: f64, b: f64, message: impl AsRef<str>) {
    assert_close_with_precision(a, b, message, DEFAULT_PRECISION);
}

fn assert_close_with_precision(a: f64, b: f64, message: impl AsRef<str>, precision: f64) {
    assert!(
        (a - b).abs() < precision,
        "{}: expected {a} to be within {precision} of {b}",
        message.as_ref()
    );
}

fn all_equals(be1: impl Fn(f64) -> f64, be2: impl Fn(f64) -> f64, samples: usize, precision: f64) {
    for i in 0..=samples {
        let x = i as f64 / samples as f64;
        let message = format!("comparing value {x}");
        if precision == DEFAULT_PRECISION {
            assert_close(be1(x), be2(x), message);
        } else {
            assert_close_with_precision(be1(x), be2(x), message, precision);
        }
    }
}

struct Random {
    state: u64,
}

impl Random {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        ((self.state >> 32) as u32) as f64 / (u32::MAX as f64 + 1.0)
    }
}

#[test]
fn bezier_easing_is_a_function() {
    let constructor = bezier_easing;

    assert!(constructor(0.0_f64, 0.0, 1.0, 1.0).is_ok());
}

#[test]
fn creates_a_callable_easing_function() {
    let easing = bezier_easing(0.0_f64, 0.0, 1.0, 1.0).unwrap();

    assert_eq!(easing(0.5), 0.5);
}

#[test]
fn fails_with_wrong_arguments() {
    assert_wrong_arguments(bezier_easing(0.5_f64, 0.5, -5.0, 0.5));
    assert_wrong_arguments(bezier_easing(0.5_f64, 0.5, 5.0, 0.5));
    assert_wrong_arguments(bezier_easing(-2.0_f64, 0.5, 0.5, 0.5));
    assert_wrong_arguments(bezier_easing(2.0_f64, 0.5, 0.5, 0.5));
}

#[test]
fn linear_curves_are_linear() {
    let linear = bezier_easing(0.0_f64, 0.0, 1.0, 1.0).unwrap();
    let reversed_linear = bezier_easing(1.0_f64, 1.0, 0.0, 0.0).unwrap();

    all_equals(linear, reversed_linear, 100, DEFAULT_PRECISION);

    let linear = bezier_easing(0.0_f64, 0.0, 1.0, 1.0).unwrap();
    all_equals(linear, identity, 100, DEFAULT_PRECISION);
}

#[test]
fn returns_the_right_value_at_extremes() {
    let mut random = Random::new(1);

    for _ in 0..1000 {
        let a = random.next();
        let b = 2.0 * random.next() - 0.5;
        let c = random.next();
        let d = 2.0 * random.next() - 0.5;
        let easing = bezier_easing(a, b, c, d).unwrap();

        assert_eq!(easing(0.0), 0.0);
        assert_eq!(easing(1.0), 1.0);
    }
}

#[test]
fn approaches_the_projected_value_of_its_xy_projected_curve() {
    let mut random = Random::new(2);

    for _ in 0..1000 {
        let a = random.next();
        let b = random.next();
        let c = random.next();
        let d = random.next();
        let easing = bezier_easing(a, b, c, d).unwrap();
        let projected = bezier_easing(b, a, d, c).unwrap();
        let composed = |x| projected(easing(x));

        all_equals(identity, composed, 100, 0.05);
    }
}

#[test]
fn two_same_instances_are_strictly_equal() {
    let mut random = Random::new(3);

    for _ in 0..100 {
        let a = random.next();
        let b = 2.0 * random.next() - 0.5;
        let c = random.next();
        let d = 2.0 * random.next() - 0.5;
        let easing = bezier_easing(a, b, c, d).unwrap();
        let same_easing = bezier_easing(a, b, c, d).unwrap();

        all_equals(easing, same_easing, 100, DEFAULT_PRECISION);
    }
}

#[test]
fn symmetric_curves_have_a_central_value_close_to_half() {
    let mut random = Random::new(4);

    for _ in 0..100 {
        let a = random.next();
        let b = 2.0 * random.next() - 0.5;
        let c = 1.0 - a;
        let d = 1.0 - b;
        let easing = bezier_easing(a, b, c, d).unwrap();

        assert_close_with_precision(easing(0.5), 0.5, "easing(0.5) should be 0.5", 0.0005);
    }
}

#[test]
fn symmetric_curves_are_symmetric() {
    let mut random = Random::new(5);

    for _ in 0..100 {
        let a = random.next();
        let b = 2.0 * random.next() - 0.5;
        let c = 1.0 - a;
        let d = 1.0 - b;
        let easing = bezier_easing(a, b, c, d).unwrap();
        let sym = |x| 1.0 - easing(1.0 - x);

        all_equals(&easing, sym, 100, DEFAULT_PRECISION);
    }
}

#[test]
fn degenerate_x_curve_matches_js_identity_x_to_t_fallback() {
    let easing = bezier_easing(0.0_f64, 0.75, 1.0 / 3.0, 0.25).unwrap();

    assert_close(easing(0.25), 0.3671875, "a == 0 fallback should use t = x");
    assert_close(easing(0.5), 0.5, "a == 0 fallback should use t = x");
    assert_close(easing(0.75), 0.6328125, "a == 0 fallback should use t = x");
}

#[test]
fn supports_f32() {
    let easing = bezier_easing(0.0_f32, 0.0, 1.0, 0.5).unwrap();

    assert!((easing(0.5) - 0.3125).abs() < 0.000001);
}

fn assert_wrong_arguments(result: Result<bezier_easing::BezierEasing, BezierEasingError>) {
    match result {
        Ok(_) => panic!("expected wrong arguments to return an error"),
        Err(error) => assert_eq!(error.to_string(), "bezier x values must be in [0, 1] range"),
    }
}
