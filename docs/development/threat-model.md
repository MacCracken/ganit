# Threat Model

## Trust Boundaries

Hisab operates at the **library boundary**. It trusts the calling application to:
- Provide valid numeric inputs (not NaN/Infinity unless documented)
- Respect feature-gate contracts (AI module requires network access)
- Handle `Result` errors appropriately

Hisab does NOT trust:
- Input sizes (validates dimensions for matrix operations)
- Input magnitudes (uses epsilon thresholds for near-zero detection)

## Attack Surface

| Module | Risk | Mitigation |
|--------|------|------------|
| `num` (gaussian/LU) | Division by near-zero pivot | Partial pivoting with 1e-12 threshold; returns `Err(SingularPivot)` |
| `num` (newton/bisection) | Non-convergence / infinite loop | `max_iter` parameter; returns `Err(NoConvergence)` |
| `num` (eigenvalue) | Non-convergence for repeated eigenvalues | `max_iter` + tolerance; returns `Err(NoConvergence)` |
| `num` (cholesky) | Non-positive-definite matrix | Returns `Err(InvalidInput)` on non-positive diagonal |
| `num` (fft) | Non-power-of-2 input | Returns `Err(InvalidInput)` |
| `calc` (integration) | Zero step count | Returns `Err(ZeroSteps)` |
| `geo` (ConvexPolygon) | Empty vertex list | Validated in constructor; returns `Err(InvalidInput)` |
| `geo` (Sphere) | Negative radius | Validated in constructor; returns `Err(InvalidInput)` |
| `geo` (Ray/Line) | Zero-length direction | Validated in constructor; returns `Err(InvalidInput)` |
| `geo` (SpatialHash) | Non-positive cell size | Validated in constructor; returns `Err(InvalidInput)` |
| `geo` (GJK/EPA) | Non-convergence on degenerate shapes | 64-iteration hard limit; returns false/None |
| `geo` (Quadtree/Octree) | Unbounded tree depth with coincident points | Configurable `max_depth` prevents stack overflow |
| `geo` (SpatialHash) | Memory growth with many cells | Caller controls cell_size; `clear()` available |
| `geo` (BVH/KdTree) | Stack overflow on deep recursion | Balanced construction limits depth to O(log n) |
| `ai` (DaimonClient) | Network I/O, untrusted responses | Feature-gated; not compiled by default |
| All | NaN/Infinity propagation | IEEE 754 semantics; no special handling (caller's responsibility) |

## Known Panic Sites

All previously identified panic sites have been resolved (0.24.3-audit). The library contains **zero `assert!`/`unwrap`/`panic!`** in non-test code.

## Unsafe Code

None. The crate contains zero `unsafe` blocks.

## Supply Chain

- `cargo-deny` enforces license allowlist, bans wildcards, denies unknown registries
- `cargo-audit` checks for known vulnerabilities in CI
- Minimal direct dependencies:
  - **Core:** glam, serde, thiserror, tracing (4 deps)
  - **AI (opt-in):** reqwest, tokio, serde_json (+3 deps)
  - **Logging (opt-in):** tracing-subscriber (+1 dep)
- No transitive dependency on `openssl` (reqwest uses rustls by default)

## Numerical Precision

Hisab uses inconsistent epsilon values across modules:

| Context | Current value | Location |
|---------|--------------|----------|
| Ray parallel to plane | `1e-8` | `geo::ray_plane` |
| Plane parallel check | `1e-12` | `geo::plane_plane` |
| Singular pivot | `1e-12` | `num::gaussian_elimination`, `num::lu_decompose` |
| Zero derivative | `1e-15` | `num::newton_raphson` |
| Zero eigenvector | `1e-15` | `num::eigenvalue_power` |
| Degenerate segment | `1e-12` | `geo::Segment::closest_point` |
| GJK degenerate | `1e-12` | `geo::gjk_intersect` |

**0.25.3 action:** Define `EPSILON_F32` and `EPSILON_F64` constants and normalize all checks.
