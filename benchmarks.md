# Benchmarks

Latest: **2026-03-27T20:10:18Z** — commit `1cf5fa3`

Tracking: `9463cf0` (baseline) → `5b4b707` (optimized) → `1cf5fa3` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.08 ns **-13%** | 6.66 ns **-5%** |
| `transform2d_apply_point` | 12.18 ns | 6.15 ns **-50%** | 6.65 ns **-45%** |
| `transform3d_to_matrix` | 11.20 ns | 5.16 ns **-54%** | 5.62 ns **-50%** |
| `transform3d_apply_point` | 13.71 ns | 5.93 ns **-57%** | 6.43 ns **-53%** |
| `projection_perspective` | 13.99 ns | 12.82 ns **-8%** | 12.94 ns **-8%** |
| `projection_orthographic` | 4.29 ns | 4.00 ns **-7%** | 4.56 ns +6% |
| `lerp_f32` | 1.05 ns | 0.99 ns **-6%** | 1.08 ns +3% |
| `lerp_vec3` | 2.71 ns | 2.58 ns **-5%** | 2.82 ns +4% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.69 ns **-51%** | 2.95 ns **-46%** |
| `ray_plane_hit` | 2.10 ns | 2.00 ns **-5%** | 2.14 ns |
| `ray_aabb_hit` | 4.52 ns | 4.56 ns | 4.98 ns +10% |
| `ray_sphere_miss` | 3.40 ns | 1.96 ns **-42%** | 2.21 ns **-35%** |
| `aabb_contains` | 3.05 ns | 2.88 ns **-6%** | 3.07 ns |
| `sphere_contains` | 2.31 ns | 2.25 ns | 2.40 ns +4% |
| `aabb_merge` | 4.13 ns | 4.06 ns | 4.38 ns +6% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.09 ns | 1.17 ns +6% |
| `integral_simpson_100` | 78.23 ns | 77.50 ns | 83.27 ns +6% |
| `integral_simpson_1000` | 770.1 ns | 756.0 ns | 785.8 ns |
| `integral_trapezoidal_100` | 78.23 ns | 80.23 ns | 76.73 ns |
| `integral_trapezoidal_1000` | 753.4 ns | 719.8 ns **-4%** | 776.9 ns +3% |
| `bezier_quadratic` | 1.49 ns | 1.43 ns **-4%** | 1.58 ns +7% |
| `bezier_cubic` | 2.36 ns | 2.23 ns **-5%** | 2.55 ns +8% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.15 ns **-7%** | 6.71 ns |
| `bisection_sqrt2` | 113.2 ns | 104.1 ns **-8%** | 114.2 ns |
| `gaussian_3x3` | 83.64 ns | 80.47 ns **-4%** | 80.53 ns **-4%** |
| `gaussian_4x4` | 121.1 ns | 105.2 ns **-13%** | 113.7 ns **-6%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 190.7 ns | 216.6 ns |
| `aabb_contains_100` | — | 127.5 ns | 137.5 ns |
| `transform3d_batch_100` | — | 360.2 ns | 392.4 ns |
| `simpson_sin_10000` | — | 80416.0 ns | 92494.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.71 ns | 8.40 ns |
| `aabb_aabb_overlap` | — | 2.26 ns | 2.43 ns |
| `sphere_sphere_overlap` | — | 1.76 ns | 1.90 ns |
| `frustum_contains_point` | — | 4.39 ns | 4.87 ns |
| `frustum_contains_aabb` | — | 4.27 ns | 6.41 ns |
| `slerp` | — | 19.08 ns | 21.81 ns |
| `transform3d_lerp` | — | 20.36 ns | 26.25 ns |
| `closest_on_aabb` | — | 2.51 ns | 2.91 ns |
| `segment_closest_point` | — | 2.88 ns | 3.36 ns |
| `plane_plane_intersection` | — | 6.98 ns | 8.87 ns |
| `triangle_unit_normal` | — | 4.87 ns | 6.43 ns |
| `line_closest_point` | — | 2.05 ns | 2.47 ns |
| `closest_on_sphere` | — | 4.91 ns | 5.79 ns |
| `inverse_matrix` | — | 18.23 ns | 27.73 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.74 ns | 7.15 ns |
| `de_casteljau_split` | — | 5.73 ns | 7.74 ns |
| `catmull_rom` | — | 2.43 ns | 4.94 ns |
| `bspline_cubic` | — | 21.89 ns | 30.92 ns |
| `gauss_legendre_5` | — | 3.27 ns | 5.54 ns |
| `gauss_legendre_10_panels` | — | 395.3 ns | 712.6 ns |
| `arc_length_100` | — | 540.3 ns | 1039.9 ns |
| `ease_in_out` | — | 0.55 ns | 1.26 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 1.56 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 106.9 ns | 167.8 ns |
| `lu_solve_3x3` | — | 32.06 ns | 49.78 ns |
| `cholesky_3x3` | — | 62.71 ns | 121.9 ns |
| `cholesky_solve_3x3` | — | 37.47 ns | 65.78 ns |
| `qr_decompose_3col` | — | 126.0 ns | 233.4 ns |
| `least_squares_linear_6pt` | — | 182.3 ns | 348.6 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 408.0 ns | 620.7 ns |
| `fft_64` | — | 615.8 ns | 1078.4 ns |
| `fft_1024` | — | 15241.0 ns | 23780.0 ns |
| `fft_ifft_256` | — | 6269.4 ns | 10158.0 ns |
| `dst_64` | — | — | 43857.0 ns |
| `dct_64` | — | — | 46690.0 ns |
| `dst_idst_256` | — | — | 1556.1 µs |
| `dct_idct_256` | — | — | 1630.5 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 5563.5 ns | 2759.4 ns |
| `rk4_exp_1000_steps` | — | 54750.0 ns | 27306.0 ns |
| `rk4_oscillator_1000` | — | 62798.0 ns | 29530.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 9075.0 ns |
| `bvh_ray_query_100` | — | — | 71.10 ns |
| `bvh_build_1000` | — | — | 111060.0 ns |
| `kdtree_build_1000` | — | — | 119100.0 ns |
| `kdtree_nearest_1000` | — | — | 285.8 ns |
| `kdtree_radius_1000` | — | — | 1617.1 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 83044.0 ns |
| `quadtree_query_1000` | — | — | 479.5 ns |
| `octree_insert_1000` | — | — | 95031.0 ns |
| `octree_query_1000` | — | — | 881.2 ns |
| `spatial_hash_insert_1000` | — | — | 42347.0 ns |
| `spatial_hash_query_cell` | — | — | 25.65 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2269.7 ns |
| `gjk_intersect` | — | — | 30.23 ns |
| `gjk_no_intersect` | — | — | 24.63 ns |
| `gjk_epa_penetration` | — | — | 162.8 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 878.2 ns |
| `svd_5x5` | — | — | 135020.0 ns |
| `matrix_inverse_3x3` | — | — | 271.8 ns |
| `pseudo_inverse_3x2` | — | — | 668.7 ns |
| `csr_spmv_100x100` | — | — | 265.5 ns |
| `svd_4x2_tall` | — | — | 596.9 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
