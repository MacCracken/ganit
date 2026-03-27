# Benchmarks

Latest: **2026-03-27T20:32:34Z** — commit `1cf5fa3`

Tracking: `9463cf0` (baseline) → `5b4b707` (optimized) → `1cf5fa3` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.08 ns **-13%** | 6.96 ns |
| `transform2d_apply_point` | 12.18 ns | 6.15 ns **-50%** | 6.78 ns **-44%** |
| `transform3d_to_matrix` | 11.20 ns | 5.16 ns **-54%** | 6.23 ns **-44%** |
| `transform3d_apply_point` | 13.71 ns | 5.93 ns **-57%** | 7.25 ns **-47%** |
| `projection_perspective` | 13.99 ns | 12.82 ns **-8%** | 15.00 ns +7% |
| `projection_orthographic` | 4.29 ns | 4.00 ns **-7%** | 5.04 ns +18% |
| `lerp_f32` | 1.05 ns | 0.99 ns **-6%** | 1.24 ns +18% |
| `lerp_vec3` | 2.71 ns | 2.58 ns **-5%** | 3.18 ns +17% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.69 ns **-51%** | 3.34 ns **-39%** |
| `ray_plane_hit` | 2.10 ns | 2.00 ns **-5%** | 2.34 ns +12% |
| `ray_aabb_hit` | 4.52 ns | 4.56 ns | 5.44 ns +20% |
| `ray_sphere_miss` | 3.40 ns | 1.96 ns **-42%** | 2.42 ns **-29%** |
| `aabb_contains` | 3.05 ns | 2.88 ns **-6%** | 3.37 ns +11% |
| `sphere_contains` | 2.31 ns | 2.25 ns | 2.69 ns +17% |
| `aabb_merge` | 4.13 ns | 4.06 ns | 4.46 ns +8% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.09 ns | 1.20 ns +8% |
| `integral_simpson_100` | 78.23 ns | 77.50 ns | 83.78 ns +7% |
| `integral_simpson_1000` | 770.1 ns | 756.0 ns | 871.5 ns +13% |
| `integral_trapezoidal_100` | 78.23 ns | 80.23 ns | 78.52 ns |
| `integral_trapezoidal_1000` | 753.4 ns | 719.8 ns **-4%** | 824.8 ns +9% |
| `bezier_quadratic` | 1.49 ns | 1.43 ns **-4%** | 1.69 ns +14% |
| `bezier_cubic` | 2.36 ns | 2.23 ns **-5%** | 2.54 ns +8% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.15 ns **-7%** | 6.89 ns +4% |
| `bisection_sqrt2` | 113.2 ns | 104.1 ns **-8%** | 118.1 ns +4% |
| `gaussian_3x3` | 83.64 ns | 80.47 ns **-4%** | 84.61 ns |
| `gaussian_4x4` | 121.1 ns | 105.2 ns **-13%** | 120.4 ns |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 190.7 ns | 217.9 ns |
| `aabb_contains_100` | — | 127.5 ns | 141.3 ns |
| `transform3d_batch_100` | — | 360.2 ns | 405.7 ns |
| `simpson_sin_10000` | — | 80416.0 ns | 91360.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.71 ns | 8.86 ns |
| `aabb_aabb_overlap` | — | 2.26 ns | 2.62 ns |
| `sphere_sphere_overlap` | — | 1.76 ns | 2.02 ns |
| `frustum_contains_point` | — | 4.39 ns | 5.01 ns |
| `frustum_contains_aabb` | — | 4.27 ns | 4.75 ns |
| `slerp` | — | 19.08 ns | 22.09 ns |
| `transform3d_lerp` | — | 20.36 ns | 23.73 ns |
| `closest_on_aabb` | — | 2.51 ns | 3.06 ns |
| `segment_closest_point` | — | 2.88 ns | 3.46 ns |
| `plane_plane_intersection` | — | 6.98 ns | 8.27 ns |
| `triangle_unit_normal` | — | 4.87 ns | 6.09 ns |
| `line_closest_point` | — | 2.05 ns | 2.37 ns |
| `closest_on_sphere` | — | 4.91 ns | 5.51 ns |
| `inverse_matrix` | — | 18.23 ns | 20.29 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.74 ns | 3.14 ns |
| `de_casteljau_split` | — | 5.73 ns | 6.65 ns |
| `catmull_rom` | — | 2.43 ns | 3.00 ns |
| `bspline_cubic` | — | 21.89 ns | 18.38 ns |
| `gauss_legendre_5` | — | 3.27 ns | 4.41 ns |
| `gauss_legendre_10_panels` | — | 395.3 ns | 449.3 ns |
| `arc_length_100` | — | 540.3 ns | 673.0 ns |
| `ease_in_out` | — | 0.55 ns | 0.60 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 0.85 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 106.9 ns | 114.3 ns |
| `lu_solve_3x3` | — | 32.06 ns | 33.24 ns |
| `cholesky_3x3` | — | 62.71 ns | 66.63 ns |
| `cholesky_solve_3x3` | — | 37.47 ns | 39.25 ns |
| `qr_decompose_3col` | — | 126.0 ns | 141.2 ns |
| `least_squares_linear_6pt` | — | 182.3 ns | 186.5 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 408.0 ns | 432.9 ns |
| `fft_64` | — | 615.8 ns | 664.1 ns |
| `fft_1024` | — | 15241.0 ns | 16482.0 ns |
| `fft_ifft_256` | — | 6269.4 ns | 6745.7 ns |
| `dst_64` | — | — | 41084.0 ns |
| `dct_64` | — | — | 42615.0 ns |
| `dst_idst_256` | — | — | 1447.3 µs |
| `dct_idct_256` | — | — | 1487.5 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 5563.5 ns | 2580.2 ns |
| `rk4_exp_1000_steps` | — | 54750.0 ns | 25425.0 ns |
| `rk4_oscillator_1000` | — | 62798.0 ns | 27792.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 8157.3 ns |
| `bvh_ray_query_100` | — | — | 69.36 ns |
| `bvh_build_1000` | — | — | 101390.0 ns |
| `kdtree_build_1000` | — | — | 116880.0 ns |
| `kdtree_nearest_1000` | — | — | 272.3 ns |
| `kdtree_radius_1000` | — | — | 1579.1 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 76698.0 ns |
| `quadtree_query_1000` | — | — | 421.6 ns |
| `octree_insert_1000` | — | — | 86895.0 ns |
| `octree_query_1000` | — | — | 751.0 ns |
| `spatial_hash_insert_1000` | — | — | 38603.0 ns |
| `spatial_hash_query_cell` | — | — | 23.89 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2046.3 ns |
| `gjk_intersect` | — | — | 27.64 ns |
| `gjk_no_intersect` | — | — | 23.47 ns |
| `gjk_epa_penetration` | — | — | 154.2 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`5b4b707`) | Current (`1cf5fa3`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 764.9 ns |
| `svd_5x5` | — | — | 123620.0 ns |
| `matrix_inverse_3x3` | — | — | 242.1 ns |
| `pseudo_inverse_3x2` | — | — | 601.5 ns |
| `csr_spmv_100x100` | — | — | 211.6 ns |
| `svd_4x2_tall` | — | — | 564.7 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
