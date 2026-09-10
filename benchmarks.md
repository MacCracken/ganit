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

Latest: **2026-09-10T21:39:05Z** — commit `5857854`

Tracking: `a09d228` (baseline) → `8387ff8` (mid) → `5857854` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 30.00 ns **-52%** | 28.00 ns **-56%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 31.00 ns **-22%** | 35.00 ns **-12%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 393.0 ns **-34%** | 380.0 ns **-36%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 292.0 ns **-10%** | 297.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1961.0 ns | 1909.0 ns **-12%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2809.0 ns **-14%** | 2717.0 ns **-17%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 247.0 ns | 240.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 39.00 ns **-38%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 122.0 ns | 118.0 ns **-11%** |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 253.0 ns | 244.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 95.00 ns **-22%** | 95.00 ns **-22%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 152.0 ns **-30%** | 151.0 ns **-30%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 196.0 ns **-28%** | 190.0 ns **-30%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 103.0 ns **-40%** | 100.0 ns **-42%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 474.0 ns **-39%** | 467.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 187.0 ns **-36%** | 178.0 ns **-39%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 554.0 ns **-32%** | 541.0 ns **-34%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 723.0 ns **-27%** | 691.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 45118.0 ns **-22%** | 43830.0 ns **-25%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4599.0 ns **-21%** | 4505.0 ns **-23%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 288.0 ns **-35%** | 282.0 ns **-36%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 66.00 ns **-43%** | 66.00 ns **-43%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 550.0 ns **-26%** | 511.0 ns **-31%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 398.0 ns **-26%** | 376.0 ns **-30%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 59.00 ns **-34%** | 58.00 ns **-36%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 43.00 ns **-54%** | 41.00 ns **-56%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 142.0 ns **-42%** | 141.0 ns **-43%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 97.00 ns +18% | 96.00 ns +17% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 23.00 ns **-23%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 93.00 ns | 91.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5002.0 ns | 4822.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19726.0 ns | 19690.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 25.00 ns **-29%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 76.00 ns | 74.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1728.0 µs | 1645.0 µs **-14%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 745820.0 ns | 730157.0 ns **-12%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2325.0 µs **-30%** | 2297.0 µs **-30%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 968625.0 ns **-45%** | 935245.0 ns **-47%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2354.0 µs | 2292.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 549755.0 ns | 529838.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 969.0 ns **-12%** | 955.0 ns **-13%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 445.0 ns | 418.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 687.0 ns **-34%** | 669.0 ns **-35%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1473.0 µs | 1385.0 µs **-13%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 374274.0 ns | 355401.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1649.0 µs | 1544.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5580.0 µs | 5259.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1239.0 ns **-15%** | 1184.0 ns **-19%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 117671.0 ns **-26%** | 111422.0 ns **-30%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 89403.0 ns **-25%** | 84593.0 ns **-29%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 84532.0 ns **-23%** | 80638.0 ns **-26%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13461.0 ns **-23%** | 12845.0 ns **-26%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13547.0 ns **-22%** | 12905.0 ns **-26%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 422191.0 ns **-21%** | 408642.0 ns **-24%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 83939.0 ns **-25%** | 79652.0 ns **-29%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 482.0 ns **-27%** | 459.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 713.0 ns **-21%** | 660.0 ns **-27%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1096.0 ns **-32%** | 998.0 ns **-38%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1327.0 ns **-28%** | 1147.0 ns **-38%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 1028.0 ns **-25%** | 957.0 ns **-30%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3554.0 ns **-27%** | 3413.0 ns **-30%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3595.0 ns **-28%** | 3482.0 ns **-30%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4966.0 µs **-14%** | 4728.0 µs **-19%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 384990.0 ns **-10%** | 361108.0 ns **-16%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 286494.0 ns | 270984.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6692.0 µs | 6510.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1598.0 µs | 1571.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`5857854`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 443965.0 ns | 436871.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
