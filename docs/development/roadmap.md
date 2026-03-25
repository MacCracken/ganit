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
| **impetus** | Vectors, quaternions, transforms, spatial geometry, 3D GJK/EPA broadphase+narrowphase |
| **kiran** | Projections, transforms, frustum culling, camera math, OBB/Capsule ray tests |
| **joshua** | ODE solvers (RK4, DOPRI45), simulation math, deterministic replay |
| **aethersafha** | Projection matrices, transform composition/interpolation for compositor |
| **abaco** | Symbolic algebra (Expr), interval arithmetic for verified evaluation |

## Versioning

Post-1.0: standard semver.

---

## V1.0.0 — Stable Release

### Modules

| Module | Feature | Description |
|--------|---------|-------------|
| **transforms** | `transforms` (default) | Transform2D/3D, projections, lerp, slerp, glam re-exports |
| **geo** | `geo` (default) | Primitives, intersections, BVH, KdTree, Quadtree, Octree, SpatialHash, 2D+3D GJK/EPA, OBB, Capsule |
| **calc** | `calc` (default) | Differentiation, integration (Simpson, Gauss-Legendre, adaptive, Monte Carlo), Bezier, splines, easing, gradient/jacobian/hessian |
| **num** | `num` (default) | Root finding, LU/Cholesky/QR/SVD, FFT/DST/DCT, optimization (GD, CG, BFGS, L-BFGS, LM), ODE (RK4, DOPRI45), sparse matrices |
| **autodiff** | `autodiff` | Forward-mode automatic differentiation (dual numbers) |
| **interval** | `interval` | Interval arithmetic for verified numerics |
| **symbolic** | `symbolic` | Expression tree with evaluation, differentiation, simplification |
| **tensor** | `tensor` | N-dimensional dense tensor type |
| **parallel** | `parallel` | Rayon-powered parallel batch operations |
| **ai** | `ai` | Daimon/hoosh AI client |
| **logging** | `logging` | Structured logging via tracing-subscriber |

### Stats

- 617 tests (574 unit + 34 integration + 9 doc)
- 12 modules, 13 feature flags
- Zero clippy warnings, cargo audit clean, cargo deny clean
- Consumer smoke tests for impetus, kiran, joshua, aethersafha, abaco

---

## 1.0.1 — Low-Hanging Critical + Important (patch)

- [ ] Symplectic integrators: Verlet, leapfrog, symplectic Euler (joshua, impetus)
- [ ] Quaternion utilities: look_at, from_euler, to_euler with rotation order (kiran)
- [ ] `Frustum::contains_sphere()` (kiran)
- [ ] Spring dynamics: critically damped spring solver for UI animation (aethersafha)
- [ ] `cubic_bezier_ease()` — CSS cubic-bezier timing function (aethersafha)
- [ ] `world_to_screen()`, `screen_to_world_ray()` screen-space projection (kiran)
- [ ] sRGB/linear color space conversions (kiran, aethersafha)
- [ ] Noise functions: Perlin, simplex (OpenSimplex2), fBm (kiran)
- [ ] PCG random number generator (joshua deterministic replay)
- [ ] Expr::substitute() for symbolic module (abaco)
- [ ] 2D FFT/IFFT (prakash)
- [ ] Truncated SVD convenience function (num)

## 1.1.0 — Medium Critical + Important (minor)

- [ ] Contact manifold generation: SAT, Sutherland-Hodgman clipping (impetus)
- [ ] Inertia tensor computation from triangle mesh + primitive shapes (impetus)
- [ ] CSS transform decomposition: decompose_mat4/recompose_mat4 (aethersafha)
- [ ] Polygon triangulation: ear clipping (kiran)
- [ ] Dual quaternions for skinning: DualQuat type (kiran)
- [ ] Signed distance fields: analytical SDFs + CSG operations (impetus, kiran)
- [ ] Sort-and-sweep (SAP) broadphase (impetus)
- [ ] MPR / XenoCollide collision alternative (impetus)
- [ ] NURBS evaluation (kiran)
- [ ] Color matrix operations: saturation, hue rotation, Oklab/Oklch (aethersafha)
- [ ] Delaunay triangulation + Voronoi diagrams 2D (kiran)
- [ ] Ray-quadric intersection (prakash)
- [ ] Fresnel equations + refraction vector (prakash)
- [ ] Sparse LU/Cholesky factorization (joshua)
- [ ] GMRES, BiCGSTAB iterative sparse solvers (joshua)
- [ ] Spherical harmonics L0-L2 evaluation + projection (kiran)

## 1.2.0 — Heavy Critical (minor)

- [ ] Stiff ODE solvers: backward Euler, BDF-2 through BDF-5, TR-BDF2 (joshua)
- [ ] Full eigendecomposition: QR algorithm (Francis double-shift), symmetric tridiag+QR (num)
- [ ] Continuous collision detection: TOI via conservative advancement (impetus)
- [ ] Constraint solvers: sequential impulse, projected Gauss-Seidel (impetus)
- [ ] Stochastic differential equations: Euler-Maruyama, Milstein (joshua)
- [ ] Convex decomposition: V-HACD or Hertel-Mehlhorn (impetus)
- [ ] Stability analysis: Lyapunov exponents, eigenvalue-based (joshua)
- [ ] Reverse-mode automatic differentiation (tape-based) (num/autodiff)
- [ ] GPU compute kernels via wgpu (shared with ranga)

## Watch List

| Item | Area |
|------|------|
| Randomized SVD (Halko-Martinsson-Tropp) | num |
| Differentiable rendering math | geo/autodiff |
| Neural implicit representation primitives | tensor |
| Conformal geometric algebra | geo |
| Low-rank approximations (CUR, Nystrom) | num |

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | abaco parses and evaluates | — |
| `Vec3::cross(a, b)` | — | transforms |
| `ray.intersect(sphere)` | — | geo |
| `integral(f, 0, 1)` | — | calc |
| `newton_raphson(f, df, x0)` | — | num |
| `eval("solve x^2 - 2 = 0")` | abaco parses | num solves |

Hisab should never depend on abaco. Abaco may optionally depend on hisab (num, symbolic) for solver/algebra features.
