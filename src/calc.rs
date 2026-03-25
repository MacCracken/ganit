//! Calculus: differentiation, integration, interpolation, curves.
//!
//! Provides numerical differentiation, integration (trapezoidal and Simpson's),
//! linear interpolation, and Bezier curve evaluation.

use crate::HisabError;
use glam::{Vec2, Vec3};

/// Numerical derivative using the central difference method.
///
/// `f`: the function to differentiate.
/// `x`: the point at which to evaluate the derivative.
/// `h`: the step size (smaller = more accurate but risk of cancellation).
///
/// # Examples
///
/// ```
/// use hisab::calc::derivative;
///
/// // d/dx(x²) at x=3 ≈ 6
/// let d = derivative(|x| x * x, 3.0, 1e-7);
/// assert!((d - 6.0).abs() < 1e-5);
/// ```
#[must_use]
#[inline]
pub fn derivative(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Numerical integration using the trapezoidal rule.
///
/// Divides [a, b] into `n` sub-intervals.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the computed integral"]
#[inline]
pub fn integral_trapezoidal(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    Ok(sum * h)
}

/// Numerical integration using Simpson's rule.
///
/// `n` must be even. If odd, it is rounded up.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero (after rounding).
#[must_use = "returns the computed integral"]
#[inline]
pub fn integral_simpson(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, HisabError> {
    let n = if n % 2 == 1 { n + 1 } else { n };
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    // Process pairs: odd indices get coefficient 4, even get 2.
    // Unrolled to avoid branch per iteration.
    let mut i = 1;
    while i < n {
        sum += 4.0 * f(a + i as f64 * h);
        sum += 2.0 * f(a + (i + 1) as f64 * h);
        i += 2;
    }
    // Correct the last even-index term (we added 2*f(b) but f(b) is already counted)
    sum -= 2.0 * f(b);

    Ok(sum * h / 3.0)
}

/// Linear interpolation between two f64 values.
#[must_use]
#[inline]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Evaluate a quadratic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^2 * p0 + 2(1-t)t * p1 + t^2 * p2
#[must_use]
#[inline]
pub fn bezier_quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^3 * p0 + 3(1-t)^2*t * p1 + 3(1-t)*t^2 * p2 + t^3 * p3
#[must_use]
#[inline]
pub fn bezier_cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    let u2 = u * u;
    let t2 = t * t;
    p0 * (u2 * u) + p1 * (3.0 * u2 * t) + p2 * (3.0 * u * t2) + p3 * (t2 * t)
}

// ---------------------------------------------------------------------------
// 3D Bezier curves
// ---------------------------------------------------------------------------

/// Evaluate a quadratic Bezier curve in 3D at parameter `t` in [0, 1].
#[must_use]
#[inline]
pub fn bezier_quadratic_3d(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve in 3D at parameter `t` in [0, 1].
#[must_use]
#[inline]
pub fn bezier_cubic_3d(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    let u2 = u * u;
    let t2 = t * t;
    p0 * (u2 * u) + p1 * (3.0 * u2 * t) + p2 * (3.0 * u * t2) + p3 * (t2 * t)
}

// ---------------------------------------------------------------------------
// De Casteljau subdivision
// ---------------------------------------------------------------------------

/// Evaluate a cubic Bezier at `t` using the de Casteljau algorithm (2D).
///
/// Also returns the subdivision — the two sets of control points for the
/// left `[0, t]` and right `[t, 1]` sub-curves.
#[must_use]
#[inline]
pub fn de_casteljau_split(
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
    t: f32,
) -> (Vec2, [Vec2; 4], [Vec2; 4]) {
    let u = 1.0 - t;
    // Level 1
    let q0 = p0 * u + p1 * t;
    let q1 = p1 * u + p2 * t;
    let q2 = p2 * u + p3 * t;
    // Level 2
    let r0 = q0 * u + q1 * t;
    let r1 = q1 * u + q2 * t;
    // Level 3 (the point)
    let s = r0 * u + r1 * t;

    let left = [p0, q0, r0, s];
    let right = [s, r1, q2, p3];
    (s, left, right)
}

// ---------------------------------------------------------------------------
// Catmull-Rom splines
// ---------------------------------------------------------------------------

/// Evaluate a Catmull-Rom spline segment at parameter `t` in [0, 1].
///
/// Takes four control points: `p0` and `p3` are the tangent-influencing
/// neighbors, `p1` and `p2` are the interpolated segment endpoints.
/// The curve passes through `p1` at `t=0` and `p2` at `t=1`.
#[must_use]
#[inline]
pub fn catmull_rom(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t2 = t * t;
    let t3 = t2 * t;
    // Standard Catmull-Rom matrix form (alpha = 0.5)
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

// ---------------------------------------------------------------------------
// B-spline evaluation
// ---------------------------------------------------------------------------

/// Evaluate a uniform B-spline of arbitrary degree using de Boor's algorithm.
///
/// - `degree`: spline degree (1 = linear, 2 = quadratic, 3 = cubic).
/// - `control_points`: the control polygon.
/// - `knots`: the knot vector (length = control_points.len() + degree + 1).
/// - `t`: parameter value (must be within the valid knot range).
///
/// Returns `None` if inputs are invalid or `t` is out of range.
#[must_use = "returns the evaluated spline point"]
#[inline]
pub fn bspline_eval(degree: usize, control_points: &[Vec3], knots: &[f64], t: f64) -> Option<Vec3> {
    let n = control_points.len();
    if n == 0 || knots.len() != n + degree + 1 {
        return None;
    }

    // Find the knot span: largest k such that knots[k] <= t, clamped to valid range
    if t < knots[degree] || t > knots[n] {
        return None;
    }
    let mut k = degree;
    while k < n - 1 && knots[k + 1] <= t {
        k += 1;
    }

    // De Boor's algorithm — stack buffer for degree <= 4, heap otherwise
    let mut buf = [Vec3::ZERO; 5];
    let d: &mut [Vec3] = if degree < 5 {
        for j in 0..=degree {
            buf[j] = control_points[k - degree + j];
        }
        &mut buf[..=degree]
    } else {
        // Fallback for high degree (rare)
        let mut v: Vec<Vec3> = (0..=degree)
            .map(|j| control_points[k - degree + j])
            .collect();
        // High-degree fallback: use heap allocation and return early
        return {
            for r in 1..=degree {
                for j in (r..=degree).rev() {
                    let i = k - degree + j;
                    let denom = knots[i + degree + 1 - r] - knots[i];
                    if denom.abs() < crate::EPSILON_F64 {
                        continue;
                    }
                    let alpha = ((t - knots[i]) / denom) as f32;
                    v[j] = v[j - 1] * (1.0 - alpha) + v[j] * alpha;
                }
            }
            Some(v[degree])
        };
    };

    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = k - degree + j;
            let denom = knots[i + degree + 1 - r] - knots[i];
            if denom.abs() < crate::EPSILON_F64 {
                continue;
            }
            let alpha = ((t - knots[i]) / denom) as f32;
            d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
        }
    }

    Some(d[degree])
}

// ---------------------------------------------------------------------------
// Arc-length parameterization
// ---------------------------------------------------------------------------

/// Approximate the arc length of a cubic Bezier curve in 3D.
///
/// Uses `n` linear segments to approximate. Higher `n` = more accurate.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the computed arc length"]
#[inline]
pub fn bezier_cubic_3d_arc_length(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    n: usize,
) -> Result<f32, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let mut length = 0.0f32;
    let mut prev = p0;
    for i in 1..=n {
        let t = i as f32 / n as f32;
        let curr = bezier_cubic_3d(p0, p1, p2, p3, t);
        length += (curr - prev).length();
        prev = curr;
    }
    Ok(length)
}

