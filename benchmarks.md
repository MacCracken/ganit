# Benchmarks

Latest: **2026-03-25T03:04:21Z** — commit `01fa97c`

Tracking: `9463cf0` (baseline) → `2acbf1f` (optimized) → `01fa97c` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.11 ns **-12%** | 7.71 ns +11% |
| `transform2d_apply_point` | 12.18 ns | 6.05 ns **-50%** | 7.44 ns **-39%** |
| `transform3d_to_matrix` | 11.20 ns | 5.13 ns **-54%** | 6.20 ns **-45%** |
| `transform3d_apply_point` | 13.71 ns | 5.87 ns **-57%** | 6.61 ns **-52%** |
| `projection_perspective` | 13.99 ns | 21.92 ns +57% | 13.65 ns |
| `projection_orthographic` | 4.29 ns | 3.99 ns **-7%** | 4.59 ns +7% |
| `lerp_f32` | 1.05 ns | 1.00 ns **-5%** | 1.14 ns +8% |
| `lerp_vec3` | 2.71 ns | 2.62 ns **-3%** | 2.95 ns +9% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.70 ns **-51%** | 3.06 ns **-44%** |
| `ray_plane_hit` | 2.10 ns | 1.96 ns **-7%** | 2.19 ns +4% |
| `ray_aabb_hit` | 4.52 ns | 4.74 ns +5% | 5.15 ns +14% |
| `ray_sphere_miss` | 3.40 ns | 2.01 ns **-41%** | 2.32 ns **-32%** |
| `aabb_contains` | 3.05 ns | 2.82 ns **-7%** | 3.38 ns +11% |
| `sphere_contains` | 2.31 ns | 2.18 ns **-6%** | 2.63 ns +14% |
| `aabb_merge` | 4.13 ns | 3.90 ns **-6%** | 4.21 ns |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.07 ns **-4%** | 1.17 ns +6% |
| `integral_simpson_100` | 78.23 ns | 75.91 ns | 83.31 ns +6% |
| `integral_simpson_1000` | 770.1 ns | 744.1 ns **-3%** | 849.8 ns +10% |
| `integral_trapezoidal_100` | 78.23 ns | 68.98 ns **-12%** | 76.18 ns |
| `integral_trapezoidal_1000` | 753.4 ns | 706.3 ns **-6%** | 815.5 ns +8% |
| `bezier_quadratic` | 1.49 ns | 1.42 ns **-4%** | 1.68 ns +13% |
| `bezier_cubic` | 2.36 ns | 2.50 ns +6% | 2.65 ns +12% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.03 ns **-9%** | 7.11 ns +7% |
| `bisection_sqrt2` | 113.2 ns | 102.4 ns **-9%** | 119.7 ns +6% |
| `gaussian_3x3` | 83.64 ns | 75.55 ns **-10%** | 86.13 ns |
| `gaussian_4x4` | 121.1 ns | 101.8 ns **-16%** | 124.0 ns |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 185.2 ns | 229.6 ns |
| `aabb_contains_100` | — | 125.9 ns | 149.1 ns |
| `transform3d_batch_100` | — | 350.7 ns | 437.0 ns |
| `simpson_sin_10000` | — | 78046.0 ns | 97243.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.30 ns | 8.93 ns |
| `aabb_aabb_overlap` | — | 2.19 ns | 2.72 ns |
| `sphere_sphere_overlap` | — | 1.75 ns | 2.15 ns |
| `frustum_contains_point` | — | 4.38 ns | 4.98 ns |
| `frustum_contains_aabb` | — | 4.18 ns | 5.78 ns |
| `slerp` | — | 19.15 ns | 23.97 ns |
| `transform3d_lerp` | — | 23.25 ns | 27.64 ns |
| `closest_on_aabb` | — | 2.51 ns | 3.03 ns |
| `segment_closest_point` | — | 2.90 ns | 3.57 ns |
| `plane_plane_intersection` | — | 7.01 ns | 14.39 ns |
| `triangle_unit_normal` | — | 4.86 ns | 7.76 ns |
| `line_closest_point` | — | 2.07 ns | 3.64 ns |
| `closest_on_sphere` | — | 4.90 ns | 8.78 ns |
| `inverse_matrix` | — | 18.28 ns | 24.95 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.79 ns | 3.81 ns |
| `de_casteljau_split` | — | 5.66 ns | 7.64 ns |
| `catmull_rom` | — | 2.47 ns | 4.66 ns |
| `bspline_cubic` | — | 21.75 ns | 25.97 ns |
| `gauss_legendre_5` | — | 3.26 ns | 4.81 ns |
| `gauss_legendre_10_panels` | — | 395.2 ns | 496.9 ns |
| `arc_length_100` | — | 543.6 ns | 704.0 ns |
| `ease_in_out` | — | 0.55 ns | 0.72 ns |
| `ease_in_out_smooth` | — | 0.77 ns | 0.91 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 103.0 ns | 129.8 ns |
| `lu_solve_3x3` | — | 33.46 ns | 37.25 ns |
| `cholesky_3x3` | — | 61.48 ns | 74.33 ns |
| `cholesky_solve_3x3` | — | 36.19 ns | 41.50 ns |
| `qr_decompose_3col` | — | 121.5 ns | 202.4 ns |
| `least_squares_linear_6pt` | — | 179.6 ns | 218.8 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 404.0 ns | 481.2 ns |
| `fft_64` | — | 617.7 ns | 831.4 ns |
| `fft_1024` | — | 15389.0 ns | 18410.0 ns |
| `fft_ifft_256` | — | 6258.7 ns | 7386.4 ns |
| `dst_64` | — | — | 44148.0 ns |
| `dct_64` | — | — | 67169.0 ns |
| `dst_idst_256` | — | — | 1948.1 µs |
| `dct_idct_256` | — | — | 1671.9 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2724.9 ns |
| `rk4_exp_1000_steps` | — | — | 26635.0 ns |
| `rk4_oscillator_1000` | — | — | 29200.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 9899.4 ns |
| `bvh_ray_query_100` | — | — | 70.69 ns |
| `bvh_build_1000` | — | — | 109260.0 ns |
| `kdtree_build_1000` | — | — | 125070.0 ns |
| `kdtree_nearest_1000` | — | — | 317.7 ns |
| `kdtree_radius_1000` | — | — | 1911.8 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 89361.0 ns |
| `quadtree_query_1000` | — | — | 521.2 ns |
| `octree_insert_1000` | — | — | 106830.0 ns |
| `octree_query_1000` | — | — | 931.9 ns |
| `spatial_hash_insert_1000` | — | — | 43647.0 ns |
| `spatial_hash_query_cell` | — | — | 27.93 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`2acbf1f`) | Current (`01fa97c`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2417.6 ns |
| `gjk_intersect` | — | — | 29.43 ns |
| `gjk_no_intersect` | — | — | 27.51 ns |
| `gjk_epa_penetration` | — | — | 312.4 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
