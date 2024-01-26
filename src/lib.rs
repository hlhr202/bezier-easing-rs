/**
 * BezierEasing Rust - use bezier curve for transition easing function
 * 
 * This is a rust port of Gaëtan Renaudeau's bezier-easing from https://github.com/gre/bezier-easing
 * by 2024 Genkagaku – MIT License
 */
type BFloat = f32;

const NEWTON_ITERATIONS: usize = 4;
const NEWTON_MIN_SLOPE: BFloat = 0.001;
const SUBDIVISION_PRECISION: BFloat = 0.0000001;
const SUBDIVISION_MAX_ITERATIONS: usize = 10;

const K_SPLINE_TABLE_SIZE: usize = 11;
const K_SAMPLE_STEP_SIZE: BFloat = 1.0 / (K_SPLINE_TABLE_SIZE - 1) as BFloat;

#[inline]
fn a(a_a1: BFloat, a_a2: BFloat) -> BFloat {
    1.0 - 3.0 * a_a2 + 3.0 * a_a1
}

#[inline]
fn b(a_a1: BFloat, a_a2: BFloat) -> BFloat {
    3.0 * a_a2 - 6.0 * a_a1
}

#[inline]
fn c(a_a1: BFloat) -> BFloat {
    3.0 * a_a1
}

#[inline]
fn calc_bezier(a_t: BFloat, a_a1: BFloat, a_a2: BFloat) -> BFloat {
    ((a(a_a1, a_a2) * a_t + b(a_a1, a_a2)) * a_t + c(a_a1)) * a_t
}

#[inline]
fn get_slope(a_t: BFloat, a_a1: BFloat, a_a2: BFloat) -> BFloat {
    3.0 * a(a_a1, a_a2) * a_t * a_t + 2.0 * b(a_a1, a_a2) * a_t + c(a_a1)
}

#[inline]
fn binary_subdivide(a_x: BFloat, a_a: BFloat, a_b: BFloat, m_x1: BFloat, m_x2: BFloat) -> BFloat {
    let mut m_x1 = m_x1;
    let mut m_x2 = m_x2;
    let mut current_x: BFloat;
    let mut current_t = 0.0;
    let mut i = 0;
    while i < SUBDIVISION_MAX_ITERATIONS {
        current_t = m_x1 + (m_x2 - m_x1) / 2.0;
        current_x = calc_bezier(current_t, a_a, a_b) - a_x;
        if current_x > 0.0 {
            m_x2 = current_t;
        } else {
            m_x1 = current_t;
        }
        if current_x.abs() < SUBDIVISION_PRECISION {
            break;
        }
        i += 1;
    }
    current_t
}

fn newton_raphson_iterate(a_x: BFloat, a_guess_t: BFloat, a_a: BFloat, a_b: BFloat) -> BFloat {
    let mut guess_t = a_guess_t;
    for _ in 0..NEWTON_ITERATIONS {
        let current_slope = get_slope(guess_t, a_a, a_b);
        if current_slope == 0.0 {
            return guess_t;
        }
        let current_x = calc_bezier(guess_t, a_a, a_b) - a_x;
        guess_t -= current_x / current_slope;
    }
    guess_t
}

#[inline]
fn linear_easing(x: BFloat) -> BFloat {
    x
}

#[inline]
fn calc_sample_values(m_x1: BFloat, m_x2: BFloat) -> [BFloat; K_SPLINE_TABLE_SIZE] {
    let mut sample_values = [0.0; K_SPLINE_TABLE_SIZE];
    for (i, value) in sample_values.iter_mut().enumerate() {
        *value = calc_bezier(i as BFloat * K_SAMPLE_STEP_SIZE, m_x1, m_x2);
    }
    sample_values
}

#[inline]
fn get_t_for_x(x: BFloat, m_x1: BFloat, m_x2: BFloat) -> BFloat {
    let mut interval_start = 0.0;
    let mut current_sample = 1;
    let last_sample = K_SPLINE_TABLE_SIZE - 1;
    let sample_values = calc_sample_values(m_x1, m_x2);

    while current_sample != last_sample && sample_values[current_sample] <= x {
        interval_start += K_SAMPLE_STEP_SIZE;
        current_sample += 1;
    }
    current_sample -= 1;

    let dist = (x - sample_values[current_sample])
        / (sample_values[current_sample + 1] - sample_values[current_sample]);
    let guess_for_t = interval_start + dist * K_SAMPLE_STEP_SIZE;
    let initial_slope = get_slope(guess_for_t, m_x1, m_x2);
    if initial_slope >= NEWTON_MIN_SLOPE {
        newton_raphson_iterate(x, guess_for_t, m_x1, m_x2)
    } else if initial_slope == 0.0 {
        guess_for_t
    } else {
        binary_subdivide(
            x,
            interval_start,
            interval_start + K_SAMPLE_STEP_SIZE,
            m_x1,
            m_x2,
        )
    }
}

#[derive(Debug)]
pub struct BezierEasingError(String);

pub fn bezier_easing(
    m_x1: BFloat,
    m_y1: BFloat,
    m_x2: BFloat,
    m_y2: BFloat,
) -> Result<impl Fn(BFloat) -> BFloat, BezierEasingError> {
    if !((0.0..=1.0).contains(&m_x1) && (0.0..=1.0).contains(&m_x2)) {
        return Err(BezierEasingError("x values must be in [0, 1]".to_string()));
    }
    Ok(move |x: BFloat| {
        if m_x1 == m_y1 && m_x2 == m_y2 {
            return linear_easing(x);
        }
        if x == 0.0 || x == 1.0 {
            return x;
        }
        calc_bezier(get_t_for_x(x, m_x1, m_x2), m_y1, m_y2)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), BezierEasingError> {
        let ease = bezier_easing(0.0, 0.0, 1.0, 0.5)?;
        assert_eq!(ease(0.0), 0.0);
        assert_eq!(ease(0.5), 0.3125);
        assert_eq!(ease(1.0), 1.0);

        Ok(())
    }
}
