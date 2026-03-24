# Roadmap

> **Hisab** (Arabic: حساب — calculation, reckoning) — higher mathematics library for the AGNOS ecosystem.
> Basic expression evaluation and unit conversion lives in [abaco](https://github.com/MacCracken/abaco).

## Scope

Hisab owns **typed mathematical operations** — the programmatic math that engines, physics, rendering, and simulation need. It does NOT own:

- **Expression parsing, user-typed math** → abaco (eval engine)
- **Unit conversion** → abaco (unit registry)
- **Physics simulation** → impetus (wraps rapier, uses hisab types)
- **Game engine** → kiran (uses hisab for transforms/projections)

## Consumers

| Consumer | What it uses from hisab |
|----------|----------------------|
| **impetus** | Vectors, quaternions, transforms, spatial geometry for broadphase |
| **kiran** | Projections, transforms, frustum culling, camera math |
| **joshua** | ODE solvers, simulation math, deterministic replay |
| **aethersafha** | Projection matrices, transform composition for compositor |
| **abaco** | Future: symbolic algebra backend for advanced expression evaluation |

## Versioning

Pre-1.0 releases follow **calver/semver hybrid**: `0.DD.M` (day.month).
Post-1.0 releases follow standard semver.

---

## Completed Milestones

### 0.22.3 — Foundation + Geometry + Curves + Numerical (2026-03-22)
- transforms: Transform2D/3D, projections, lerp, slerp, inverse_matrix, flip_handedness_z
- geo: Ray, Plane, Aabb, Sphere, Triangle, Line, Segment, Frustum, Rect
- geo: Ray-plane/sphere/AABB/triangle intersection, closest-point functions
- geo: BVH, KdTree, Quadtree, Octree, SpatialHash
- geo: 2D convex hull, GJK/EPA collision detection
- calc: derivative, trapezoidal/Simpson/Gauss-Legendre integration, 2D/3D Bezier
- calc: de Casteljau, Catmull-Rom, B-spline, arc-length, easing functions
- num: Newton-Raphson, bisection, Gaussian elimination, LU/Cholesky/QR
- num: least-squares fitting, eigenvalues, Complex, FFT/IFFT, RK4
- ai: daimon/hoosh client (feature-gated)
- Infrastructure: feature flags, unified HisabError, CI, 360 tests, 82 benchmarks

### 0.24.3 — DST/DCT, Display, Rect parity, Hardening, Complex API (2026-03-24)
- num: DST-I/IDST-I, DCT-II/IDCT
- geo: Display for Ray, Plane, Aabb, Sphere, Triangle
- geo: Rect::merge(), Rect::area()
- transforms: Transform2D::inverse_matrix()
- **Audit — Safety**: All `assert!`/`unwrap`/`panic!` replaced with `Result` returns
- **Audit — Quality**: `#[must_use]` on all ~90 pure public fns, `#[inline]` on 14 hot-path fns
- **API**: Complex gains `Div`, `Div<f64>`, `Neg`, `From<f64>`, `From<(f64,f64)>`, Serialize/Deserialize
- **Helpers**: `matrix_determinant()`, `matrix_trace()`, `matrix_multiply()`
- **Epsilon**: `EPSILON_F32`, `EPSILON_F64` constants; all tolerance checks normalized
- **Docs**: `# Errors` sections on all Result-returning fns, initial doctests, zero `cargo doc` warnings
- License `GPL-3.0` → `GPL-3.0-only`, removed duplicate Result alias
- 394+ tests, 82 benchmarks

---

## Upcoming Milestones

### 0.25.3 — Performance (2026-03-25)
**Focus:** Eliminate hot-path allocations, tighten inner loops.

- [ ] `rk4`/`rk4_trajectory`: refactor closure to `f(t, y, out: &mut [f64])` — 4 allocs/step → 0
- [ ] GJK: replace `Vec` simplex with fixed `[Vec2; 3]` array
- [ ] EPA: replace `polytope.insert()` O(n) with circular buffer
- [ ] `convex_hull_2d`: take `&mut [Vec2]` or document the clone
- [ ] `lu_decompose`/`qr_decompose`: offer in-place variants
- [ ] Benchmark before/after — prove the wins

### 0.26.3 — Extended Linear Algebra + Multivariable Calculus (2026-03-26)
**Focus:** Complete numerical toolkit, extend calc to N-D.

- [ ] SVD (Singular Value Decomposition)
- [ ] Matrix rank, condition number, inverse (via LU), pseudo-inverse
- [ ] Sparse matrix support (CSR format)
- [ ] `partial_derivative()`, `gradient()`, `jacobian()`, `hessian()`
- [ ] `integral_monte_carlo()`, `integral_adaptive_simpson()`

### 0.27.3 — Optimization Solvers + 3D Collision (2026-03-27)
**Focus:** Iterative solvers and 3D geometry.

- [ ] Gradient descent, conjugate gradient, BFGS/L-BFGS, Levenberg-Marquardt
- [ ] Adaptive RK4/5 (Dormand-Prince) with step-size control
- [ ] 3D convex hull (Quickhull), 3D GJK, 3D EPA
- [ ] OBB, Capsule primitives, mesh-mesh intersection

### 0.28.3 — Autodiff, Interval, Symbolic, Tensor (2026-03-28)
**Focus:** Advanced math modules (feature-gated).

- [ ] Forward-mode automatic differentiation (dual numbers) — `autodiff`
- [ ] Interval arithmetic for verified numerics — `interval`
- [ ] Symbolic algebra primitives (simplify, expand, factor, differentiate) — `symbolic`
- [ ] N-dimensional tensor type for ML interop — `tensor`

### 0.29.3 — GPU, Parallelism, Pre-publish Polish (2026-03-29)
**Focus:** Acceleration + final quality pass.

- [ ] Compute kernels via wgpu (shared with ranga) — `gpu`
- [ ] Rayon integration for batch spatial queries, large FFTs — `parallel`
- [ ] Complete doctests across all modules
- [ ] Final API review — naming consistency, argument order
- [ ] `cargo semver-checks`, update CHANGELOG + README
- [ ] Full benchmark sweep and history baseline

---

## V1 — Stable Release (2026-03-31)

- [ ] Bump version to 1.0.0 via `scripts/version-bump.sh`
- [ ] Tag and push — CI handles crates.io publish + GitHub Release
- [ ] Announce to AGNOS consumers (impetus, kiran, joshua, aethersafha)

---

## Known Technical Debt

| Area | Issue | Severity | Target |
|------|-------|----------|--------|
| num | rk4 closure allocates Vec per call (4x per step) | High | 0.25.3 |
| geo | GJK/EPA hardcoded to 64 iterations, not configurable | Medium | 0.25.3 |
| geo | EPA `polytope.insert()` is O(n) per iteration | Medium | 0.25.3 |
| calc | `convex_hull_2d` clones input unnecessarily | Low | 0.25.3 |
| geo | 3D collision deferred (only 2D GJK/EPA) | Medium | 0.27.3 |

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | abaco parses and evaluates | — |
| `Vec3::cross(a, b)` | — | transforms |
| `ray.intersect(sphere)` | — | geo |
| `integral(f, 0, 1)` | — | calc |
| `newton_raphson(f, df, x0)` | — | num |
| `eval("solve x^2 - 2 = 0")` | abaco parses | num solves |

Hisab should never depend on abaco. Abaco may optionally depend on hisab (num) for symbolic/solver features in the future.
