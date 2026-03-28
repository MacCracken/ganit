# Benchmarks

Latest: **2026-03-28T00:34:50Z** — commit `72ba090`

Tracking: `9463cf0` (baseline) → `7f26a9a` (optimized) → `72ba090` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 5.97 ns **-15%** | 15.00 ns +115% |
| `transform2d_apply_point` | 12.18 ns | 6.12 ns **-50%** | 13.84 ns +14% |
| `transform3d_to_matrix` | 11.20 ns | 5.19 ns **-54%** | 10.14 ns **-10%** |
| `transform3d_apply_point` | 13.71 ns | 5.87 ns **-57%** | 13.15 ns **-4%** |
| `projection_perspective` | 13.99 ns | 13.98 ns | 32.61 ns +133% |
| `projection_orthographic` | 4.29 ns | 4.07 ns **-5%** | 8.87 ns +107% |
| `lerp_f32` | 1.05 ns | 1.01 ns **-4%** | 2.22 ns +111% |
| `lerp_vec3` | 2.71 ns | 2.61 ns **-4%** | 3.70 ns +36% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.71 ns **-51%** | 4.42 ns **-19%** |
| `ray_plane_hit` | 2.10 ns | 1.97 ns **-6%** | 2.84 ns +36% |
| `ray_aabb_hit` | 4.52 ns | 4.74 ns +5% | 5.65 ns +25% |
| `ray_sphere_miss` | 3.40 ns | 1.91 ns **-44%** | 3.72 ns +10% |
| `aabb_contains` | 3.05 ns | 2.85 ns **-7%** | 4.24 ns +39% |
| `sphere_contains` | 2.31 ns | 2.20 ns **-5%** | 4.72 ns +105% |
| `aabb_merge` | 4.13 ns | 3.79 ns **-8%** | 6.90 ns +67% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.06 ns **-4%** | 1.92 ns +72% |
| `integral_simpson_100` | 78.23 ns | 75.83 ns **-3%** | 104.0 ns +33% |
| `integral_simpson_1000` | 770.1 ns | 749.5 ns | 948.3 ns +23% |
| `integral_trapezoidal_100` | 78.23 ns | 68.92 ns **-12%** | 86.88 ns +11% |
| `integral_trapezoidal_1000` | 753.4 ns | 708.4 ns **-6%** | 920.6 ns +22% |
| `bezier_quadratic` | 1.49 ns | 1.41 ns **-5%** | 2.53 ns +70% |
| `bezier_cubic` | 2.36 ns | 2.29 ns | 4.18 ns +77% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.11 ns **-8%** | 7.42 ns +12% |
| `bisection_sqrt2` | 113.2 ns | 103.1 ns **-9%** | 133.3 ns +18% |
| `gaussian_3x3` | 83.64 ns | 79.98 ns **-4%** | 103.4 ns +24% |
| `gaussian_4x4` | 121.1 ns | 104.3 ns **-14%** | 161.0 ns +33% |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 185.9 ns | 246.0 ns |
| `aabb_contains_100` | — | 124.1 ns | 151.4 ns |
| `transform3d_batch_100` | — | 349.1 ns | 501.9 ns |
| `simpson_sin_10000` | — | 78076.0 ns | 104430.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.26 ns | 9.02 ns |
| `aabb_aabb_overlap` | — | 2.18 ns | 3.45 ns |
| `sphere_sphere_overlap` | — | 1.75 ns | 2.50 ns |
| `frustum_contains_point` | — | 4.39 ns | 5.52 ns |
| `frustum_contains_aabb` | — | 4.16 ns | 5.24 ns |
| `slerp` | — | 17.21 ns | 22.86 ns |
| `transform3d_lerp` | — | 20.22 ns | 26.56 ns |
| `closest_on_aabb` | — | 2.50 ns | 3.23 ns |
| `segment_closest_point` | — | 2.88 ns | 3.30 ns |
| `plane_plane_intersection` | — | 6.93 ns | 7.76 ns |
| `triangle_unit_normal` | — | 4.86 ns | 5.54 ns |
| `line_closest_point` | — | 2.17 ns | 2.33 ns |
| `closest_on_sphere` | — | 4.99 ns | 5.40 ns |
| `inverse_matrix` | — | 18.30 ns | 20.09 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.73 ns | 3.10 ns |
| `de_casteljau_split` | — | 5.65 ns | 6.22 ns |
| `catmull_rom` | — | 2.62 ns | 2.65 ns |
| `bspline_cubic` | — | 22.00 ns | 16.72 ns |
| `gauss_legendre_5` | — | 3.32 ns | 3.84 ns |
| `gauss_legendre_10_panels` | — | 399.7 ns | 432.0 ns |
| `arc_length_100` | — | 550.0 ns | 644.3 ns |
| `ease_in_out` | — | 0.55 ns | 0.61 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 0.85 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 110.5 ns | 123.1 ns |
| `lu_solve_3x3` | — | 32.29 ns | 35.67 ns |
| `cholesky_3x3` | — | 62.49 ns | 72.23 ns |
| `cholesky_solve_3x3` | — | 37.68 ns | 39.67 ns |
| `qr_decompose_3col` | — | 127.3 ns | 139.6 ns |
| `least_squares_linear_6pt` | — | 177.1 ns | 192.9 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 402.1 ns | 445.8 ns |
| `fft_64` | — | 607.6 ns | 667.4 ns |
| `fft_1024` | — | 15166.0 ns | 16635.0 ns |
| `fft_ifft_256` | — | 6205.3 ns | 6777.2 ns |
| `dst_64` | — | — | 41777.0 ns |
| `dct_64` | — | — | 43313.0 ns |
| `dst_idst_256` | — | — | 1474.9 µs |
| `dct_idct_256` | — | — | 1543.6 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 2527.8 ns | 2594.6 ns |
| `rk4_exp_1000_steps` | — | 25193.0 ns | 25537.0 ns |
| `rk4_oscillator_1000` | — | 24136.0 ns | 27877.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `bvh_build_100` | — | 10695.0 ns | 8247.2 ns |
| `bvh_ray_query_100` | — | 62.08 ns | 86.60 ns |
| `bvh_build_1000` | — | 141510.0 ns | 107320.0 ns |
| `kdtree_build_1000` | — | 139570.0 ns | 113800.0 ns |
| `kdtree_nearest_1000` | — | 265.8 ns | 280.8 ns |
| `kdtree_radius_1000` | — | 1401.1 ns | 1711.0 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 76084.0 ns |
| `quadtree_query_1000` | — | — | 445.6 ns |
| `octree_insert_1000` | — | — | 90469.0 ns |
| `octree_query_1000` | — | — | 820.6 ns |
| `spatial_hash_insert_1000` | — | — | 40222.0 ns |
| `spatial_hash_query_cell` | — | — | 23.68 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2099.8 ns |
| `gjk_intersect` | — | — | 28.19 ns |
| `gjk_no_intersect` | — | — | 23.29 ns |
| `gjk_epa_penetration` | — | — | 155.5 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`72ba090`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 798.1 ns |
| `svd_5x5` | — | — | 131120.0 ns |
| `matrix_inverse_3x3` | — | — | 311.2 ns |
| `pseudo_inverse_3x2` | — | — | 671.1 ns |
| `csr_spmv_100x100` | — | — | 219.5 ns |
| `svd_4x2_tall` | — | — | 575.3 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
