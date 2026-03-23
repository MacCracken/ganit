# Benchmarks

Latest: **2026-03-23T02:11:55Z** — commit `7f26a9a`

Tracking: `9463cf0` (baseline) → `7d870de` (optimized) → `7f26a9a` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.17 ns **-12%** | 5.97 ns **-15%** |
| `transform2d_apply_point` | 12.18 ns | 6.17 ns **-49%** | 6.12 ns **-50%** |
| `transform3d_to_matrix` | 11.20 ns | 5.24 ns **-53%** | 5.19 ns **-54%** |
| `transform3d_apply_point` | 13.71 ns | 5.93 ns **-57%** | 5.87 ns **-57%** |
| `projection_perspective` | 13.99 ns | 21.89 ns +57% | 13.98 ns |
| `projection_orthographic` | 4.29 ns | 4.05 ns **-6%** | 4.07 ns **-5%** |
| `lerp_f32` | 1.05 ns | 1.00 ns **-5%** | 1.01 ns **-4%** |
| `lerp_vec3` | 2.71 ns | 2.65 ns | 2.61 ns **-4%** |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.72 ns **-50%** | 2.71 ns **-51%** |
| `ray_plane_hit` | 2.10 ns | 1.99 ns **-5%** | 1.97 ns **-6%** |
| `ray_aabb_hit` | 4.52 ns | 4.56 ns | 4.74 ns +5% |
| `ray_sphere_miss` | 3.40 ns | 2.03 ns **-40%** | 1.91 ns **-44%** |
| `aabb_contains` | 3.05 ns | 2.84 ns **-7%** | 2.85 ns **-7%** |
| `sphere_contains` | 2.31 ns | 2.21 ns **-4%** | 2.20 ns **-5%** |
| `aabb_merge` | 4.13 ns | 3.85 ns **-7%** | 3.79 ns **-8%** |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.09 ns | 1.06 ns **-4%** |
| `integral_simpson_100` | 78.23 ns | 76.43 ns | 75.83 ns **-3%** |
| `integral_simpson_1000` | 770.1 ns | 728.3 ns **-5%** | 749.5 ns |
| `integral_trapezoidal_100` | 78.23 ns | 69.70 ns **-11%** | 68.92 ns **-12%** |
| `integral_trapezoidal_1000` | 753.4 ns | 743.4 ns | 708.4 ns **-6%** |
| `bezier_quadratic` | 1.49 ns | 1.43 ns **-4%** | 1.41 ns **-5%** |
| `bezier_cubic` | 2.36 ns | 2.36 ns | 2.29 ns |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.17 ns **-7%** | 6.11 ns **-8%** |
| `bisection_sqrt2` | 113.2 ns | 104.3 ns **-8%** | 103.1 ns **-9%** |
| `gaussian_3x3` | 83.64 ns | 74.59 ns **-11%** | 79.98 ns **-4%** |
| `gaussian_4x4` | 121.1 ns | 103.1 ns **-15%** | 104.3 ns **-14%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 189.9 ns | 185.9 ns |
| `aabb_contains_100` | — | 128.8 ns | 124.1 ns |
| `transform3d_batch_100` | — | 362.1 ns | 349.1 ns |
| `simpson_sin_10000` | — | 79353.0 ns | 78076.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.43 ns | 7.26 ns |
| `aabb_aabb_overlap` | — | 2.22 ns | 2.18 ns |
| `sphere_sphere_overlap` | — | 1.80 ns | 1.75 ns |
| `frustum_contains_point` | — | 4.52 ns | 4.39 ns |
| `frustum_contains_aabb` | — | 4.39 ns | 4.16 ns |
| `slerp` | — | 19.45 ns | 17.21 ns |
| `transform3d_lerp` | — | 24.25 ns | 20.22 ns |
| `closest_on_aabb` | — | 2.64 ns | 2.50 ns |
| `segment_closest_point` | — | 2.95 ns | 2.88 ns |
| `plane_plane_intersection` | — | 7.21 ns | 6.93 ns |
| `triangle_unit_normal` | — | 5.05 ns | 4.86 ns |
| `line_closest_point` | — | 2.59 ns | 2.17 ns |
| `closest_on_sphere` | — | 5.06 ns | 4.99 ns |
| `inverse_matrix` | — | 18.89 ns | 18.30 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.92 ns | 2.73 ns |
| `de_casteljau_split` | — | 5.85 ns | 5.65 ns |
| `catmull_rom` | — | 2.52 ns | 2.62 ns |
| `bspline_cubic` | — | 21.93 ns | 22.00 ns |
| `gauss_legendre_5` | — | 3.33 ns | 3.32 ns |
| `gauss_legendre_10_panels` | — | 404.4 ns | 399.7 ns |
| `arc_length_100` | — | 554.4 ns | 550.0 ns |
| `ease_in_out` | — | 0.56 ns | 0.55 ns |
| `ease_in_out_smooth` | — | 0.79 ns | 0.78 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | — | 110.5 ns |
| `lu_solve_3x3` | — | — | 32.29 ns |
| `cholesky_3x3` | — | — | 62.49 ns |
| `cholesky_solve_3x3` | — | — | 37.68 ns |
| `qr_decompose_3col` | — | — | 127.3 ns |
| `least_squares_linear_6pt` | — | — | 177.1 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | — | 402.1 ns |
| `fft_64` | — | — | 607.6 ns |
| `fft_1024` | — | — | 15166.0 ns |
| `fft_ifft_256` | — | — | 6205.3 ns |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2527.8 ns |
| `rk4_exp_1000_steps` | — | — | 25193.0 ns |
| `rk4_oscillator_1000` | — | — | 24136.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`7d870de`) | Current (`7f26a9a`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 10695.0 ns |
| `bvh_ray_query_100` | — | — | 62.08 ns |
| `bvh_build_1000` | — | — | 141510.0 ns |
| `kdtree_build_1000` | — | — | 139570.0 ns |
| `kdtree_nearest_1000` | — | — | 265.8 ns |
| `kdtree_radius_1000` | — | — | 1401.1 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
