# Benchmarks

> **Measurement changes** — read before comparing across a date.
> * **2026-08-21** (hisab 2.11.2, cyrius 6.5.18 → 6.5.33): `lib/bench.cyr` now
>   MEASURES one clock read on the host and subtracts it from every sample, and
>   `bench_run` sizes its own batches instead of wrapping a clock pair around every
>   iteration. **No hisab source changed.** The floor is re-measured every run and
>   recorded per row in `floor_ns` (~1,340–1,350 ns on this host; upstream measured a
>   230x spread across its four gate hosts, and it moves between reboots, so it is
>   not a constant and is not written down as one).
>   `ease_in_out` 1,407 ns → 7 ns, `cx_mul` 1,459 → 37, `quat_mul` 1,473 → 67,
>   `perlin_2d` 1,461 → 82 — those four were 94–100% instrument. The old numbers
>   also FLATTENED them: four operations spanning **149x** in reality were reported
>   within **1.26x** of each other, so a real regression had room to hide. 44 of 72
>   rows moved more than 10%; **none of it is a speedup**. Rows carry `regime=net`
>   from this date and the trend filter refuses to mix them with `raw` ones.
>   ⚠ `min_ns`/`max_ns` also changed meaning for anything `bench_run` chose to batch:
>   they are now per-CHUNK averages, not per-iteration extremes. That is the honest
>   reading — a single sub-floor iteration has no measurable duration — but it means
>   the spread narrows for reasons unrelated to the code. `estimate_ns` (the avg) is
>   unaffected and remains the column the trend is built on.
> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to
>   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call
>   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock
>   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns
>   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the
>   true ratio is 3.6x — so a real regression could hide inside the overhead.
>   Their drop at this date is an artefact of the fix, not a speedup.
> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.
>   The `stat` column records which; rows with no value there are `max`.

Latest: **2026-09-10T23:33:44Z** — commit `8924440`

Tracking: `a09d228` (baseline) → `8387ff8` (mid) → `8924440` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 17.00 ns **-26%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 30.00 ns **-52%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 31.00 ns **-22%** | 36.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 381.0 ns **-36%** | 401.0 ns **-32%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 284.0 ns **-13%** | 317.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1906.0 ns **-12%** | 2246.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2731.0 ns **-16%** | 3240.0 ns |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 46.00 ns **-27%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 239.0 ns | 268.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 48.00 ns **-24%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 119.0 ns | 147.0 ns +11% |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 248.0 ns | 321.0 ns +24% |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 92.00 ns **-25%** | 109.0 ns **-11%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 150.0 ns **-31%** | 191.0 ns **-12%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 192.0 ns **-29%** | 226.0 ns **-17%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 101.0 ns **-42%** | 119.0 ns **-31%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 460.0 ns **-40%** | 517.0 ns **-33%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 178.0 ns **-39%** | 185.0 ns **-36%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 538.0 ns **-34%** | 558.0 ns **-32%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 697.0 ns **-30%** | 712.0 ns **-28%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43581.0 ns **-25%** | 45170.0 ns **-22%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4450.0 ns **-24%** | 4684.0 ns **-20%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 279.0 ns **-37%** | 283.0 ns **-36%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 516.0 ns **-30%** | 525.0 ns **-29%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 379.0 ns **-29%** | 390.0 ns **-27%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 57.00 ns **-37%** | 59.00 ns **-34%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 41.00 ns **-56%** | 43.00 ns **-54%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 136.0 ns **-45%** | 145.0 ns **-41%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 98.00 ns +20% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 23.00 ns **-23%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 90.00 ns | 93.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4814.0 ns | 4999.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 24.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19091.0 ns | 20298.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 24.00 ns **-31%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 73.00 ns | 76.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1682.0 µs **-12%** | 1675.0 µs **-12%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 721699.0 ns **-13%** | 734924.0 ns **-11%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2274.0 µs **-31%** | 2361.0 µs **-29%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 925245.0 ns **-48%** | 987275.0 ns **-44%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2392.0 µs | 2362.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 527142.0 ns | 543241.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 953.0 ns **-14%** | 979.0 ns **-11%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 433.0 ns | 439.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 663.0 ns **-36%** | 663.0 ns **-36%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1382.0 µs **-13%** | 1447.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 356356.0 ns | 372065.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1539.0 µs | 1595.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5240.0 µs | 5387.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1180.0 ns **-19%** | 1207.0 ns **-17%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 117653.0 ns **-26%** | 118962.0 ns **-25%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 88850.0 ns **-26%** | 90107.0 ns **-24%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 81090.0 ns **-26%** | 82876.0 ns **-24%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12613.0 ns **-27%** | 13877.0 ns **-20%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 12571.0 ns **-28%** | 13651.0 ns **-22%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 399850.0 ns **-25%** | 424455.0 ns **-21%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79750.0 ns **-28%** | 82473.0 ns **-26%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 458.0 ns **-31%** | 463.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 669.0 ns **-26%** | 678.0 ns **-25%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1010.0 ns **-37%** | 1031.0 ns **-36%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1151.0 ns **-37%** | 1164.0 ns **-37%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 954.0 ns **-30%** | 979.0 ns **-29%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3363.0 ns **-31%** | 3451.0 ns **-30%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3473.0 ns **-30%** | 3561.0 ns **-29%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4666.0 µs **-20%** | 4853.0 µs **-16%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 363944.0 ns **-15%** | 381486.0 ns **-11%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 269476.0 ns | 282521.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6464.0 µs | 6633.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1557.0 µs | 1661.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`8924440`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 431040.0 ns | 466001.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
