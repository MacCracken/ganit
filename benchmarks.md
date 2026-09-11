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

Latest: **2026-09-11T01:55:19Z** — commit `fcdae2b`

Tracking: `a09d228` (baseline) → `8387ff8` (mid) → `fcdae2b` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 17.00 ns **-26%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 29.00 ns **-54%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 31.00 ns **-22%** | 36.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 381.0 ns **-36%** | 381.0 ns **-36%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 284.0 ns **-13%** | 300.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1906.0 ns **-12%** | 1920.0 ns **-12%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2731.0 ns **-16%** | 2738.0 ns **-16%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 239.0 ns | 240.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 119.0 ns | 119.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 248.0 ns | 246.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 92.00 ns **-25%** | 93.00 ns **-24%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 150.0 ns **-31%** | 148.0 ns **-32%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 192.0 ns **-29%** | 195.0 ns **-28%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 101.0 ns **-42%** | 99.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 460.0 ns **-40%** | 473.0 ns **-39%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 178.0 ns **-39%** | 182.0 ns **-37%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 538.0 ns **-34%** | 545.0 ns **-33%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 697.0 ns **-30%** | 697.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43581.0 ns **-25%** | 43904.0 ns **-25%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4450.0 ns **-24%** | 4488.0 ns **-23%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 279.0 ns **-37%** | 282.0 ns **-36%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 516.0 ns **-30%** | 513.0 ns **-31%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 379.0 ns **-29%** | 386.0 ns **-28%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 57.00 ns **-37%** | 58.00 ns **-36%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 41.00 ns **-56%** | 43.00 ns **-54%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 136.0 ns **-45%** | 141.0 ns **-43%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 96.00 ns +17% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 90.00 ns | 92.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4814.0 ns | 4857.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 24.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19091.0 ns | 19823.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 73.00 ns | 73.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1682.0 µs **-12%** | 1668.0 µs **-13%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 721699.0 ns **-13%** | 716451.0 ns **-13%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2274.0 µs **-31%** | 2320.0 µs **-30%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 925245.0 ns **-48%** | 927979.0 ns **-48%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2392.0 µs | 2353.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 527142.0 ns | 532991.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 953.0 ns **-14%** | 964.0 ns **-13%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 433.0 ns | 433.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 663.0 ns **-36%** | 706.0 ns **-32%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1382.0 µs **-13%** | 1388.0 µs **-12%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 356356.0 ns | 358679.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1539.0 µs | 1549.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5240.0 µs | 5293.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1180.0 ns **-19%** | 1187.0 ns **-19%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 117653.0 ns **-26%** | 112795.0 ns **-29%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 88850.0 ns **-26%** | 85141.0 ns **-29%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 81090.0 ns **-26%** | 81012.0 ns **-26%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12613.0 ns **-27%** | 12854.0 ns **-26%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 12571.0 ns **-28%** | 12929.0 ns **-26%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 399850.0 ns **-25%** | 410163.0 ns **-24%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79750.0 ns **-28%** | 80201.0 ns **-28%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 458.0 ns **-31%** | 459.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 669.0 ns **-26%** | 665.0 ns **-27%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1010.0 ns **-37%** | 1005.0 ns **-38%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1151.0 ns **-37%** | 1151.0 ns **-37%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 954.0 ns **-30%** | 968.0 ns **-29%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3363.0 ns **-31%** | 3399.0 ns **-31%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3473.0 ns **-30%** | 3498.0 ns **-30%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4666.0 µs **-20%** | 4746.0 µs **-18%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 363944.0 ns **-15%** | 378831.0 ns **-11%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 269476.0 ns | 272080.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6464.0 µs | 6502.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1557.0 µs | 1565.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`fcdae2b`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 431040.0 ns | 434956.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
