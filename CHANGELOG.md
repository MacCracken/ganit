# Changelog

## 0.24.3 (2026-03-24)

### num — Discrete Sine/Cosine Transforms
- `dst()` — Discrete Sine Transform Type-I (Dirichlet boundary conditions)
- `idst()` — Inverse DST-I (self-inverse with `2/(N+1)` scaling)
- `dct()` — Discrete Cosine Transform Type-II (Neumann boundary conditions)
- `idct()` — Inverse DCT (DCT-III)

### num — Complex API completeness
- `Div`, `Div<f64>`, `Neg`, `From<f64>`, `From<(f64, f64)>` operators
- `Serialize`/`Deserialize` derives

### num — Matrix helpers
- `matrix_determinant()` — via LU decomposition with permutation parity
- `matrix_trace()` — sum of diagonal elements
- `matrix_multiply()` — dense A*B with dimension validation

### geo — Display + Rect parity
- `Display` for `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`
- `Rect::merge()`, `Rect::area()`

### transforms
- `Transform2D::inverse_matrix()`

### Safety — Panic elimination
- All `assert!`/`unwrap`/`panic!` in library code replaced with `Result` returns
  - calc: `integral_trapezoidal`, `integral_simpson`, `integral_gauss_legendre`, `bezier_cubic_3d_arc_length`, `bezier_cubic_3d_param_at_length`
  - num: `fft`, `ifft`, `rk4`, `rk4_trajectory`
  - geo: `SpatialHash::new`, `ConvexPolygon::new`, `Sphere::new`, `Ray::new`, `Line::new`

### Quality
- `#[must_use]` on all ~90 pure public functions/methods
- `#[inline]` on 14 hot-path functions
- Removed duplicate `Result<T>` type alias
- License identifier: `GPL-3.0` → `GPL-3.0-only`

### Stats
- 394 tests, 82 benchmarks, zero clippy warnings

## 0.22.3 (2026-03-22)

Initial release — 360 tests, 82 benchmarks.

### transforms
- Vec2/Vec3/Vec4/Mat3/Mat4/Quat re-exports from glam
- `Transform2D`, `Transform3D` with `to_matrix()`, `apply_to_point()`
- Orthographic and perspective projection matrices
- `lerp_f32`, `lerp_vec3`, `slerp`, `transform3d_lerp`
- `Transform3D::inverse_matrix()`, `flip_handedness_z()`

### geo — Primitives & Intersections
- `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`, `Line`, `Segment`
- `Frustum` with `contains_point()`, `contains_aabb()`
- Ray-plane, ray-sphere, ray-AABB, ray-triangle intersection
- Plane-plane, AABB-AABB, sphere-sphere overlap
- Closest point on ray, plane, sphere, AABB
- `Rect` with `contains_point()`, `overlaps()`

### geo — Spatial Structures
- `Bvh`, `KdTree`, `Quadtree`, `Octree`, `SpatialHash`

### geo — Collision
- `convex_hull_2d()`, `ConvexSupport` trait, `ConvexPolygon`
- `gjk_intersect()`, `epa_penetration()`, `gjk_epa()`

### calc
- `derivative`, `integral_trapezoidal`, `integral_simpson`
- `integral_gauss_legendre_5`, `integral_gauss_legendre`
- `bezier_quadratic/cubic` (2D/3D), `de_casteljau_split`
- `catmull_rom`, `bspline_eval`, arc-length parameterization
- Easing: `ease_in/out/in_out`, cubic, quintic smootherstep

### num
- `newton_raphson`, `bisection`, `gaussian_elimination`
- `lu_decompose/solve`, `cholesky/solve`, `qr_decompose`
- `least_squares_poly`, `eigenvalue_power`
- `Complex` with arithmetic, `fft`/`ifft`
- `rk4`, `rk4_trajectory`

### Infrastructure
- Feature flags, unified `HisabError`, `DaimonError`
- CI, benchmarks with CSV history, docs
