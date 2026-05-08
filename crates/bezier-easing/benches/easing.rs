use bezier_easing::{BezierFloat, bezier_easing};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::time::Duration;

type LegacyBezierEasing<T> = Box<dyn Fn(T) -> T + Send + Sync + 'static>;

// Keep the default benchmark matrix small enough for local iteration. These
// curves cover the cheap linear path, normal cubic path, degenerate path, and a
// near-degenerate cubic path.
const F64_CURVES: [(&str, [f64; 4]); 4] = [
    ("linear", [0.0, 0.0, 1.0, 1.0]),
    ("css_ease", [0.25, 0.1, 0.25, 1.0]),
    ("degenerate_a_zero", [0.0, 0.75, 1.0 / 3.0, 0.25]),
    ("near_degenerate_x", [1.0e-9, 0.8, 0.33333333433333334, 0.2]),
];

const F32_CURVES: [(&str, [f32; 4]); 4] = [
    ("linear", [0.0, 0.0, 1.0, 1.0]),
    ("css_ease", [0.25, 0.1, 0.25, 1.0]),
    ("degenerate_a_zero", [0.0, 0.75, 1.0 / 3.0, 0.25]),
    ("near_degenerate_x", [1.0e-9, 0.8, 0.33333334, 0.2]),
];

const SAMPLE_COUNT: usize = 1024;

fn bench_construct_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct/f64");

    for (name, [x1, y1, x2, y2]) in F64_CURVES {
        group.bench_function(BenchmarkId::new("current", name), |b| {
            b.iter(|| {
                black_box(
                    bezier_easing(black_box(x1), black_box(y1), black_box(x2), black_box(y2))
                        .unwrap(),
                )
            })
        });
        group.bench_function(BenchmarkId::new("legacy_boxed", name), |b| {
            b.iter(|| {
                black_box(
                    legacy_bezier_easing(
                        black_box(x1),
                        black_box(y1),
                        black_box(x2),
                        black_box(y2),
                    )
                    .unwrap(),
                )
            })
        });
    }

    group.finish();
}

fn bench_construct_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct/f32");

    for (name, [x1, y1, x2, y2]) in F32_CURVES {
        group.bench_function(BenchmarkId::new("current", name), |b| {
            b.iter(|| {
                black_box(
                    bezier_easing(black_box(x1), black_box(y1), black_box(x2), black_box(y2))
                        .unwrap(),
                )
            })
        });
        group.bench_function(BenchmarkId::new("legacy_boxed", name), |b| {
            b.iter(|| {
                black_box(
                    legacy_bezier_easing(
                        black_box(x1),
                        black_box(y1),
                        black_box(x2),
                        black_box(y2),
                    )
                    .unwrap(),
                )
            })
        });
    }

    group.finish();
}

fn bench_sample_single_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample_single/f64");

    for (name, [x1, y1, x2, y2]) in F64_CURVES {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();
        let legacy = legacy_bezier_easing(x1, y1, x2, y2).unwrap();

        group.bench_function(BenchmarkId::new("current", name), |b| {
            b.iter(|| black_box(easing.sample(black_box(0.5))))
        });
        group.bench_function(BenchmarkId::new("legacy_boxed", name), |b| {
            b.iter(|| black_box(legacy(black_box(0.5))))
        });
    }

    group.finish();
}

fn bench_sample_single_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample_single/f32");

    for (name, [x1, y1, x2, y2]) in F32_CURVES {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();
        let legacy = legacy_bezier_easing(x1, y1, x2, y2).unwrap();

        group.bench_function(BenchmarkId::new("current", name), |b| {
            b.iter(|| black_box(easing.sample(black_box(0.5))))
        });
        group.bench_function(BenchmarkId::new("legacy_boxed", name), |b| {
            b.iter(|| black_box(legacy(black_box(0.5))))
        });
    }

    group.finish();
}

fn bench_sample_batch_f64(c: &mut Criterion) {
    let samples = f64_samples();
    let mut group = c.benchmark_group("sample_batch_1024/f64");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    for (name, [x1, y1, x2, y2]) in F64_CURVES {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();
        let legacy = legacy_bezier_easing(x1, y1, x2, y2).unwrap();

        group.bench_with_input(BenchmarkId::new("current", name), &samples, |b, samples| {
            b.iter(|| {
                let mut sum = 0.0;
                for &x in samples {
                    sum += easing.sample(black_box(x));
                }
                black_box(sum)
            })
        });
        group.bench_with_input(
            BenchmarkId::new("legacy_boxed", name),
            &samples,
            |b, samples| {
                b.iter(|| {
                    let mut sum = 0.0;
                    for &x in samples {
                        sum += legacy(black_box(x));
                    }
                    black_box(sum)
                })
            },
        );
    }

    group.finish();
}

