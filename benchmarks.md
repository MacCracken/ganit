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

Latest: **2026-09-09T23:00:13Z** — commit `8387ff8`

Tracking: `a09d228` (baseline) → `1066582` (mid) → `8387ff8` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 28.00 ns **-56%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 34.00 ns **-15%** | 31.00 ns **-22%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 498.0 ns **-16%** | 385.0 ns **-35%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 429.0 ns +32% | 286.0 ns **-12%** |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1914.0 ns **-12%** | 1916.0 ns **-12%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2732.0 ns **-16%** | 2737.0 ns **-16%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 241.0 ns | 241.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 40.00 ns **-37%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 120.0 ns | 120.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 245.0 ns | 246.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 92.00 ns **-25%** | 93.00 ns **-24%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 148.0 ns **-32%** | 148.0 ns **-32%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 191.0 ns **-30%** | 190.0 ns **-30%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 100.0 ns **-42%** | 99.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 459.0 ns **-41%** | 461.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 179.0 ns **-38%** | 181.0 ns **-38%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 538.0 ns **-34%** | 544.0 ns **-34%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 706.0 ns **-29%** | 698.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 44127.0 ns **-24%** | 43712.0 ns **-25%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4559.0 ns **-22%** | 4488.0 ns **-23%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 283.0 ns **-36%** | 279.0 ns **-37%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 65.00 ns **-43%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 524.0 ns **-29%** | 516.0 ns **-30%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 384.0 ns **-28%** | 382.0 ns **-29%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 59.00 ns **-34%** | 57.00 ns **-37%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 42.00 ns **-55%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 136.0 ns **-45%** | 138.0 ns **-44%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 96.00 ns +17% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 90.00 ns | 91.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4851.0 ns | 4826.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19198.0 ns | 19212.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 6.00 ns **-14%** |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 73.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1685.0 µs **-12%** | 1689.0 µs **-12%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 727508.0 ns **-12%** | 720542.0 ns **-13%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2296.0 µs **-31%** | 2287.0 µs **-31%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 936333.0 ns **-47%** | 923805.0 ns **-48%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2282.0 µs | 2293.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 529892.0 ns | 530976.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 977.0 ns **-11%** | 965.0 ns **-12%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 435.0 ns | 438.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 665.0 ns **-36%** | 660.0 ns **-36%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1453.0 µs | 1385.0 µs **-13%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 367737.0 ns | 356610.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1561.0 µs | 1552.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5239.0 µs | 5294.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1178.0 ns **-19%** | 1177.0 ns **-20%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 111445.0 ns **-30%** | 111310.0 ns **-30%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 83606.0 ns **-30%** | 84142.0 ns **-29%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 79634.0 ns **-27%** | 79840.0 ns **-27%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12643.0 ns **-27%** | 12689.0 ns **-27%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 12766.0 ns **-27%** | 12850.0 ns **-26%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 405251.0 ns **-24%** | 405432.0 ns **-24%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 78769.0 ns **-29%** | 78354.0 ns **-30%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 456.0 ns **-31%** | 462.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 659.0 ns **-27%** | 664.0 ns **-27%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1033.0 ns **-36%** | 1016.0 ns **-37%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1164.0 ns **-37%** | 1161.0 ns **-37%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 957.0 ns **-30%** | 959.0 ns **-30%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3402.0 ns **-31%** | 3415.0 ns **-30%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3543.0 ns **-29%** | 3578.0 ns **-28%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4721.0 µs **-19%** | 4681.0 µs **-19%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 362517.0 ns **-15%** | 366653.0 ns **-14%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 270634.0 ns | 272646.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6496.0 µs | 6614.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1556.0 µs | 1573.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`1066582`) | Current (`8387ff8`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 440300.0 ns | 435258.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
