# Benchmarks

Latest: **2026-03-25T03:58:51Z** — commit `5c6b02a`

Tracking: `9463cf0` (baseline) → `2acbf1f` (optimized) → `5c6b02a` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.11 ns **-12%** | 6.62 ns **-5%** |
| `transform2d_apply_point` | 12.18 ns | 6.05 ns **-50%** | 6.61 ns **-46%** |
| `transform3d_to_matrix` | 11.20 ns | 5.13 ns **-54%** | 5.60 ns **-50%** |
| `transform3d_apply_point` | 13.71 ns | 5.87 ns **-57%** | 6.42 ns **-53%** |
| `projection_perspective` | 13.99 ns | 21.92 ns +57% | 12.75 ns **-9%** |
| `projection_orthographic` | 4.29 ns | 3.99 ns **-7%** | 4.50 ns +5% |
| `lerp_f32` | 1.05 ns | 1.00 ns **-5%** | 1.08 ns |
| `lerp_vec3` | 2.71 ns | 2.62 ns **-3%** | 2.80 ns +3% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.70 ns **-51%** | 2.93 ns **-46%** |
| `ray_plane_hit` | 2.10 ns | 1.96 ns **-7%** | 2.12 ns |
| `ray_aabb_hit` | 4.52 ns | 4.74 ns +5% | 4.91 ns +9% |
| `ray_sphere_miss` | 3.40 ns | 2.01 ns **-41%** | 2.05 ns **-40%** |
| `aabb_contains` | 3.05 ns | 2.82 ns **-7%** | 3.05 ns |
| `sphere_contains` | 2.31 ns | 2.18 ns **-6%** | 2.38 ns +3% |
| `aabb_merge` | 4.13 ns | 3.90 ns **-6%** | 4.15 ns |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.07 ns **-4%** | 1.16 ns +5% |
| `integral_simpson_100` | 78.23 ns | 75.91 ns | 82.88 ns +6% |
| `integral_simpson_1000` | 770.1 ns | 744.1 ns **-3%** | 817.4 ns +6% |
| `integral_trapezoidal_100` | 78.23 ns | 68.98 ns **-12%** | 75.72 ns **-3%** |
| `integral_trapezoidal_1000` | 753.4 ns | 706.3 ns **-6%** | 776.5 ns +3% |
| `bezier_quadratic` | 1.49 ns | 1.42 ns **-4%** | 1.53 ns +3% |
| `bezier_cubic` | 2.36 ns | 2.50 ns +6% | 2.72 ns +15% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.03 ns **-9%** | 6.66 ns |
| `bisection_sqrt2` | 113.2 ns | 102.4 ns **-9%** | 112.2 ns |
| `gaussian_3x3` | 83.64 ns | 75.55 ns **-10%** | 80.79 ns **-3%** |
| `gaussian_4x4` | 121.1 ns | 101.8 ns **-16%** | 113.6 ns **-6%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 185.2 ns | 201.5 ns |
| `aabb_contains_100` | — | 125.9 ns | 135.8 ns |
| `transform3d_batch_100` | — | 350.7 ns | 388.2 ns |
| `simpson_sin_10000` | — | 78046.0 ns | 84970.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.30 ns | 7.95 ns |
| `aabb_aabb_overlap` | — | 2.19 ns | 2.37 ns |
| `sphere_sphere_overlap` | — | 1.75 ns | 1.94 ns |
| `frustum_contains_point` | — | 4.38 ns | 4.80 ns |
| `frustum_contains_aabb` | — | 4.18 ns | 4.54 ns |
| `slerp` | — | 19.15 ns | 21.20 ns |
| `transform3d_lerp` | — | 23.25 ns | 24.86 ns |
| `closest_on_aabb` | — | 2.51 ns | 2.72 ns |
| `segment_closest_point` | — | 2.90 ns | 3.12 ns |
| `plane_plane_intersection` | — | 7.01 ns | 7.69 ns |
| `triangle_unit_normal` | — | 4.86 ns | 5.63 ns |
| `line_closest_point` | — | 2.07 ns | 2.22 ns |
| `closest_on_sphere` | — | 4.90 ns | 5.34 ns |
| `inverse_matrix` | — | 18.28 ns | 20.00 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.79 ns | 2.99 ns |
| `de_casteljau_split` | — | 5.66 ns | 6.17 ns |
| `catmull_rom` | — | 2.47 ns | 2.75 ns |
| `bspline_cubic` | — | 21.75 ns | 16.24 ns |
| `gauss_legendre_5` | — | 3.26 ns | 3.73 ns |
| `gauss_legendre_10_panels` | — | 395.2 ns | 431.8 ns |
| `arc_length_100` | — | 543.6 ns | 642.0 ns |
| `ease_in_out` | — | 0.55 ns | 0.60 ns |
| `ease_in_out_smooth` | — | 0.77 ns | 0.83 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 103.0 ns | 119.5 ns |
| `lu_solve_3x3` | — | 33.46 ns | 33.30 ns |
| `cholesky_3x3` | — | 61.48 ns | 66.01 ns |
| `cholesky_solve_3x3` | — | 36.19 ns | 39.26 ns |
| `qr_decompose_3col` | — | 121.5 ns | 137.5 ns |
| `least_squares_linear_6pt` | — | 179.6 ns | 200.5 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 404.0 ns | 436.0 ns |
| `fft_64` | — | 617.7 ns | 671.9 ns |
| `fft_1024` | — | 15389.0 ns | 16616.0 ns |
| `fft_ifft_256` | — | 6258.7 ns | 6781.9 ns |
| `dst_64` | — | — | 41455.0 ns |
| `dct_64` | — | — | 43002.0 ns |
| `dst_idst_256` | — | — | 1462.8 µs |
| `dct_idct_256` | — | — | 1499.7 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2603.3 ns |
| `rk4_exp_1000_steps` | — | — | 25564.0 ns |
| `rk4_oscillator_1000` | — | — | 27851.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 8557.6 ns |
| `bvh_ray_query_100` | — | — | 69.59 ns |
| `bvh_build_1000` | — | — | 100400.0 ns |
| `kdtree_build_1000` | — | — | 114350.0 ns |
| `kdtree_nearest_1000` | — | — | 267.4 ns |
| `kdtree_radius_1000` | — | — | 1599.4 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 76036.0 ns |
| `quadtree_query_1000` | — | — | 428.8 ns |
| `octree_insert_1000` | — | — | 86177.0 ns |
| `octree_query_1000` | — | — | 743.3 ns |
| `spatial_hash_insert_1000` | — | — | 39439.0 ns |
| `spatial_hash_query_cell` | — | — | 23.73 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2025.5 ns |
| `gjk_intersect` | — | — | 27.24 ns |
| `gjk_no_intersect` | — | — | 22.74 ns |
| `gjk_epa_penetration` | — | — | 259.3 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`5c6b02a`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 785.8 ns |
| `svd_5x5` | — | — | 124410.0 ns |
| `matrix_inverse_3x3` | — | — | 243.6 ns |
| `pseudo_inverse_3x2` | — | — | 620.5 ns |
| `csr_spmv_100x100` | — | — | 233.7 ns |
| `svd_4x2_tall` | — | — | 549.5 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