/// Re-parameterize a cubic Bezier by arc length.
///
/// Given a normalized distance `s` in [0, 1] (where 0 = start, 1 = end),
/// returns the corresponding `t` parameter.
/// `n` controls the accuracy (number of linear segments for the lookup table).
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the parameter at the given arc length"]
#[inline]
pub fn bezier_cubic_3d_param_at_length(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    s: f32,
    n: usize,
) -> Result<f32, HisabError> {
    if s <= 0.0 {
        return Ok(0.0);
    }
    if s >= 1.0 {
        return Ok(1.0);
    }
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }

    // Build cumulative arc-length table (O(n))
    let mut table = Vec::with_capacity(n + 1);
    table.push(0.0f32);
    let mut prev = p0;
    for i in 1..=n {
        let t = i as f32 / n as f32;
        let curr = bezier_cubic_3d(p0, p1, p2, p3, t);
        let seg = (curr - prev).length();
        table.push(table[i - 1] + seg);
        prev = curr;
    }

    let total = table.last().copied().unwrap_or(0.0);
    let target = s * total;

    // Binary search the table for the segment containing the target length
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if table[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    // Linearly interpolate within the segment
    if lo == 0 {
        return Ok(0.0);
    }
    let seg_start = table[lo - 1];
    let seg_end = table[lo];
    let seg_len = seg_end - seg_start;
    let frac = if seg_len > crate::EPSILON_F32 {
        (target - seg_start) / seg_len
    } else {
        0.0
    };

    Ok(((lo - 1) as f32 + frac) / n as f32)
}

// ---------------------------------------------------------------------------
// Gauss-Legendre quadrature
// ---------------------------------------------------------------------------

/// Numerical integration using 5-point Gauss-Legendre quadrature.
///
/// More accurate than Simpson's for smooth functions with fewer evaluations.
/// Integrates `f` over `[a, b]`.
#[must_use]
#[inline]
pub fn integral_gauss_legendre_5(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    // 5-point GL nodes and weights on [-1, 1]
    const NODES: [f64; 5] = [
        -0.906179845938664,
        -0.538469310105683,
        0.0,
        0.538469310105683,
        0.906179845938664,
    ];
    const WEIGHTS: [f64; 5] = [
        0.236926885056189,
        0.478628670499366,
        0.568888888888889,
        0.478628670499366,
        0.236926885056189,
    ];

    let half = (b - a) * 0.5;
    let mid = (a + b) * 0.5;
    let sum = WEIGHTS[0] * f(mid + half * NODES[0])
        + WEIGHTS[1] * f(mid + half * NODES[1])
        + WEIGHTS[2] * f(mid + half * NODES[2])
        + WEIGHTS[3] * f(mid + half * NODES[3])
        + WEIGHTS[4] * f(mid + half * NODES[4]);
    sum * half
}

/// Composite Gauss-Legendre quadrature (5-point) over `n` sub-intervals.
///
/// Divides `[a, b]` into `n` panels and applies 5-point GL to each.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the computed integral"]
#[inline]
pub fn integral_gauss_legendre(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let h = (b - a) / n as f64;
    let mut total = 0.0;
    for i in 0..n {
        let lo = a + i as f64 * h;
        let hi = lo + h;
        total += integral_gauss_legendre_5(&f, lo, hi);
    }
    Ok(total)
}

// ---------------------------------------------------------------------------
// Easing functions
// ---------------------------------------------------------------------------

/// Ease-in (quadratic): slow start, fast end.
#[must_use]
#[inline]
pub fn ease_in(t: f32) -> f32 {
    t * t
}

/// Ease-out (quadratic): fast start, slow end.
#[must_use]
#[inline]
pub fn ease_out(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Ease-in-out (cubic smoothstep): slow start, fast middle, slow end.
#[must_use]
#[inline]
pub fn ease_in_out(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Ease-in (cubic): slower start than quadratic.
#[must_use]
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease-out (cubic): slower end than quadratic.
#[must_use]
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let u = 1.0 - t;
    1.0 - u * u * u
}

/// Ease-in-out (quintic smootherstep): C2 continuous, zero first and second derivatives at endpoints.
#[must_use]
#[inline]
pub fn ease_in_out_smooth(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

// ---------------------------------------------------------------------------
// Noise functions
// ---------------------------------------------------------------------------

/// Permutation table for noise functions (256 entries, doubled for wrapping).
const PERM: [u8; 512] = {
    let base: [u8; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
    ];
    let mut table = [0u8; 512];
    let mut i = 0;
    while i < 512 {
        table[i] = base[i & 255];
        i += 1;
    }
    table
};

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn grad2(hash: u8, x: f64, y: f64) -> f64 {
    match hash & 3 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        _ => -x - y,
    }
}

#[inline]
fn grad3(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    match hash & 15 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x + z,
        5 => -x + z,
        6 => x - z,
        7 => -x - z,
        8 => y + z,
        9 => -y + z,
        10 => y - z,
        11 => -y - z,
        12 => y + x,
        13 => -y + z,
        14 => y - x,
        _ => -y - z,
    }
}

/// 2D Perlin noise. Returns a value in approximately [-1, 1].
#[must_use]
#[inline]
pub fn perlin_2d(x: f64, y: f64) -> f64 {
    let xi = x.floor() as i32 as usize & 255;
    let yi = y.floor() as i32 as usize & 255;
    let xf = x - x.floor();
    let yf = y - y.floor();
    let u = fade(xf);
    let v = fade(yf);

    let aa = PERM[PERM[xi] as usize + yi];
    let ab = PERM[PERM[xi] as usize + yi + 1];
    let ba = PERM[PERM[xi + 1] as usize + yi];
    let bb = PERM[PERM[xi + 1] as usize + yi + 1];

    let x1 = lerp(grad2(aa, xf, yf), grad2(ba, xf - 1.0, yf), u);
    let x2 = lerp(grad2(ab, xf, yf - 1.0), grad2(bb, xf - 1.0, yf - 1.0), u);
    lerp(x1, x2, v)
}

