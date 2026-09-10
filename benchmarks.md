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

Latest: **2026-09-10T08:43:05Z** — commit `bcd2e26`

Tracking: `a09d228` (baseline) → `8387ff8` (mid) → `bcd2e26` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 25.00 ns |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 30.00 ns **-52%** | 31.00 ns **-51%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 33.00 ns **-18%** | 37.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 403.0 ns **-32%** | 400.0 ns **-33%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 301.0 ns | 336.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 3034.0 ns +40% | 2068.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 4439.0 ns +36% | 3014.0 ns |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 52.00 ns **-17%** | 39.00 ns **-38%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 263.0 ns | 255.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 49.00 ns **-22%** | 40.00 ns **-37%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 183.0 ns +39% | 128.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 303.0 ns +17% | 258.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 136.0 ns +11% | 99.00 ns **-19%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 212.0 ns | 155.0 ns **-29%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 264.0 ns | 199.0 ns **-27%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 106.0 ns **-39%** | 102.0 ns **-41%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 647.0 ns **-16%** | 502.0 ns **-35%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 255.0 ns **-12%** | 191.0 ns **-34%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 773.0 ns | 562.0 ns **-31%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 1006.0 ns | 721.0 ns **-27%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 55093.0 ns | 46409.0 ns **-20%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4871.0 ns **-17%** | 4713.0 ns **-20%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 376.0 ns **-15%** | 293.0 ns **-34%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 69.00 ns **-40%** | 66.00 ns **-43%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 710.0 ns | 538.0 ns **-27%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 503.0 ns | 395.0 ns **-26%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 68.00 ns **-24%** | 60.00 ns **-33%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 44.00 ns **-53%** | 44.00 ns **-53%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 186.0 ns **-24%** | 146.0 ns **-41%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 99.00 ns +21% | 102.0 ns +24% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 32.00 ns | 24.00 ns **-20%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 94.00 ns | 99.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5080.0 ns | 5170.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 26.00 ns | 26.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20073.0 ns | 20803.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 24.00 ns **-31%** | 25.00 ns **-29%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 76.00 ns | 79.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1747.0 µs | 1780.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 761074.0 ns | 771895.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2370.0 µs **-28%** | 2603.0 µs **-21%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 995040.0 ns **-44%** | 1068.0 µs **-40%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2394.0 µs | 2483.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 550663.0 ns | 574551.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 975.0 ns **-12%** | 1039.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 441.0 ns | 458.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 731.0 ns **-29%** | 698.0 ns **-33%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1472.0 µs | 1525.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 375555.0 ns | 386891.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1642.0 µs | 1667.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5528.0 µs | 5702.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1226.0 ns **-16%** | 1271.0 ns **-13%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 119645.0 ns **-25%** | 119944.0 ns **-24%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 89232.0 ns **-25%** | 91340.0 ns **-23%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 84733.0 ns **-22%** | 88476.0 ns **-19%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13557.0 ns **-22%** | 13701.0 ns **-21%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13442.0 ns **-23%** | 13953.0 ns **-20%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 419098.0 ns **-22%** | 447296.0 ns **-17%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 80058.0 ns **-28%** | 86362.0 ns **-23%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 468.0 ns **-29%** | 487.0 ns **-26%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 685.0 ns **-24%** | 714.0 ns **-21%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1044.0 ns **-35%** | 1087.0 ns **-32%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1214.0 ns **-34%** | 1240.0 ns **-33%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 994.0 ns **-28%** | 1024.0 ns **-25%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3532.0 ns **-28%** | 3596.0 ns **-27%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3594.0 ns **-28%** | 3708.0 ns **-26%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4877.0 µs **-16%** | 5179.0 µs **-11%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 391122.0 ns | 434227.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 288191.0 ns | 302976.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6717.0 µs | 7030.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1605.0 µs | 1707.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`bcd2e26`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 465525.0 ns | 470428.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
