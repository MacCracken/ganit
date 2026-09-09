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

Latest: **2026-09-09T16:56:39Z** — commit `a6eb77f`

Tracking: `a09d228` (baseline) → `a6eb77f` (mid) → `a6eb77f` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 17.00 ns **-26%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 30.00 ns **-52%** | 29.00 ns **-54%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 31.00 ns **-22%** | 31.00 ns **-22%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 386.0 ns **-35%** | 385.0 ns **-35%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 295.0 ns | 287.0 ns **-12%** |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1959.0 ns | 1935.0 ns **-11%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2801.0 ns **-14%** | 2761.0 ns **-15%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 243.0 ns | 245.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 41.00 ns **-35%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 121.0 ns | 124.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 249.0 ns | 252.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 94.00 ns **-23%** | 95.00 ns **-22%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 153.0 ns **-29%** | 152.0 ns **-30%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 193.0 ns **-29%** | 199.0 ns **-27%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 102.0 ns **-41%** | 103.0 ns **-40%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 471.0 ns **-39%** | 465.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 187.0 ns **-36%** | 185.0 ns **-36%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 560.0 ns **-32%** | 549.0 ns **-33%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 720.0 ns **-27%** | 719.0 ns **-27%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 44611.0 ns **-23%** | 44755.0 ns **-23%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4543.0 ns **-22%** | 4570.0 ns **-22%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 294.0 ns **-34%** | 282.0 ns **-36%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 69.00 ns **-40%** | 65.00 ns **-43%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 546.0 ns **-26%** | 528.0 ns **-29%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 395.0 ns **-26%** | 386.0 ns **-28%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 58.00 ns **-36%** | 57.00 ns **-37%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 43.00 ns **-54%** | 42.00 ns **-55%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 141.0 ns **-43%** | 137.0 ns **-44%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 100.0 ns +22% | 96.00 ns +17% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 91.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4923.0 ns | 4870.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 26.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20103.0 ns | 20110.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 6.00 ns **-14%** |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 74.00 ns | 74.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1715.0 µs **-10%** | 1698.0 µs **-11%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 730314.0 ns **-12%** | 723962.0 ns **-12%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2343.0 µs **-29%** | 2355.0 µs **-29%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 977974.0 ns **-45%** | 944923.0 ns **-47%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2358.0 µs | 2318.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 536574.0 ns | 539746.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 958.0 ns **-13%** | 993.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 423.0 ns | 449.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 665.0 ns **-36%** | 669.0 ns **-35%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1396.0 µs **-12%** | 1422.0 µs **-10%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 360266.0 ns | 373602.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1582.0 µs | 1577.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5336.0 µs | 5608.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1188.0 ns **-19%** | 1229.0 ns **-16%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 111304.0 ns **-30%** | 118002.0 ns **-26%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 84076.0 ns **-30%** | 89081.0 ns **-25%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 80411.0 ns **-26%** | 84009.0 ns **-23%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12652.0 ns **-27%** | 13252.0 ns **-24%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13132.0 ns **-25%** | 13415.0 ns **-23%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 406666.0 ns **-24%** | 415210.0 ns **-23%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79205.0 ns **-29%** | 81685.0 ns **-27%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 462.0 ns **-30%** | 461.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 672.0 ns **-26%** | 665.0 ns **-27%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1080.0 ns **-33%** | 1033.0 ns **-36%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1166.0 ns **-37%** | 1175.0 ns **-36%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 968.0 ns **-29%** | 971.0 ns **-29%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3423.0 ns **-30%** | 3435.0 ns **-30%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3552.0 ns **-29%** | 3623.0 ns **-27%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4781.0 µs **-18%** | 4743.0 µs **-18%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 410849.0 ns | 373356.0 ns **-13%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 277752.0 ns | 283638.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6655.0 µs | 6618.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1584.0 µs | 1668.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`a6eb77f`) | Current (`a6eb77f`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 438589.0 ns | 456517.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