/// 3D Perlin noise. Returns a value in approximately [-1, 1].
#[must_use]
#[inline]
pub fn perlin_3d(x: f64, y: f64, z: f64) -> f64 {
    let xi = x.floor() as i32 as usize & 255;
    let yi = y.floor() as i32 as usize & 255;
    let zi = z.floor() as i32 as usize & 255;
    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();
    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let aaa = PERM[PERM[PERM[xi] as usize + yi] as usize + zi];
    let aba = PERM[PERM[PERM[xi] as usize + yi + 1] as usize + zi];
    let aab = PERM[PERM[PERM[xi] as usize + yi] as usize + zi + 1];
    let abb = PERM[PERM[PERM[xi] as usize + yi + 1] as usize + zi + 1];
    let baa = PERM[PERM[PERM[xi + 1] as usize + yi] as usize + zi];
    let bba = PERM[PERM[PERM[xi + 1] as usize + yi + 1] as usize + zi];
    let bab = PERM[PERM[PERM[xi + 1] as usize + yi] as usize + zi + 1];
    let bbb = PERM[PERM[PERM[xi + 1] as usize + yi + 1] as usize + zi + 1];

    let x1 = lerp(grad3(aaa, xf, yf, zf), grad3(baa, xf - 1.0, yf, zf), u);
    let x2 = lerp(
        grad3(aba, xf, yf - 1.0, zf),
        grad3(bba, xf - 1.0, yf - 1.0, zf),
        u,
    );
    let y1 = lerp(x1, x2, v);

    let x1 = lerp(
        grad3(aab, xf, yf, zf - 1.0),
        grad3(bab, xf - 1.0, yf, zf - 1.0),
        u,
    );
    let x2 = lerp(
        grad3(abb, xf, yf - 1.0, zf - 1.0),
        grad3(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
        u,
    );
    let y2 = lerp(x1, x2, v);

    lerp(y1, y2, w)
}

/// Fractal Brownian Motion — layered noise with octaves.
///
/// - `noise_fn`: base noise function (e.g., `perlin_2d` wrapped to take `(x, y)`).
/// - `x`, `y`: coordinates.
/// - `octaves`: number of noise layers (typically 4-8).
/// - `lacunarity`: frequency multiplier per octave (typically 2.0).
/// - `gain`: amplitude multiplier per octave (typically 0.5).
#[must_use]
#[inline]
pub fn fbm_2d(
    noise_fn: impl Fn(f64, f64) -> f64,
    x: f64,
    y: f64,
    octaves: usize,
    lacunarity: f64,
    gain: f64,
) -> f64 {
    let mut sum = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..octaves {
        sum += amplitude * noise_fn(x * frequency, y * frequency);
        max_amplitude += amplitude;
        amplitude *= gain;
        frequency *= lacunarity;
    }

    sum / max_amplitude
}

// ---------------------------------------------------------------------------
// Spring dynamics
// ---------------------------------------------------------------------------

/// Critically damped spring step (analytical solution).
///
/// Moves `current` toward `target` with smooth deceleration.
/// Returns `(new_position, new_velocity)`.
///
/// - `current`: current value.
/// - `target`: target value.
/// - `velocity`: current velocity (modified each step).
/// - `stiffness`: spring stiffness (ω² where ω is natural frequency). Typical: 100–500.
/// - `damping`: damping ratio. Use `2.0 * stiffness.sqrt()` for critical damping.
/// - `dt`: time step.
#[must_use]
#[inline]
pub fn spring_step(
    current: f64,
    target: f64,
    velocity: f64,
    stiffness: f64,
    damping: f64,
    dt: f64,
) -> (f64, f64) {
    let omega = stiffness.sqrt();
    let zeta = damping / (2.0 * omega);
    let x = current - target;

    if (zeta - 1.0).abs() < 1e-6 {
        // Critically damped: (c1 + c2*t) * e^(-ω*t)
        let exp = (-omega * dt).exp();
        let c1 = x;
        let c2 = velocity + omega * x;
        let new_x = (c1 + c2 * dt) * exp;
        let new_v = (c2 - omega * (c1 + c2 * dt)) * exp;
        (target + new_x, new_v)
    } else if zeta < 1.0 {
        // Underdamped
        let omega_d = omega * (1.0 - zeta * zeta).sqrt();
        let exp = (-zeta * omega * dt).exp();
        let cos = (omega_d * dt).cos();
        let sin = (omega_d * dt).sin();
        let new_x = exp * (x * cos + ((velocity + zeta * omega * x) / omega_d) * sin);
        let new_v = exp * ((velocity + zeta * omega * x) * cos - x * omega_d * sin)
            - zeta * omega * exp * (x * cos + ((velocity + zeta * omega * x) / omega_d) * sin);
        (target + new_x, new_v)
    } else {
        // Overdamped
        let s1 = -omega * (zeta - (zeta * zeta - 1.0).sqrt());
        let s2 = -omega * (zeta + (zeta * zeta - 1.0).sqrt());
        let c2 = (velocity - s1 * x) / (s2 - s1);
        let c1 = x - c2;
        let e1 = (s1 * dt).exp();
        let e2 = (s2 * dt).exp();
        let new_x = c1 * e1 + c2 * e2;
        let new_v = c1 * s1 * e1 + c2 * s2 * e2;
        (target + new_x, new_v)
    }
}

// ---------------------------------------------------------------------------
// CSS cubic-bezier easing
// ---------------------------------------------------------------------------

/// CSS `cubic-bezier(x1, y1, x2, y2)` timing function.
///
/// Given control points `(0,0)-(x1,y1)-(x2,y2)-(1,1)`, returns the eased
/// value `y` for input progress `t` in [0, 1].
///
/// Uses Newton-Raphson to solve the x(t) curve, then evaluates y(t).
#[must_use]
#[inline]
pub fn cubic_bezier_ease(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    // Find parameter u such that bezier_x(u) = t using Newton-Raphson
    let bezier_x = |u: f32| -> f32 {
        let iu = 1.0 - u;
        3.0 * iu * iu * u * x1 + 3.0 * iu * u * u * x2 + u * u * u
    };
    let bezier_dx = |u: f32| -> f32 {
        let iu = 1.0 - u;
        3.0 * iu * iu * x1 + 6.0 * iu * u * (x2 - x1) + 3.0 * u * u * (1.0 - x2)
    };

    let mut u = t; // Initial guess
    for _ in 0..8 {
        let dx = bezier_dx(u);
        if dx.abs() < 1e-7 {
            break;
        }
        u -= (bezier_x(u) - t) / dx;
        u = u.clamp(0.0, 1.0);
    }

    // Evaluate y at the found parameter
    let iu = 1.0 - u;
    3.0 * iu * iu * u * y1 + 3.0 * iu * u * u * y2 + u * u * u
}

// ---------------------------------------------------------------------------
// Advanced integration
// ---------------------------------------------------------------------------

/// Adaptive Simpson's rule for numerical integration.
///
/// Recursively subdivides `[a, b]` until the error estimate is below `tol`.
/// Uses Richardson extrapolation to estimate the error.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInterval`] if `a >= b`.
/// Returns [`HisabError::NoConvergence`] if maximum recursion depth (50) is exceeded.
#[must_use = "returns the computed integral or an error"]
pub fn integral_adaptive_simpson(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
) -> Result<f64, HisabError> {
    if a >= b {
        return Err(HisabError::InvalidInterval);
    }
    let fa = f(a);
    let fb = f(b);
    let mid = (a + b) * 0.5;
    let fmid = f(mid);
    let whole = (b - a) / 6.0 * (fa + 4.0 * fmid + fb);
    adaptive_simpson_recursive(&f, a, b, fa, fb, fmid, whole, tol, 50)
}

#[allow(clippy::too_many_arguments)]
fn adaptive_simpson_recursive(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    fa: f64,
    fb: f64,
    fmid: f64,
    whole: f64,
    tol: f64,
    depth: usize,
) -> Result<f64, HisabError> {
    if depth == 0 {
        return Err(HisabError::NoConvergence(50));
    }

    let mid = (a + b) * 0.5;
    let left_mid = (a + mid) * 0.5;
    let right_mid = (mid + b) * 0.5;
    let flm = f(left_mid);
    let frm = f(right_mid);

    let h = (b - a) * 0.5;
    let left = h / 6.0 * (fa + 4.0 * flm + fmid);
    let right = h / 6.0 * (fmid + 4.0 * frm + fb);
    let refined = left + right;

    // Richardson extrapolation error estimate
    let error = (refined - whole) / 15.0;

    if error.abs() < tol {
        return Ok(refined + error);
    }

    let left_result =
        adaptive_simpson_recursive(f, a, mid, fa, fmid, flm, left, tol * 0.5, depth - 1)?;
    let right_result =
        adaptive_simpson_recursive(f, mid, b, fmid, fb, frm, right, tol * 0.5, depth - 1)?;
    Ok(left_result + right_result)
}

/// Monte Carlo integration over an N-dimensional hyperrectangle.
///
/// Estimates `∫∫...∫ f(x) dx` over the region defined by `bounds`,
/// where `bounds[i] = (lower_i, upper_i)`.
///
/// Uses a simple pseudo-random LCG for deterministic reproducibility
/// (seeded from dimension count and sample count).
///
/// Returns `(estimate, standard_error)`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `bounds` is empty or `n_samples` is zero.
/// Returns [`HisabError::InvalidInterval`] if any bound has `lower >= upper`.
#[must_use = "returns the estimated integral and standard error or an error"]
pub fn integral_monte_carlo(
    f: impl Fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    n_samples: usize,
) -> Result<(f64, f64), HisabError> {
    let dim = bounds.len();
    if dim == 0 {
        return Err(HisabError::InvalidInput("empty bounds".into()));
    }
    if n_samples == 0 {
        return Err(HisabError::ZeroSteps);
    }

    // Compute volume of the hyperrectangle
    let mut volume = 1.0;
    for &(lo, hi) in bounds {
        if lo >= hi {
            return Err(HisabError::InvalidInterval);
        }
        volume *= hi - lo;
    }

    // Simple LCG for deterministic pseudo-random numbers
    let mut rng_state: u64 = 6364136223846793005_u64
        .wrapping_mul(dim as u64)
        .wrapping_add(n_samples as u64);
    let lcg_next = |state: &mut u64| -> f64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        // Map to [0, 1)
        (*state >> 11) as f64 / (1u64 << 53) as f64
    };

    let mut x = vec![0.0; dim];
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..n_samples {
        for (d, &(lo, hi)) in bounds.iter().enumerate() {
            x[d] = lo + (hi - lo) * lcg_next(&mut rng_state);
        }
        let val = f(&x);
        sum += val;
        sum_sq += val * val;
    }

    let n = n_samples as f64;
    let mean = sum / n;
    let variance = (sum_sq / n - mean * mean).max(0.0);
    let std_error = (variance / n).sqrt() * volume;
    let estimate = mean * volume;

    Ok((estimate, std_error))
}

