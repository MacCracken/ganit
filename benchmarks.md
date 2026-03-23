# Benchmarks

Latest: **2026-03-23T03:17:37Z** — commit `8f56bfa`

Tracking: `9463cf0` (baseline) → `ee84541` (optimized) → `8f56bfa` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.18 ns **-11%** | 6.06 ns **-13%** |
| `transform2d_apply_point` | 12.18 ns | 6.19 ns **-49%** | 5.99 ns **-51%** |
| `transform3d_to_matrix` | 11.20 ns | 5.30 ns **-53%** | 5.15 ns **-54%** |
| `transform3d_apply_point` | 13.71 ns | 6.08 ns **-56%** | 5.84 ns **-57%** |
| `projection_perspective` | 13.99 ns | 22.04 ns +58% | 21.22 ns +52% |
| `projection_orthographic` | 4.29 ns | 4.09 ns **-5%** | 4.00 ns **-7%** |
| `lerp_f32` | 1.05 ns | 1.02 ns **-3%** | 0.99 ns **-6%** |
| `lerp_vec3` | 2.71 ns | 2.69 ns | 2.58 ns **-5%** |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.75 ns **-50%** | 2.69 ns **-51%** |
| `ray_plane_hit` | 2.10 ns | 2.04 ns | 1.99 ns **-5%** |
| `ray_aabb_hit` | 4.52 ns | 4.85 ns +7% | 4.78 ns +6% |
| `ray_sphere_miss` | 3.40 ns | 1.95 ns **-42%** | 1.95 ns **-43%** |
| `aabb_contains` | 3.05 ns | 2.86 ns **-6%** | 2.84 ns **-7%** |
| `sphere_contains` | 2.31 ns | 2.26 ns | 2.22 ns **-4%** |
| `aabb_merge` | 4.13 ns | 3.90 ns **-6%** | 3.85 ns **-7%** |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.13 ns | 1.71 ns +54% |
| `integral_simpson_100` | 78.23 ns | 82.10 ns +5% | 105.5 ns +35% |
| `integral_simpson_1000` | 770.1 ns | 811.4 ns +5% | 931.7 ns +21% |
| `integral_trapezoidal_100` | 78.23 ns | 75.95 ns | 69.70 ns **-11%** |
| `integral_trapezoidal_1000` | 753.4 ns | 779.8 ns +3% | 714.6 ns **-5%** |
| `bezier_quadratic` | 1.49 ns | 1.61 ns +8% | 1.45 ns |
| `bezier_cubic` | 2.36 ns | 2.49 ns +5% | 2.22 ns **-6%** |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.97 ns +5% | 6.09 ns **-8%** |
| `bisection_sqrt2` | 113.2 ns | 114.0 ns | 103.4 ns **-9%** |
| `gaussian_3x3` | 83.64 ns | 84.77 ns | 106.5 ns +27% |
| `gaussian_4x4` | 121.1 ns | 115.0 ns **-5%** | 139.7 ns +15% |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 211.6 ns | 200.0 ns |
| `aabb_contains_100` | — | 141.1 ns | 133.4 ns |
| `transform3d_batch_100` | — | 407.5 ns | 386.7 ns |
| `simpson_sin_10000` | — | 87211.0 ns | 83223.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `ray_triangle` | — | 8.22 ns | 7.98 ns |
| `aabb_aabb_overlap` | — | 2.30 ns | 2.38 ns |
| `sphere_sphere_overlap` | — | 1.92 ns | 1.89 ns |
| `frustum_contains_point` | — | 4.79 ns | 4.87 ns |
| `frustum_contains_aabb` | — | 4.45 ns | 4.39 ns |
| `slerp` | — | 20.43 ns | 18.29 ns |
| `transform3d_lerp` | — | 25.29 ns | 21.45 ns |
| `closest_on_aabb` | — | 2.69 ns | 2.61 ns |
| `segment_closest_point` | — | 3.10 ns | 3.02 ns |
| `plane_plane_intersection` | — | 7.52 ns | 7.28 ns |
| `triangle_unit_normal` | — | 5.14 ns | 5.08 ns |
| `line_closest_point` | — | 2.16 ns | 2.16 ns |
| `closest_on_sphere` | — | 5.19 ns | 5.11 ns |
| `inverse_matrix` | — | 19.35 ns | 19.10 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.98 ns | 2.81 ns |
| `de_casteljau_split` | — | 6.16 ns | 6.01 ns |
| `catmull_rom` | — | 2.62 ns | 2.67 ns |
| `bspline_cubic` | — | 23.30 ns | 23.50 ns |
| `gauss_legendre_5` | — | 3.62 ns | 3.61 ns |
| `gauss_legendre_10_panels` | — | 440.4 ns | 432.4 ns |
| `arc_length_100` | — | 602.1 ns | 626.8 ns |
| `ease_in_out` | — | 0.59 ns | 0.61 ns |
| `ease_in_out_smooth` | — | 0.82 ns | 0.81 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 119.1 ns | 109.3 ns |
| `lu_solve_3x3` | — | 33.80 ns | 33.66 ns |
| `cholesky_3x3` | — | 78.11 ns | 65.49 ns |
| `cholesky_solve_3x3` | — | 38.32 ns | 39.10 ns |
| `qr_decompose_3col` | — | 127.9 ns | 130.3 ns |
| `least_squares_linear_6pt` | — | 182.2 ns | 187.5 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | — | 410.2 ns |
| `fft_64` | — | — | 622.0 ns |
| `fft_1024` | — | — | 15547.0 ns |
| `fft_ifft_256` | — | — | 6364.1 ns |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2829.6 ns |
| `rk4_exp_1000_steps` | — | — | 25674.0 ns |
| `rk4_oscillator_1000` | — | — | 27154.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 7834.9 ns |
| `bvh_ray_query_100` | — | — | 63.42 ns |
| `bvh_build_1000` | — | — | 98632.0 ns |
| `kdtree_build_1000` | — | — | 109520.0 ns |
| `kdtree_nearest_1000` | — | — | 252.0 ns |
| `kdtree_radius_1000` | — | — | 1463.7 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 71660.0 ns |
| `quadtree_query_1000` | — | — | 393.3 ns |
| `octree_insert_1000` | — | — | 83657.0 ns |
| `octree_query_1000` | — | — | 715.0 ns |
| `spatial_hash_insert_1000` | — | — | 37742.0 ns |
| `spatial_hash_query_cell` | — | — | 22.13 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`8f56bfa`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 1926.2 ns |
| `gjk_intersect` | — | — | 36.98 ns |
| `gjk_no_intersect` | — | — | 27.23 ns |
| `gjk_epa_penetration` | — | — | 327.8 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
