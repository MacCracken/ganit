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

Latest: **2026-09-10T08:24:28Z** — commit `334ac37`

Tracking: `a09d228` (baseline) → `8387ff8` (mid) → `334ac37` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 30.00 ns **-52%** | 28.00 ns **-56%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 33.00 ns **-18%** | 34.00 ns **-15%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 403.0 ns **-32%** | 372.0 ns **-37%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 301.0 ns | 311.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 3034.0 ns +40% | 1925.0 ns **-11%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 4439.0 ns +36% | 2851.0 ns **-13%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 52.00 ns **-17%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 263.0 ns | 240.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 49.00 ns **-22%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 183.0 ns +39% | 121.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 303.0 ns +17% | 247.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 136.0 ns +11% | 94.00 ns **-23%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 212.0 ns | 147.0 ns **-32%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 264.0 ns | 193.0 ns **-29%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 106.0 ns **-39%** | 97.00 ns **-44%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 647.0 ns **-16%** | 466.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 255.0 ns **-12%** | 183.0 ns **-37%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 773.0 ns | 542.0 ns **-34%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 1006.0 ns | 694.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 55093.0 ns | 43710.0 ns **-25%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4871.0 ns **-17%** | 4457.0 ns **-24%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 376.0 ns **-15%** | 281.0 ns **-37%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 69.00 ns **-40%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 710.0 ns | 511.0 ns **-31%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 503.0 ns | 377.0 ns **-30%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 68.00 ns **-24%** | 57.00 ns **-37%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 44.00 ns **-53%** | 42.00 ns **-55%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 186.0 ns **-24%** | 140.0 ns **-43%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 99.00 ns +21% | 94.00 ns +15% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 32.00 ns | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 94.00 ns | 90.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5080.0 ns | 4819.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 26.00 ns | 24.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20073.0 ns | 19562.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 24.00 ns **-31%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 76.00 ns | 73.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1747.0 µs | 1639.0 µs **-14%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 761074.0 ns | 713743.0 ns **-14%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2370.0 µs **-28%** | 2308.0 µs **-30%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 995040.0 ns **-44%** | 943947.0 ns **-47%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2394.0 µs | 2303.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 550663.0 ns | 537868.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 975.0 ns **-12%** | 961.0 ns **-13%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 441.0 ns | 433.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 731.0 ns **-29%** | 664.0 ns **-36%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1472.0 µs | 1385.0 µs **-13%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 375555.0 ns | 357960.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1642.0 µs | 1539.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5528.0 µs | 5268.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1226.0 ns **-16%** | 1186.0 ns **-19%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 119645.0 ns **-25%** | 111679.0 ns **-30%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 89232.0 ns **-25%** | 84885.0 ns **-29%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 84733.0 ns **-22%** | 81603.0 ns **-25%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13557.0 ns **-22%** | 12712.0 ns **-27%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13442.0 ns **-23%** | 12837.0 ns **-26%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 419098.0 ns **-22%** | 415921.0 ns **-22%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 80058.0 ns **-28%** | 80106.0 ns **-28%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 468.0 ns **-29%** | 458.0 ns **-31%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 685.0 ns **-24%** | 666.0 ns **-26%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1044.0 ns **-35%** | 999.0 ns **-38%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1214.0 ns **-34%** | 1133.0 ns **-38%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 994.0 ns **-28%** | 949.0 ns **-31%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3532.0 ns **-28%** | 3395.0 ns **-31%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3594.0 ns **-28%** | 3431.0 ns **-31%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4877.0 µs **-16%** | 4851.0 µs **-16%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 391122.0 ns | 362420.0 ns **-15%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 288191.0 ns | 287466.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6717.0 µs | 6589.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1605.0 µs | 1588.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8387ff8`) | Current (`334ac37`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 465525.0 ns | 432513.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