// ---------------------------------------------------------------------------
// Multivariable calculus
// ---------------------------------------------------------------------------

/// Partial derivative of a multivariate function with respect to variable `var`.
///
/// Uses central difference: `∂f/∂x_var ≈ (f(x + h·e_var) - f(x - h·e_var)) / (2h)`.
///
/// - `f`: function from ℝⁿ → ℝ.
/// - `x`: point at which to evaluate.
/// - `var`: index of the variable to differentiate with respect to.
/// - `h`: step size.
///
/// # Errors
///
/// Returns [`HisabError::OutOfRange`] if `var >= x.len()`.
#[must_use = "returns the computed partial derivative or an error"]
#[inline]
pub fn partial_derivative(
    f: impl Fn(&[f64]) -> f64,
    x: &[f64],
    var: usize,
    h: f64,
) -> Result<f64, HisabError> {
    if var >= x.len() {
        return Err(HisabError::OutOfRange(format!(
            "var index {var} >= dimension {}",
            x.len()
        )));
    }
    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();
    x_plus[var] += h;
    x_minus[var] -= h;
    Ok((f(&x_plus) - f(&x_minus)) / (2.0 * h))
}

/// Gradient of a scalar function f: ℝⁿ → ℝ.
///
/// Returns the vector of partial derivatives `[∂f/∂x₀, ∂f/∂x₁, ...]`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x` is empty.
#[must_use = "returns the gradient vector or an error"]
pub fn gradient(f: impl Fn(&[f64]) -> f64, x: &[f64], h: f64) -> Result<Vec<f64>, HisabError> {
    let n = x.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty input".into()));
    }
    let mut grad = Vec::with_capacity(n);
    let mut x_buf = x.to_vec();
    for i in 0..n {
        x_buf[i] = x[i] + h;
        let f_plus = f(&x_buf);
        x_buf[i] = x[i] - h;
        let f_minus = f(&x_buf);
        x_buf[i] = x[i]; // restore
        grad.push((f_plus - f_minus) / (2.0 * h));
    }
    Ok(grad)
}

/// Jacobian matrix of a vector function f: ℝⁿ → ℝᵐ.
///
/// Returns an `m × n` matrix (row-major) where `J[i][j] = ∂fᵢ/∂xⱼ`.
///
/// - `fs`: vector of scalar functions, each ℝⁿ → ℝ.
/// - `x`: point at which to evaluate.
/// - `h`: step size for finite differences.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `fs` or `x` is empty.
#[must_use = "returns the Jacobian matrix or an error"]
#[allow(clippy::type_complexity)]
pub fn jacobian(
    fs: &[&dyn Fn(&[f64]) -> f64],
    x: &[f64],
    h: f64,
) -> Result<Vec<Vec<f64>>, HisabError> {
    let m = fs.len();
    let n = x.len();
    if m == 0 || n == 0 {
        return Err(HisabError::InvalidInput("empty input".into()));
    }

    let mut jac = vec![vec![0.0; n]; m];
    let mut x_buf = x.to_vec();

    for j in 0..n {
        x_buf[j] = x[j] + h;
        let f_plus: Vec<f64> = fs.iter().map(|fi| fi(&x_buf)).collect();
        x_buf[j] = x[j] - h;
        let f_minus: Vec<f64> = fs.iter().map(|fi| fi(&x_buf)).collect();
        x_buf[j] = x[j]; // restore
        let inv_2h = 1.0 / (2.0 * h);
        for i in 0..m {
            jac[i][j] = (f_plus[i] - f_minus[i]) * inv_2h;
        }
    }

    Ok(jac)
}

