# Benchmarks

Latest: **2026-03-23T02:27:33Z** — commit `fc9aa33`

Tracking: `9463cf0` (baseline) → `0314972` (optimized) → `fc9aa33` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.09 ns **-13%** | 6.12 ns **-12%** |
| `transform2d_apply_point` | 12.18 ns | 6.14 ns **-50%** | 6.09 ns **-50%** |
| `transform3d_to_matrix` | 11.20 ns | 5.16 ns **-54%** | 5.18 ns **-54%** |
| `transform3d_apply_point` | 13.71 ns | 6.01 ns **-56%** | 5.92 ns **-57%** |
| `projection_perspective` | 13.99 ns | 22.07 ns +58% | 21.54 ns +54% |
| `projection_orthographic` | 4.29 ns | 4.04 ns **-6%** | 4.01 ns **-6%** |
| `lerp_f32` | 1.05 ns | 1.00 ns **-5%** | 0.99 ns **-6%** |
| `lerp_vec3` | 2.71 ns | 2.60 ns **-4%** | 2.59 ns **-5%** |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.73 ns **-50%** | 2.70 ns **-51%** |
| `ray_plane_hit` | 2.10 ns | 2.01 ns **-4%** | 1.97 ns **-6%** |
| `ray_aabb_hit` | 4.52 ns | 4.54 ns | 4.54 ns |
| `ray_sphere_miss` | 3.40 ns | 1.92 ns **-43%** | 2.00 ns **-41%** |
| `aabb_contains` | 3.05 ns | 2.81 ns **-8%** | 2.84 ns **-7%** |
| `sphere_contains` | 2.31 ns | 2.22 ns **-4%** | 2.22 ns **-4%** |
| `aabb_merge` | 4.13 ns | 3.95 ns **-4%** | 4.00 ns **-3%** |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.07 ns **-4%** | 1.09 ns |
| `integral_simpson_100` | 78.23 ns | 75.67 ns **-3%** | 76.63 ns |
| `integral_simpson_1000` | 770.1 ns | 742.1 ns **-4%** | 757.9 ns |
| `integral_trapezoidal_100` | 78.23 ns | 80.89 ns +3% | 80.02 ns |
| `integral_trapezoidal_1000` | 753.4 ns | 714.6 ns **-5%** | 716.0 ns **-5%** |
| `bezier_quadratic` | 1.49 ns | 1.43 ns **-4%** | 1.43 ns **-4%** |
| `bezier_cubic` | 2.36 ns | 2.24 ns **-5%** | 2.22 ns **-6%** |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.20 ns **-6%** | 6.12 ns **-7%** |
| `bisection_sqrt2` | 113.2 ns | 102.3 ns **-10%** | 104.0 ns **-8%** |
| `gaussian_3x3` | 83.64 ns | 74.40 ns **-11%** | 74.93 ns **-10%** |
| `gaussian_4x4` | 121.1 ns | 102.1 ns **-16%** | 104.2 ns **-14%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 188.0 ns | 186.7 ns |
| `aabb_contains_100` | — | 126.8 ns | 125.0 ns |
| `transform3d_batch_100` | — | 357.2 ns | 358.1 ns |
| `simpson_sin_10000` | — | 79271.0 ns | 79820.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.42 ns | 7.38 ns |
| `aabb_aabb_overlap` | — | 2.28 ns | 2.23 ns |
| `sphere_sphere_overlap` | — | 1.77 ns | 1.76 ns |
| `frustum_contains_point` | — | 4.45 ns | 4.44 ns |
| `frustum_contains_aabb` | — | 4.30 ns | 4.25 ns |
| `slerp` | — | 19.59 ns | 19.22 ns |
| `transform3d_lerp` | — | 20.79 ns | 24.06 ns |
| `closest_on_aabb` | — | 2.60 ns | 2.52 ns |
| `segment_closest_point` | — | 2.96 ns | 2.91 ns |
| `plane_plane_intersection` | — | 7.28 ns | 7.04 ns |
| `triangle_unit_normal` | — | 5.02 ns | 4.91 ns |
| `line_closest_point` | — | 2.10 ns | 2.12 ns |
| `closest_on_sphere` | — | 5.05 ns | 4.87 ns |
| `inverse_matrix` | — | 18.74 ns | 18.24 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.77 ns | 2.70 ns |
| `de_casteljau_split` | — | 6.01 ns | 5.66 ns |
| `catmull_rom` | — | 2.54 ns | 2.50 ns |
| `bspline_cubic` | — | 22.02 ns | 21.66 ns |
| `gauss_legendre_5` | — | 3.44 ns | 3.30 ns |
| `gauss_legendre_10_panels` | — | 404.7 ns | 399.6 ns |
| `arc_length_100` | — | 559.1 ns | 552.8 ns |
| `ease_in_out` | — | 0.56 ns | 0.56 ns |
| `ease_in_out_smooth` | — | 0.80 ns | 0.78 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 104.5 ns | 99.78 ns |
| `lu_solve_3x3` | — | 34.90 ns | 32.36 ns |
| `cholesky_3x3` | — | 64.40 ns | 62.69 ns |
| `cholesky_solve_3x3` | — | 38.98 ns | 36.21 ns |
| `qr_decompose_3col` | — | 126.7 ns | 123.1 ns |
| `least_squares_linear_6pt` | — | 185.4 ns | 173.5 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | — | 399.3 ns |
| `fft_64` | — | — | 612.9 ns |
| `fft_1024` | — | — | 15168.0 ns |
| `fft_ifft_256` | — | — | 6149.7 ns |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2333.1 ns |
| `rk4_exp_1000_steps` | — | — | 25720.0 ns |
| `rk4_oscillator_1000` | — | — | 26020.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`fc9aa33`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 7322.8 ns |
| `bvh_ray_query_100` | — | — | 62.64 ns |
| `bvh_build_1000` | — | — | 93431.0 ns |
| `kdtree_build_1000` | — | — | 104080.0 ns |
| `kdtree_nearest_1000` | — | — | 246.4 ns |
| `kdtree_radius_1000` | — | — | 1427.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