fn bench_sample_batch_f32(c: &mut Criterion) {
    let samples = f32_samples();
    let mut group = c.benchmark_group("sample_batch_1024/f32");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    for (name, [x1, y1, x2, y2]) in F32_CURVES {
        let easing = bezier_easing(x1, y1, x2, y2).unwrap();
        let legacy = legacy_bezier_easing(x1, y1, x2, y2).unwrap();

        group.bench_with_input(BenchmarkId::new("current", name), &samples, |b, samples| {
            b.iter(|| {
                let mut sum = 0.0;
                for &x in samples {
                    sum += easing.sample(black_box(x));
                }
                black_box(sum)
            })
        });
        group.bench_with_input(
            BenchmarkId::new("legacy_boxed", name),
            &samples,
            |b, samples| {
                b.iter(|| {
                    let mut sum = 0.0;
                    for &x in samples {
                        sum += legacy(black_box(x));
                    }
                    black_box(sum)
                })
            },
        );
    }

    group.finish();
}

fn f64_samples() -> [f64; SAMPLE_COUNT] {
    let mut samples = [0.0; SAMPLE_COUNT];
    let denominator = (SAMPLE_COUNT - 1) as f64;

    for (index, sample) in samples.iter_mut().enumerate() {
        *sample = index as f64 / denominator;
    }

    samples
}

fn f32_samples() -> [f32; SAMPLE_COUNT] {
    let mut samples = [0.0; SAMPLE_COUNT];
    let denominator = (SAMPLE_COUNT - 1) as f32;

    for (index, sample) in samples.iter_mut().enumerate() {
        *sample = index as f32 / denominator;
    }

    samples
}

#[inline]
fn legacy_linear_easing<T: BezierFloat>(x: T) -> T {
    x
}

#[inline]
fn legacy_y<T: BezierFloat>(t: T, ay: T, by: T, cy: T) -> T {
    ((ay * t + T::THREE * by) * t + cy) * t
}

#[inline]
fn legacy_x2t<T: BezierFloat>(x: T, a: T, b: T, c: T, d: T) -> T {
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

fn legacy_bezier_easing<T: BezierFloat>(
    m_x1: T,
    m_y1: T,
    m_x2: T,
    m_y2: T,
) -> Result<LegacyBezierEasing<T>, &'static str> {
    if !(T::ZERO <= m_x1 && m_x1 <= T::ONE && T::ZERO <= m_x2 && m_x2 <= T::ONE) {
        return Err("bezier x values must be in [0, 1] range");
    }

    if m_x1 == m_y1 && m_x2 == m_y2 {
        return Ok(Box::new(legacy_linear_easing));
    }

    let a = T::SIX * (T::THREE * m_x1 - T::THREE * m_x2 + T::ONE);
    let b = T::SIX * (m_x2 - T::TWO * m_x1);
    let c = T::THREE * m_x1;

    let ay = T::THREE * m_y1 - T::THREE * m_y2 + T::ONE;
    let by = m_y2 - T::TWO * m_y1;
    let cy = T::THREE * m_y1;

    if a == T::ZERO {
        return Ok(Box::new(move |x| {
            if x == T::ZERO || x == T::ONE {
                return x;
            }
            legacy_y(x, ay, by, cy)
        }));
    }

    let a2 = a * a;
    let b2 = b * b;
    let d = b / a;
    let e = (T::THREE * b * c) / a2 - (b2 * b) / (a2 * a);
    let w1 = (T::TWO * c) / a - b2 / a2;
    let w = w1 * w1 * w1;
    let o = T::THREE / a;

    Ok(Box::new(move |x| {
        if x == T::ZERO || x == T::ONE {
            return x;
        }
        legacy_y(legacy_x2t(x, e, o, w, d), ay, by, cy)
    }))
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .without_plots()
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(200))
        .sample_size(10)
        .nresamples(1_000)
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets =
        bench_construct_f64,
        bench_construct_f32,
        bench_sample_single_f64,
        bench_sample_single_f32,
        bench_sample_batch_f64,
        bench_sample_batch_f32,
}
criterion_main!(benches);
