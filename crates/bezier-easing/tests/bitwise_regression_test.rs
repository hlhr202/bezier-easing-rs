#![allow(clippy::excessive_precision)]

use bezier_easing::bezier_easing;

type LegacyBezierEasing<T = f64> = Box<dyn Fn(T) -> T + Send + Sync + 'static>;

const F64_CASES: [[f64; 4]; 10] = [
    [0.0, 0.0, 1.0, 1.0],
    [0.25, 0.1, 0.25, 1.0],
    [0.42, 0.0, 1.0, 1.0],
    [0.0, 0.0, 0.58, 1.0],
    [0.42, 0.0, 0.58, 1.0],
    [0.25, 0.1, 0.0, 1.0],
    [0.35, -0.5, 0.65, 1.5],
    [0.0, 0.75, 1.0 / 3.0, 0.25],
    [1.0000000000000001e-9, 0.8, 0.33333333433333334, 0.2],
    [1.0, 0.0, 1.0, 0.0],
];

const F64_SAMPLES: [f64; 11] = [
    -0.25, 0.0, 0.000001, 0.1, 0.25, 0.5, 0.75, 0.9, 0.999999, 1.0, 1.25,
];

const F32_CASES: [[f32; 4]; 10] = [
    [0.0, 0.0, 1.0, 1.0],
    [0.25, 0.1, 0.25, 1.0],
    [0.42, 0.0, 1.0, 1.0],
    [0.0, 0.0, 0.58, 1.0],
    [0.42, 0.0, 0.58, 1.0],
    [0.25, 0.1, 0.0, 1.0],
    [0.35, -0.5, 0.65, 1.5],
    [0.0, 0.75, 1.0 / 3.0, 0.25],
    [1.0e-9, 0.8, 0.33333334, 0.2],
    [1.0, 0.0, 1.0, 0.0],
];

const F32_SAMPLES: [f32; 11] = [
    -0.25, 0.0, 0.000001, 0.1, 0.25, 0.5, 0.75, 0.9, 0.999999, 1.0, 1.25,
];

#[test]
fn f64_samples_match_legacy_boxed_implementation_bitwise() {
    for [x1, y1, x2, y2] in F64_CASES {
        let current = bezier_easing(x1, y1, x2, y2).unwrap();
        let legacy = legacy_bezier_easing_f64(x1, y1, x2, y2);

        for x in F64_SAMPLES {
            let current_value = current.sample(x);
            let legacy_value = legacy(x);

            assert_eq!(
                current_value.to_bits(),
                legacy_value.to_bits(),
                "x1={x1}, y1={y1}, x2={x2}, y2={y2}, x={x}, current={current_value}, legacy={legacy_value}",
            );
        }
    }
}

#[test]
fn f32_samples_match_legacy_boxed_implementation_bitwise() {
    for [x1, y1, x2, y2] in F32_CASES {
        let current = bezier_easing(x1, y1, x2, y2).unwrap();
        let legacy = legacy_bezier_easing_f32(x1, y1, x2, y2);

        for x in F32_SAMPLES {
            let current_value = current.sample(x);
            let legacy_value = legacy(x);

            assert_eq!(
                current_value.to_bits(),
                legacy_value.to_bits(),
                "x1={x1}, y1={y1}, x2={x2}, y2={y2}, x={x}, current={current_value}, legacy={legacy_value}",
            );
        }
    }
}

fn legacy_y_f64(t: f64, ay: f64, by: f64, cy: f64) -> f64 {
    ((ay * t + 3.0 * by) * t + cy) * t
}

fn legacy_x2t_f64(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    let q = a + b * x;
    let s = q * q + c;

    if s > 0.0 {
        let root = s.sqrt();
        return (q + root).cbrt() + (q - root).cbrt() - d;
    }

    let l = (q * q - s).sqrt().cbrt();
    let angle = if q != 0.0 {
        ((-s).sqrt() / q).atan()
    } else {
        -std::f64::consts::PI / 2.0
    };

    let phi = if b < 0.0 {
        (if q > 0.0 {
            2.0 * std::f64::consts::PI
        } else {
            std::f64::consts::PI
        }) - angle
    } else if d < 0.0 {
        (if q > 0.0 {
            2.0 * std::f64::consts::PI
        } else {
            -3.0 * std::f64::consts::PI
        }) + angle
    } else {
        (if q > 0.0 { 0.0 } else { std::f64::consts::PI }) + angle
    };

    2.0 * l * (phi / 3.0).cos() - d
}

