//! BezierEasing Rust - use Bezier curves for transition easing functions.
//!
//! This is a Rust port of Gaëtan Renaudeau's bezier-easing from
//! <https://github.com/gre/bezier-easing>.
//! by 2024 Genkagaku - MIT License.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// Floating point types supported by [`bezier_easing`].
pub trait BezierFloat:
    Copy
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + 'static
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const SIX: Self;
    const PI: Self;

    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn atan(self) -> Self;
    fn cos(self) -> Self;
}

macro_rules! impl_bezier_float {
    ($type:ty, $consts:ident) => {
        impl BezierFloat for $type {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            const TWO: Self = 2.0;
            const THREE: Self = 3.0;
            const SIX: Self = 6.0;
            const PI: Self = std::$consts::consts::PI;

            #[inline]
            fn sqrt(self) -> Self {
                <$type>::sqrt(self)
            }

            #[inline]
            fn cbrt(self) -> Self {
                <$type>::cbrt(self)
            }

            #[inline]
            fn atan(self) -> Self {
                <$type>::atan(self)
            }

            #[inline]
            fn cos(self) -> Self {
                <$type>::cos(self)
            }
        }
    };
}

impl_bezier_float!(f32, f32);
impl_bezier_float!(f64, f64);

/// Bezier easing function. Defaults to `f64`.
#[derive(Debug, Clone, Copy)]
pub struct BezierEasing<T = f64> {
    kind: BezierEasingKind<T>,
}

#[derive(Debug, Clone, Copy)]
enum BezierEasingKind<T> {
    Linear,
    Degenerate {
        ay: T,
        by: T,
        cy: T,
    },
    Cubic {
        e: T,
        o: T,
        w: T,
        d: T,
        ay: T,
        by: T,
        cy: T,
    },
}

#[inline]
fn linear_easing<T: BezierFloat>(x: T) -> T {
    x
}

#[inline]
fn y<T: BezierFloat>(t: T, ay: T, by: T, cy: T) -> T {
    ((ay * t + T::THREE * by) * t + cy) * t
}

#[inline]
fn x2t<T: BezierFloat>(x: T, a: T, b: T, c: T, d: T) -> T {
    let q = a + b * x;
    let s = q * q + c;

    if s > T::ZERO {
        let root = s.sqrt();
        return (q + root).cbrt() + (q - root).cbrt() - d;
    }

    let l = (q * q - s).sqrt().cbrt();
    let angle = if q != T::ZERO {
        ((-s).sqrt() / q).atan()
    } else {
        -T::PI / T::TWO
    };

    let phi = if b < T::ZERO {
        (if q > T::ZERO { T::TWO * T::PI } else { T::PI }) - angle
    } else if d < T::ZERO {
        (if q > T::ZERO {
            T::TWO * T::PI
        } else {
            -T::THREE * T::PI
        }) + angle
    } else {
        (if q > T::ZERO { T::ZERO } else { T::PI }) + angle
    };

    T::TWO * l * (phi / T::THREE).cos() - d
}

#[derive(Debug, Clone)]
pub struct BezierEasingError(String);

impl std::fmt::Display for BezierEasingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for BezierEasingError {}

impl<T: BezierFloat> BezierEasing<T> {
    /// Create a Bezier easing function.
    pub fn new(m_x1: T, m_y1: T, m_x2: T, m_y2: T) -> Result<Self, BezierEasingError> {
        if !(T::ZERO <= m_x1 && m_x1 <= T::ONE && T::ZERO <= m_x2 && m_x2 <= T::ONE) {
            return Err(BezierEasingError(
                "bezier x values must be in [0, 1] range".to_string(),
            ));
        }

        if m_x1 == m_y1 && m_x2 == m_y2 {
            return Ok(Self {
                kind: BezierEasingKind::Linear,
            });
        }

        let a = T::SIX * (T::THREE * m_x1 - T::THREE * m_x2 + T::ONE);
        let b = T::SIX * (m_x2 - T::TWO * m_x1);
        let c = T::THREE * m_x1;

        let ay = T::THREE * m_y1 - T::THREE * m_y2 + T::ONE;
        let by = m_y2 - T::TWO * m_y1;
        let cy = T::THREE * m_y1;

        if a == T::ZERO {
            return Ok(Self {
                kind: BezierEasingKind::Degenerate { ay, by, cy },
            });
        }

        let a2 = a * a;
        let b2 = b * b;
        let d = b / a;
        let e = (T::THREE * b * c) / a2 - (b2 * b) / (a2 * a);
        let w1 = (T::TWO * c) / a - b2 / a2;
        let w = w1 * w1 * w1;
        let o = T::THREE / a;

        Ok(Self {
            kind: BezierEasingKind::Cubic {
                e,
                o,
                w,
                d,
                ay,
                by,
                cy,
            },
        })
    }

    /// Sample the easing curve at `x`.
    #[inline]
    pub fn sample(&self, x: T) -> T {
        match self.kind {
            BezierEasingKind::Linear => linear_easing(x),
            BezierEasingKind::Degenerate { ay, by, cy } => {
                if x == T::ZERO || x == T::ONE {
                    return x;
                }
                y(x, ay, by, cy)
            }
            BezierEasingKind::Cubic {
                e,
                o,
                w,
                d,
                ay,
                by,
                cy,
            } => {
                if x == T::ZERO || x == T::ONE {
                    return x;
                }
                y(x2t(x, e, o, w, d), ay, by, cy)
            }
        }
    }
}

/// Create a Bezier easing function.
/// ## Examples
/// ```
/// use bezier_easing::bezier_easing;
/// let ease = bezier_easing(0.0_f64, 0.0, 1.0, 0.5).unwrap();
/// assert_eq!(ease.sample(0.0), 0.0);
/// assert!((ease.sample(0.5) - 0.3125).abs() < 0.000001);
/// assert_eq!(ease.sample(1.0), 1.0);
/// ```
pub fn bezier_easing<T: BezierFloat>(
    m_x1: T,
    m_y1: T,
    m_x2: T,
    m_y2: T,
) -> Result<BezierEasing<T>, BezierEasingError> {
    BezierEasing::new(m_x1, m_y1, m_x2, m_y2)
}