/// Hessian matrix of a scalar function f: ℝⁿ → ℝ.
///
/// Returns an `n × n` symmetric matrix where `H[i][j] = ∂²f/∂xᵢ∂xⱼ`.
///
/// Uses second-order central differences.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x` is empty.
#[must_use = "returns the Hessian matrix or an error"]
pub fn hessian(f: impl Fn(&[f64]) -> f64, x: &[f64], h: f64) -> Result<Vec<Vec<f64>>, HisabError> {
    let n = x.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty input".into()));
    }

    let mut hess = vec![vec![0.0; n]; n];
    let mut x_buf = x.to_vec();
    let f0 = f(x);
    let h2 = h * h;

    // Diagonal: ∂²f/∂xᵢ² ≈ (f(x+heᵢ) - 2f(x) + f(x-heᵢ)) / h²
    for i in 0..n {
        x_buf[i] = x[i] + h;
        let fp = f(&x_buf);
        x_buf[i] = x[i] - h;
        let fm = f(&x_buf);
        x_buf[i] = x[i];
        hess[i][i] = (fp - 2.0 * f0 + fm) / h2;
    }

    // Off-diagonal: ∂²f/∂xᵢ∂xⱼ ≈ (f(x+heᵢ+heⱼ) - f(x+heᵢ-heⱼ) - f(x-heᵢ+heⱼ) + f(x-heᵢ-heⱼ)) / (4h²)
    let inv_4h2 = 1.0 / (4.0 * h2);
    for i in 0..n {
        for j in (i + 1)..n {
            x_buf[i] = x[i] + h;
            x_buf[j] = x[j] + h;
            let fpp = f(&x_buf);
            x_buf[j] = x[j] - h;
            let fpm = f(&x_buf);
            x_buf[i] = x[i] - h;
            let fmm = f(&x_buf);
            x_buf[j] = x[j] + h;
            let fmp = f(&x_buf);

            // Restore
            x_buf[i] = x[i];
            x_buf[j] = x[j];

            let val = (fpp - fpm - fmp + fmm) * inv_4h2;
            hess[i][j] = val;
            hess[j][i] = val;
        }
    }

    Ok(hess)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HisabError;

    const EPSILON_F64: f64 = 1e-6;
    const EPSILON_F32: f32 = 1e-4;

    fn approx_eq_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON_F64
    }

    fn approx_eq_f32(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON_F32
    }

    #[test]
    fn derivative_of_x_squared() {
        let d = derivative(|x| x * x, 3.0, 1e-7);
        assert!(approx_eq_f64(d, 6.0));
    }

    #[test]
    fn derivative_of_sin() {
        let d = derivative(f64::sin, 0.0, 1e-7);
        assert!(approx_eq_f64(d, 1.0));
    }

    #[test]
    fn derivative_of_exp() {
        let d = derivative(f64::exp, 1.0, 1e-7);
        assert!((d - std::f64::consts::E).abs() < 1e-5);
    }

    #[test]
    fn integral_trapezoidal_constant() {
        let result = integral_trapezoidal(|_| 5.0, 0.0, 2.0, 100).unwrap();
        assert!(approx_eq_f64(result, 10.0));
    }

    #[test]
    fn integral_trapezoidal_linear() {
        let result = integral_trapezoidal(|x| x, 0.0, 4.0, 1000).unwrap();
        assert!((result - 8.0).abs() < 1e-4);
    }

    #[test]
    fn integral_trapezoidal_quadratic() {
        let result = integral_trapezoidal(|x| x * x, 0.0, 3.0, 10000).unwrap();
        assert!((result - 9.0).abs() < 1e-3);
    }

    #[test]
    fn integral_simpson_quadratic() {
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 4).unwrap();
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn integral_simpson_cubic() {
        let result = integral_simpson(|x| x * x * x, 0.0, 2.0, 4).unwrap();
        assert!(approx_eq_f64(result, 4.0));
    }

    #[test]
    fn integral_simpson_sin() {
        let result = integral_simpson(f64::sin, 0.0, std::f64::consts::PI, 100).unwrap();
        assert!((result - 2.0).abs() < 1e-6);
    }

    #[test]
    fn lerp_endpoints() {
        assert!(approx_eq_f64(lerp(0.0, 10.0, 0.0), 0.0));
        assert!(approx_eq_f64(lerp(0.0, 10.0, 1.0), 10.0));
        assert!(approx_eq_f64(lerp(0.0, 10.0, 0.5), 5.0));
    }

    #[test]
    fn bezier_quadratic_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::new(1.0, 0.0);

        let start = bezier_quadratic(p0, p1, p2, 0.0);
        let end = bezier_quadratic(p0, p1, p2, 1.0);

        assert!(approx_eq_f32(start.x, 0.0) && approx_eq_f32(start.y, 0.0));
        assert!(approx_eq_f32(end.x, 1.0) && approx_eq_f32(end.y, 0.0));
    }

    #[test]
    fn bezier_cubic_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.25, 1.0);
        let p2 = Vec2::new(0.75, 1.0);
        let p3 = Vec2::new(1.0, 0.0);

        let start = bezier_cubic(p0, p1, p2, p3, 0.0);
        let end = bezier_cubic(p0, p1, p2, p3, 1.0);

        assert!(approx_eq_f32(start.x, 0.0) && approx_eq_f32(start.y, 0.0));
        assert!(approx_eq_f32(end.x, 1.0) && approx_eq_f32(end.y, 0.0));
    }

    #[test]
    fn bezier_quadratic_midpoint() {
        let mid = bezier_quadratic(Vec2::ZERO, Vec2::new(0.5, 0.5), Vec2::ONE, 0.5);
        assert!(approx_eq_f32(mid.x, 0.5));
        assert!(approx_eq_f32(mid.y, 0.5));
    }

    #[test]
    fn integral_simpson_odd_n_rounds_up() {
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 3).unwrap();
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn derivative_of_constant() {
        let d = derivative(|_| 5.0, 3.0, 1e-7);
        assert!(approx_eq_f64(d, 0.0));
    }

    #[test]
    fn derivative_of_cubic() {
        let d = derivative(|x| x * x * x, 2.0, 1e-7);
        assert!((d - 12.0).abs() < 1e-4);
    }

    #[test]
    fn derivative_of_cos() {
        let d = derivative(f64::cos, std::f64::consts::FRAC_PI_2, 1e-7);
        assert!((d - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn integral_trapezoidal_sin() {
        let result = integral_trapezoidal(f64::sin, 0.0, std::f64::consts::PI, 10000).unwrap();
        assert!((result - 2.0).abs() < 1e-4);
    }

    #[test]
    fn integral_simpson_constant() {
        let result = integral_simpson(|_| 7.0, 1.0, 4.0, 4).unwrap();
        assert!(approx_eq_f64(result, 21.0));
    }

    #[test]
    fn integral_simpson_linear() {
        let result = integral_simpson(|x| 2.0 * x, 0.0, 3.0, 2).unwrap();
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn lerp_at_quarter() {
        assert!(approx_eq_f64(lerp(0.0, 100.0, 0.25), 25.0));
        assert!(approx_eq_f64(lerp(0.0, 100.0, 0.75), 75.0));
    }

    #[test]
    fn lerp_negative_range() {
        assert!(approx_eq_f64(lerp(-10.0, -20.0, 0.5), -15.0));
    }

    #[test]
    fn lerp_extrapolation() {
        assert!(approx_eq_f64(lerp(0.0, 10.0, 2.0), 20.0));
        assert!(approx_eq_f64(lerp(0.0, 10.0, -1.0), -10.0));
    }

    #[test]
    fn bezier_quadratic_straight_line() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 0.5);
        let p2 = Vec2::ONE;
        let quarter = bezier_quadratic(p0, p1, p2, 0.25);
        assert!(approx_eq_f32(quarter.x, 0.25));
        assert!(approx_eq_f32(quarter.y, 0.25));
    }

    #[test]
    fn bezier_cubic_midpoint() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 0.0);
        let p3 = Vec2::ONE;
        let mid = bezier_cubic(p0, p1, p2, p3, 0.5);
        assert!(approx_eq_f32(mid.x, 0.5));
        assert!(approx_eq_f32(mid.y, 0.5));
    }

    #[test]
    fn bezier_cubic_straight_line() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 1.0);
        let p2 = Vec2::new(2.0, 2.0);
        let p3 = Vec2::new(3.0, 3.0);
        let mid = bezier_cubic(p0, p1, p2, p3, 0.5);
        assert!(approx_eq_f32(mid.x, 1.5));
        assert!(approx_eq_f32(mid.y, 1.5));
    }

    #[test]
    fn integral_trapezoidal_single_step() {
        let result = integral_trapezoidal(|x| x, 0.0, 2.0, 1).unwrap();
        assert!(approx_eq_f64(result, 2.0));
    }

    #[test]
    fn integral_simpson_exp() {
        let expected = std::f64::consts::E - 1.0;
        let result = integral_simpson(f64::exp, 0.0, 1.0, 100).unwrap();
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn calc_error_display() {
        assert_eq!(
            HisabError::InvalidInterval.to_string(),
            "invalid interval: a must be less than b"
        );
        assert_eq!(
            HisabError::ZeroSteps.to_string(),
            "step count must be positive"
        );
    }

    // --- V0.3 tests ---

    fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq_f32(a.x, b.x) && approx_eq_f32(a.y, b.y) && approx_eq_f32(a.z, b.z)
    }

    // 3D Bezier
    #[test]
    fn bezier_quadratic_3d_endpoints() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(0.5, 1.0, 0.5);
        let p2 = Vec3::ONE;
        assert!(vec3_approx_eq(bezier_quadratic_3d(p0, p1, p2, 0.0), p0));
        assert!(vec3_approx_eq(bezier_quadratic_3d(p0, p1, p2, 1.0), p2));
    }

    #[test]
    fn bezier_cubic_3d_endpoints() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 2.0, 0.0);
        let p2 = Vec3::new(2.0, 2.0, 1.0);
        let p3 = Vec3::new(3.0, 0.0, 1.0);
        assert!(vec3_approx_eq(bezier_cubic_3d(p0, p1, p2, p3, 0.0), p0));
        assert!(vec3_approx_eq(bezier_cubic_3d(p0, p1, p2, p3, 1.0), p3));
    }

    #[test]
    fn bezier_cubic_3d_straight_line() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::ONE;
        let p2 = Vec3::splat(2.0);
        let p3 = Vec3::splat(3.0);
        let mid = bezier_cubic_3d(p0, p1, p2, p3, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::splat(1.5)));
    }

    // De Casteljau
    #[test]
    fn de_casteljau_matches_direct() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.25, 1.0);
        let p2 = Vec2::new(0.75, 1.0);
        let p3 = Vec2::ONE;
        let (pt, _left, _right) = de_casteljau_split(p0, p1, p2, p3, 0.5);
        let direct = bezier_cubic(p0, p1, p2, p3, 0.5);
        assert!(approx_eq_f32(pt.x, direct.x));
        assert!(approx_eq_f32(pt.y, direct.y));
    }

    #[test]
    fn de_casteljau_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 2.0);
        let p2 = Vec2::new(3.0, 2.0);
        let p3 = Vec2::new(4.0, 0.0);
        let (pt0, _, _) = de_casteljau_split(p0, p1, p2, p3, 0.0);
        let (pt1, _, _) = de_casteljau_split(p0, p1, p2, p3, 1.0);
        assert!(approx_eq_f32(pt0.x, p0.x) && approx_eq_f32(pt0.y, p0.y));
        assert!(approx_eq_f32(pt1.x, p3.x) && approx_eq_f32(pt1.y, p3.y));
    }

    #[test]
    fn de_casteljau_left_right_rejoin() {
        // Evaluating the left sub-curve at t=1 and right at t=0 should give the split point
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 2.0);
        let p2 = Vec2::new(3.0, 2.0);
        let p3 = Vec2::new(4.0, 0.0);
        let (split_pt, left, right) = de_casteljau_split(p0, p1, p2, p3, 0.3);
        let left_end = bezier_cubic(left[0], left[1], left[2], left[3], 1.0);
        let right_start = bezier_cubic(right[0], right[1], right[2], right[3], 0.0);
        assert!(approx_eq_f32(left_end.x, split_pt.x));
        assert!(approx_eq_f32(right_start.x, split_pt.x));
    }

    // Catmull-Rom
    #[test]
    fn catmull_rom_passes_through_endpoints() {
        let p0 = Vec3::new(-1.0, 0.0, 0.0);
        let p1 = Vec3::ZERO;
        let p2 = Vec3::new(1.0, 1.0, 0.0);
        let p3 = Vec3::new(2.0, 1.0, 0.0);
        let at_0 = catmull_rom(p0, p1, p2, p3, 0.0);
        let at_1 = catmull_rom(p0, p1, p2, p3, 1.0);
        assert!(vec3_approx_eq(at_0, p1));
        assert!(vec3_approx_eq(at_1, p2));
    }

    #[test]
    fn catmull_rom_straight_line() {
        // Equally spaced collinear points -> straight line
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(2.0, 0.0, 0.0);
        let p3 = Vec3::new(3.0, 0.0, 0.0);
        let mid = catmull_rom(p0, p1, p2, p3, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::new(1.5, 0.0, 0.0)));
    }

    // B-spline
    #[test]
    fn bspline_linear_interpolation() {
        // Degree 1 (linear), 2 control points
        let pts = [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)];
        let knots = [0.0, 0.0, 1.0, 1.0]; // degree+1 repeated at each end
        let mid = bspline_eval(1, &pts, &knots, 0.5).unwrap();
        assert!(vec3_approx_eq(mid, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn bspline_cubic_endpoints() {
        // Degree 3, 4 control points, clamped knot vector
        let pts = [
            Vec3::ZERO,
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let start = bspline_eval(3, &pts, &knots, 0.0).unwrap();
        let end = bspline_eval(3, &pts, &knots, 1.0).unwrap();
        assert!(vec3_approx_eq(start, pts[0]));
        assert!(vec3_approx_eq(end, pts[3]));
    }

    #[test]
    fn bspline_invalid_knots() {
        let pts = [Vec3::ZERO, Vec3::X];
        let bad_knots = [0.0, 1.0]; // Wrong length
        assert!(bspline_eval(1, &pts, &bad_knots, 0.5).is_none());
    }

    // Arc length
    #[test]
    fn bezier_arc_length_straight_line() {
        // Straight line from origin to (10,0,0) -> length = 10
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(10.0 / 3.0, 0.0, 0.0);
        let p2 = Vec3::new(20.0 / 3.0, 0.0, 0.0);
        let p3 = Vec3::new(10.0, 0.0, 0.0);
        let len = bezier_cubic_3d_arc_length(p0, p1, p2, p3, 100).unwrap();
        assert!((len - 10.0).abs() < 0.01);
    }

    // Gauss-Legendre
    #[test]
    fn gauss_legendre_5_constant() {
        let result = integral_gauss_legendre_5(|_| 3.0, 0.0, 5.0);
        assert!(approx_eq_f64(result, 15.0));
    }

    #[test]
    fn gauss_legendre_5_quadratic() {
        // GL5 is exact for polynomials up to degree 9
        let result = integral_gauss_legendre_5(|x| x * x, 0.0, 3.0);
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn gauss_legendre_composite_sin() {
        let result = integral_gauss_legendre(f64::sin, 0.0, std::f64::consts::PI, 10).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn gauss_legendre_vs_simpson() {
        // GL with 2 panels should beat Simpson with 100 panels for smooth functions
        let gl = integral_gauss_legendre(f64::exp, 0.0, 1.0, 2).unwrap();
        let simp = integral_simpson(f64::exp, 0.0, 1.0, 100).unwrap();
        let exact = std::f64::consts::E - 1.0;
        assert!((gl - exact).abs() <= (simp - exact).abs());
    }

    // Easing functions
    #[test]
    fn ease_in_endpoints() {
        assert!(approx_eq_f32(ease_in(0.0), 0.0));
        assert!(approx_eq_f32(ease_in(1.0), 1.0));
    }

    #[test]
    fn ease_out_endpoints() {
        assert!(approx_eq_f32(ease_out(0.0), 0.0));
        assert!(approx_eq_f32(ease_out(1.0), 1.0));
    }

    #[test]
    fn ease_in_out_endpoints_and_midpoint() {
        assert!(approx_eq_f32(ease_in_out(0.0), 0.0));
        assert!(approx_eq_f32(ease_in_out(1.0), 1.0));
        assert!(approx_eq_f32(ease_in_out(0.5), 0.5));
    }

    #[test]
    fn ease_in_cubic_slower_than_quadratic() {
        // At t=0.5, cubic ease should be slower (lower value) than quadratic
        assert!(ease_in_cubic(0.5) < ease_in(0.5));
    }

    #[test]
    fn ease_out_cubic_endpoints() {
        assert!(approx_eq_f32(ease_out_cubic(0.0), 0.0));
        assert!(approx_eq_f32(ease_out_cubic(1.0), 1.0));
    }

    #[test]
    fn ease_in_out_smooth_endpoints() {
        assert!(approx_eq_f32(ease_in_out_smooth(0.0), 0.0));
        assert!(approx_eq_f32(ease_in_out_smooth(1.0), 1.0));
        assert!(approx_eq_f32(ease_in_out_smooth(0.5), 0.5));
    }

    #[test]
    fn ease_in_monotonic() {
        let mut prev = 0.0f32;
        for i in 1..=10 {
            let t = i as f32 / 10.0;
            let v = ease_in(t);
            assert!(v >= prev);
            prev = v;
        }
    }

    #[test]
    fn ease_out_monotonic() {
        let mut prev = 0.0f32;
        for i in 1..=10 {
            let t = i as f32 / 10.0;
            let v = ease_out(t);
            assert!(v >= prev);
            prev = v;
        }
    }

    // --- Audit tests ---

    #[test]
    fn param_at_length_endpoints() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(10.0 / 3.0, 0.0, 0.0);
        let p2 = Vec3::new(20.0 / 3.0, 0.0, 0.0);
        let p3 = Vec3::new(10.0, 0.0, 0.0);
        assert!(approx_eq_f32(
            bezier_cubic_3d_param_at_length(p0, p1, p2, p3, 0.0, 100).unwrap(),
            0.0
        ));
        assert!(approx_eq_f32(
            bezier_cubic_3d_param_at_length(p0, p1, p2, p3, 1.0, 100).unwrap(),
            1.0
        ));
    }

    #[test]
    fn param_at_length_midpoint_straight_line() {
        // Straight line: s=0.5 should give t≈0.5
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(10.0 / 3.0, 0.0, 0.0);
        let p2 = Vec3::new(20.0 / 3.0, 0.0, 0.0);
        let p3 = Vec3::new(10.0, 0.0, 0.0);
        let t = bezier_cubic_3d_param_at_length(p0, p1, p2, p3, 0.5, 100).unwrap();
        assert!((t - 0.5).abs() < 0.02);
    }

    #[test]
    fn gauss_legendre_5_high_degree_poly() {
        // GL5 is exact for degree <= 9: test x^8
        // ∫₀¹ x^8 dx = 1/9
        let result = integral_gauss_legendre_5(|x| x.powi(8), 0.0, 1.0);
        assert!((result - 1.0 / 9.0).abs() < 1e-10);
    }

    #[test]
    fn bspline_quadratic() {
        // Degree 2, 3 control points
        let pts = [
            Vec3::ZERO,
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let start = bspline_eval(2, &pts, &knots, 0.0).unwrap();
        let end = bspline_eval(2, &pts, &knots, 1.0).unwrap();
        assert!(vec3_approx_eq(start, pts[0]));
        assert!(vec3_approx_eq(end, pts[2]));
    }

    #[test]
    fn catmull_rom_quarter() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(2.0, 0.0, 0.0);
        let p3 = Vec3::new(3.0, 0.0, 0.0);
        let q = catmull_rom(p0, p1, p2, p3, 0.25);
        assert!(approx_eq_f32(q.x, 1.25));
    }

    #[test]
    fn bezier_quadratic_3d_midpoint() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(0.5, 0.5, 0.5);
        let p2 = Vec3::ONE;
        let mid = bezier_quadratic_3d(p0, p1, p2, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::splat(0.5)));
    }

    #[test]
    fn ease_in_out_smooth_c2_symmetry() {
        // Smootherstep is symmetric: f(t) + f(1-t) = 1
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let sum = ease_in_out_smooth(t) + ease_in_out_smooth(1.0 - t);
            assert!(approx_eq_f32(sum, 1.0));
        }
    }

    // --- Adaptive Simpson tests ---

    #[test]
    fn adaptive_simpson_quadratic() {
        // ∫₀¹ x² dx = 1/3
        let result = integral_adaptive_simpson(|x| x * x, 0.0, 1.0, 1e-10).unwrap();
        assert!((result - 1.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn adaptive_simpson_sin() {
        // ∫₀^π sin(x) dx = 2
        let result = integral_adaptive_simpson(f64::sin, 0.0, std::f64::consts::PI, 1e-10).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn adaptive_simpson_high_accuracy() {
        // ∫₀¹ e^x dx = e - 1
        let result = integral_adaptive_simpson(f64::exp, 0.0, 1.0, 1e-12).unwrap();
        assert!((result - (std::f64::consts::E - 1.0)).abs() < 1e-11);
    }

    #[test]
    fn adaptive_simpson_invalid_interval() {
        assert!(integral_adaptive_simpson(|x| x, 1.0, 0.0, 1e-6).is_err());
    }

    // --- Monte Carlo integration tests ---

    #[test]
    fn monte_carlo_constant() {
        // ∫₀¹ 1 dx = 1
        let (est, _err) = integral_monte_carlo(|_| 1.0, &[(0.0, 1.0)], 10000).unwrap();
        assert!((est - 1.0).abs() < 0.05);
    }

    #[test]
    fn monte_carlo_linear() {
        // ∫₀¹ x dx = 0.5
        let (est, _err) = integral_monte_carlo(|x| x[0], &[(0.0, 1.0)], 50000).unwrap();
        assert!((est - 0.5).abs() < 0.05);
    }

    #[test]
    fn monte_carlo_2d() {
        // ∫₀¹ ∫₀¹ (x+y) dx dy = 1
        let (est, _err) =
            integral_monte_carlo(|x| x[0] + x[1], &[(0.0, 1.0), (0.0, 1.0)], 50000).unwrap();
        assert!((est - 1.0).abs() < 0.1);
    }

    #[test]
    fn monte_carlo_empty_bounds() {
        assert!(integral_monte_carlo(|_| 1.0, &[], 100).is_err());
    }

    #[test]
    fn monte_carlo_zero_samples() {
        assert!(integral_monte_carlo(|_| 1.0, &[(0.0, 1.0)], 0).is_err());
    }

    #[test]
    fn monte_carlo_invalid_bounds() {
        assert!(integral_monte_carlo(|_| 1.0, &[(1.0, 0.0)], 100).is_err());
    }

    // --- Multivariable calculus tests ---

    #[test]
    fn partial_derivative_linear() {
        let f = |x: &[f64]| 3.0 * x[0] + 2.0 * x[1];
        let x = [1.0, 1.0];
        let dfx = partial_derivative(f, &x, 0, 1e-7).unwrap();
        let dfy = partial_derivative(f, &x, 1, 1e-7).unwrap();
        assert!(approx_eq_f64(dfx, 3.0));
        assert!(approx_eq_f64(dfy, 2.0));
    }

    #[test]
    fn partial_derivative_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] * x[1];
        let x = [3.0, 2.0];
        let dfx = partial_derivative(f, &x, 0, 1e-5).unwrap();
        let dfy = partial_derivative(f, &x, 1, 1e-5).unwrap();
        assert!((dfx - 12.0).abs() < 1e-4);
        assert!((dfy - 9.0).abs() < 1e-4);
    }

    #[test]
    fn partial_derivative_out_of_range() {
        let f = |x: &[f64]| x[0];
        assert!(partial_derivative(f, &[1.0], 5, 1e-7).is_err());
    }

    #[test]
    fn gradient_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + 2.0 * x[1] * x[1] + 3.0 * x[2] * x[2];
        let x = [1.0, 2.0, 3.0];
        let g = gradient(f, &x, 1e-5).unwrap();
        assert!((g[0] - 2.0).abs() < 1e-4);
        assert!((g[1] - 8.0).abs() < 1e-4);
        assert!((g[2] - 18.0).abs() < 1e-4);
    }

    #[test]
    fn gradient_empty_errors() {
        let f = |_: &[f64]| 0.0;
        assert!(gradient(f, &[], 1e-7).is_err());
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn jacobian_linear_map() {
        let f1: &dyn Fn(&[f64]) -> f64 = &|x: &[f64]| 2.0 * x[0] + x[1];
        let f2: &dyn Fn(&[f64]) -> f64 = &|x: &[f64]| x[0] - 3.0 * x[1];
        let fs: Vec<&dyn Fn(&[f64]) -> f64> = vec![f1, f2];
        let x = [1.0, 1.0];
        let j = jacobian(&fs, &x, 1e-7).unwrap();
        assert!((j[0][0] - 2.0).abs() < 1e-4);
        assert!((j[0][1] - 1.0).abs() < 1e-4);
        assert!((j[1][0] - 1.0).abs() < 1e-4);
        assert!((j[1][1] - (-3.0)).abs() < 1e-4);
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn jacobian_empty_errors() {
        let fs: Vec<&dyn Fn(&[f64]) -> f64> = vec![];
        assert!(jacobian(&fs, &[1.0], 1e-7).is_err());
    }

    #[test]
    fn hessian_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + 3.0 * x[0] * x[1] + 2.0 * x[1] * x[1];
        let x = [1.0, 1.0];
        let h = hessian(f, &x, 1e-4).unwrap();
        assert!((h[0][0] - 2.0).abs() < 1e-3);
        assert!((h[0][1] - 3.0).abs() < 1e-3);
        assert!((h[1][0] - 3.0).abs() < 1e-3);
        assert!((h[1][1] - 4.0).abs() < 1e-3);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn hessian_symmetric() {
        let f = |x: &[f64]| x[0] * x[0] * x[1] + x[1] * x[1] * x[2] + x[0] * x[2];
        let x = [2.0, 3.0, 4.0];
        let h = hessian(f, &x, 1e-4).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (h[i][j] - h[j][i]).abs() < 1e-3,
                    "Hessian not symmetric at [{i}][{j}]"
                );
            }
        }
    }

    #[test]
    fn hessian_empty_errors() {
        let f = |_: &[f64]| 0.0;
        assert!(hessian(f, &[], 1e-7).is_err());
    }

    // --- Noise tests ---

    #[test]
    fn perlin_2d_deterministic() {
        let a = perlin_2d(1.5, 2.3);
        let b = perlin_2d(1.5, 2.3);
        assert!((a - b).abs() < 1e-15);
    }

    #[test]
    fn perlin_2d_range() {
        for i in 0..100 {
            let x = i as f64 * 0.37;
            let y = i as f64 * 0.53;
            let v = perlin_2d(x, y);
            assert!((-1.5..=1.5).contains(&v), "perlin_2d out of range: {v}");
        }
    }

    #[test]
    fn perlin_3d_deterministic() {
        let a = perlin_3d(0.5, 1.5, 2.5);
        let b = perlin_3d(0.5, 1.5, 2.5);
        assert!((a - b).abs() < 1e-15);
    }

    #[test]
    fn fbm_2d_range() {
        let v = fbm_2d(perlin_2d, 1.0, 2.0, 4, 2.0, 0.5);
        assert!(v > -2.0 && v < 2.0);
    }

    #[test]
    fn fbm_2d_more_octaves_more_detail() {
        // fBm with 1 octave = raw noise; more octaves adds detail (different value)
        let v1 = fbm_2d(perlin_2d, 1.23, 4.56, 1, 2.0, 0.5);
        let v4 = fbm_2d(perlin_2d, 1.23, 4.56, 4, 2.0, 0.5);
        // They should differ (more octaves changes the result)
        assert!((v1 - v4).abs() > 1e-6 || v1.abs() < 1e-10);
    }

    // --- Spring dynamics tests ---

    #[test]
    fn spring_critically_damped_converges() {
        let stiffness: f64 = 100.0;
        let damping = 2.0 * stiffness.sqrt(); // critical damping
        let mut pos = 0.0;
        let mut vel = 0.0;
        let target = 1.0;
        for _ in 0..1000 {
            let (p, v) = spring_step(pos, target, vel, stiffness, damping, 0.01);
            pos = p;
            vel = v;
        }
        assert!((pos - target).abs() < 1e-4, "pos={pos}");
        assert!(vel.abs() < 1e-4, "vel={vel}");
    }

    #[test]
    fn spring_underdamped_oscillates() {
        let stiffness = 100.0;
        let damping = 2.0; // underdamped (zeta < 1)
        let mut pos = 1.0;
        let mut vel = 0.0;
        let target = 0.0;
        let mut crossed_zero = false;
        for _ in 0..500 {
            let (p, v) = spring_step(pos, target, vel, stiffness, damping, 0.01);
            if pos > 0.0 && p < 0.0 {
                crossed_zero = true;
            }
            pos = p;
            vel = v;
        }
        assert!(
            crossed_zero,
            "underdamped spring should oscillate past target"
        );
    }

    // --- Cubic bezier easing tests ---

    #[test]
    fn cubic_bezier_ease_endpoints() {
        // Any curve should map 0→0 and 1→1
        assert!(approx_eq_f32(
            cubic_bezier_ease(0.25, 0.1, 0.25, 1.0, 0.0),
            0.0
        ));
        assert!(approx_eq_f32(
            cubic_bezier_ease(0.25, 0.1, 0.25, 1.0, 1.0),
            1.0
        ));
    }

    #[test]
    fn cubic_bezier_ease_linear() {
        // Linear: (0.0, 0.0, 1.0, 1.0) should give y ≈ t
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let y = cubic_bezier_ease(0.0, 0.0, 1.0, 1.0, t);
            assert!((y - t).abs() < 0.02, "linear ease at t={t}: got {y}");
        }
    }

    #[test]
    fn cubic_bezier_ease_monotonic() {
        // CSS ease: (0.25, 0.1, 0.25, 1.0) should be monotonically increasing
        let mut prev = 0.0;
        for i in 1..=20 {
            let t = i as f32 / 20.0;
            let y = cubic_bezier_ease(0.25, 0.1, 0.25, 1.0, t);
            assert!(y >= prev - 1e-4, "non-monotonic at t={t}: {prev} → {y}");
            prev = y;
        }
    }
}