fn legacy_bezier_easing_f64(m_x1: f64, m_y1: f64, m_x2: f64, m_y2: f64) -> LegacyBezierEasing<f64> {
    if m_x1 == m_y1 && m_x2 == m_y2 {
        return Box::new(|x| x);
    }

    let a = 6.0 * (3.0 * m_x1 - 3.0 * m_x2 + 1.0);
    let b = 6.0 * (m_x2 - 2.0 * m_x1);
    let c = 3.0 * m_x1;

    let ay = 3.0 * m_y1 - 3.0 * m_y2 + 1.0;
    let by = m_y2 - 2.0 * m_y1;
    let cy = 3.0 * m_y1;

    if a == 0.0 {
        return Box::new(move |x| {
            if x == 0.0 || x == 1.0 {
                return x;
            }
            legacy_y_f64(x, ay, by, cy)
        });
    }

    let a2 = a * a;
    let b2 = b * b;
    let d = b / a;
    let e = (3.0 * b * c) / a2 - (b2 * b) / (a2 * a);
    let w1 = (2.0 * c) / a - b2 / a2;
    let w = w1 * w1 * w1;
    let o = 3.0 / a;

    Box::new(move |x| {
        if x == 0.0 || x == 1.0 {
            return x;
        }
        legacy_y_f64(legacy_x2t_f64(x, e, o, w, d), ay, by, cy)
    })
}

fn legacy_y_f32(t: f32, ay: f32, by: f32, cy: f32) -> f32 {
    ((ay * t + 3.0 * by) * t + cy) * t
}

fn legacy_x2t_f32(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let q = a + b * x;
    let s = q * q + c;

    if s > 0.0 {
        let root = s.sqrt();
        return (q + root).cbrt() + (q - root).cbrt() - d;
    }

    let l = (q * q - s).sqrt().cbrt();
    let angle = if q != 0.0 {
        ((-s).sqrt() / q).atan()
    } else {
        -std::f32::consts::PI / 2.0
    };

    let phi = if b < 0.0 {
        (if q > 0.0 {
            2.0 * std::f32::consts::PI
        } else {
            std::f32::consts::PI
        }) - angle
    } else if d < 0.0 {
        (if q > 0.0 {
            2.0 * std::f32::consts::PI
        } else {
            -3.0 * std::f32::consts::PI
        }) + angle
    } else {
        (if q > 0.0 { 0.0 } else { std::f32::consts::PI }) + angle
    };

    2.0 * l * (phi / 3.0).cos() - d
}

fn legacy_bezier_easing_f32(m_x1: f32, m_y1: f32, m_x2: f32, m_y2: f32) -> LegacyBezierEasing<f32> {
    if m_x1 == m_y1 && m_x2 == m_y2 {
        return Box::new(|x| x);
    }

    let a = 6.0 * (3.0 * m_x1 - 3.0 * m_x2 + 1.0);
    let b = 6.0 * (m_x2 - 2.0 * m_x1);
    let c = 3.0 * m_x1;

    let ay = 3.0 * m_y1 - 3.0 * m_y2 + 1.0;
    let by = m_y2 - 2.0 * m_y1;
    let cy = 3.0 * m_y1;

    if a == 0.0 {
        return Box::new(move |x| {
            if x == 0.0 || x == 1.0 {
                return x;
            }
            legacy_y_f32(x, ay, by, cy)
        });
    }

    let a2 = a * a;
    let b2 = b * b;
    let d = b / a;
    let e = (3.0 * b * c) / a2 - (b2 * b) / (a2 * a);
    let w1 = (2.0 * c) / a - b2 / a2;
    let w = w1 * w1 * w1;
    let o = 3.0 / a;

    Box::new(move |x| {
        if x == 0.0 || x == 1.0 {
            return x;
        }
        legacy_y_f32(legacy_x2t_f32(x, e, o, w, d), ay, by, cy)
    })
}
