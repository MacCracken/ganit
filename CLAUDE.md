# Hisab — Claude Code Instructions

## Project Identity

**Hisab** (Arabic: calculation/mathematics) — Higher math: linear algebra,
geometry, calculus, numerical methods, spatial structures, Lie groups,
differential geometry, symbolic algebra.

- **Type**: Cyrius library + CLI (math toolkit)
- **License**: GPL-3.0-only
- **Language**: Cyrius (sovereign systems language, compiled by cycc)
- **Toolchain**: Cyrius 6.6.2 (`cyrius.cyml: cyrius = "6.6.2"`)
- **Version**: SemVer, version file at `VERSION` (manifest pulls via `${file:VERSION}`)
- **Status**: 2.11.5 — compiles cleanly under cycc 6.6.2. Library source lives in `src/` (smoke `main.cyr` + 35 math modules, 23,319 lines); `lib/` is vendored stdlib + deps only. CLI smoke binary builds (257,176 B static x86_64 ELF); the **full 35-module distlib bundle** (905,803 B / 23,405 lines — ~**3.6%** of cycc 6.6.1's **24 MB expanded-source** cap; ⚠ note the *token* cap, 4,194,304, can bind first, and the retired "16 MB `input_buf`" figure described `_SRC_CAP`, which is not what rejects a consumer build) ships at `dist/hisab.cyr`, and since 2.9.2 the `dist/hisab.deps` leaf sidecar (15 stdlib leaves) is tracked beside it so a consumer's `cyrius deps` auto-resolves them instead of the consumer hand-declaring the list. Library validated via tests (**3538/3538** across 5 suites — hisab 416, foundation 351, modules 1797, edge_cases 233, abuse 741), 72 benchmarks, and `scripts/check-constants.sh` (159/159 verified, 1 skipped). `cyrius coverage` **640/644 (99%) functions** over 36/36 files; the 80% target set at the start of the 2.6.x audit was met in 2.7.2. **2.7.0–2.11.1 are the arc after the 2.6.12–2.6.15 audit-repair arc** — that earlier record lives in `CHANGELOG.md` and `docs/development/roadmap.md`, and is not restated here. 2.7.0 closed the 2026-08-04 re-audit (35 of its 41 findings, every fix mutation-proven) and found the constant gate *itself* had been silently skipping 35 of 145 declarations while printing "110/110". 2.8.0 rewrote `delaunay_2d` onto triangle adjacency after establishing the shipped code was **wrong, not just slow**: 150 points on a circle returned 1651 triangles where 148 is correct, 206 edges shared by three or more triangles, 9 input points dropped. 2.8.1 audited that tree (42 findings, 4 critical, 0 refuted — `docs/audit/2026-08-04-v2.8.0-full.md`); 2.8.3 integrated eight parallel repair tracks and made the narrowphase return the exact MTV. 2.9.0 added `tests/abuse.tcyr` — 732 assertions that surfaced 11 defects on public entry points — plus a finding-disposition column and a struct-layout contract; 2.9.1 cleared the deferred tier of four latent ghost in-circle predicate defects, one of them (`_col_dl_ic_g1`) wrong on ordinary CCW input. 2.9.2 is a toolchain release — pin 6.5.9 → 6.5.16, sakshi 2.4.8 → 2.4.10, **no library source change** (the `dist/hisab.cyr` diff is the version header alone) — whose content is **three gates that were green while checking nothing**: `scripts/bench-history.sh` recorded each benchmark's MAX rather than its average in every row it ever wrote (44 of 55 in the latest run), CI's version gate was an unanchored `grep -q "$VERSION" CHANGELOG.md` that matched `32,942,104 B` in an unrelated benchmark line, and `cyrius distlib`'s 6.5.14 bundle self-check rejects any bundle reading a stdlib global. **No performance claim is made in 2.9.2**; it establishes the first *correct* benchmark baseline (measured noise on the `avg` field: median 3.5% spread over three back-to-back runs of the same binary, worst 6.9%, 0 of 55 above 20%). 2.9.3 worked the open filings and found **the filings themselves were the thing needing audit**: two of six argued for repairs that measurement refuted — an exact determinant of *pre-differenced* operands fixes nothing (the information is gone before it runs), and dropping EPA's seed test produces wrong depths at exact tangency — and a third rested on a measurement that had only looked at one of the two entry points it was about. Both real defects it did fix were silent: `delaunay_2d` DROPPED input points on any set mixing scales, and `_col_dl_incircle` answered differently by vertex order. 2.10.0 is the first feature line since 2.8.x — differentiable geometry, `src/geo_diff.cyr`, jets for plane/sphere/triangle giving the full gradient from one evaluation as a post-pass on the shipped primal (sphere 7 partials at 3.6x its primal, triangle 15 at 3.3x, against the ~15 full evaluations forward-mode duals would need). **It too led with a defect in shipped code**: running the feature's own homogeneity check against the EXISTING primitives first showed `geo_ray_sphere` was not degree -1 in the direction, returning a hit point 3.74 from the centre of a unit sphere, with `geo_ray_capsule` inheriting it — and the same pass found 17 of 60 benchmarks measuring `clock_gettime` rather than the operation. 2.10.1 completed the seam — jets for aabb/obb/capsule, each primal split onto a face/branch-reporting variant with the plain entry point a one-line wrapper, so the value path is unchanged by construction (aabb 3.1x its primal for 12 partials, obb 1.7x for 12, capsule 1.38x for 13). **For the third release running the design check found defects in shipped code first**: `geo_ray_aabb`/`_obb` returned +Inf for a degenerate direction, and `geo_ray_sphere`/`_capsule` compared a SQUARED length against the unsquared `EPSILON_F64` — the sphere reporting a miss for any |d| < 1e-6 (a guard **2.10.0's own repair introduced**, and which 2.10.0's homogeneity sweep could not have caught because its scales are 2, 0.5 and 7), the capsule silently returning a cap hit instead of the cylinder hit, wrong by 0.942%. ⚠ **A third defect came from a check commissioned against 2.10.1's FINISHED code**: an independent derivation confirmed all 37 partials (0 of 24,237 FD comparisons failing) and its value was entirely in the ENUMERATION — the tie flag saw edges and corners but not a zero-width slab or a tangential clip, so a flat box returned the whole gradient in the wrong slot with tie = 0 on half of all configurations. **A test written by the author of the code inherits the author's list of cases.** 2.10.2 closed all three of those PRIMAL defects and added `geo_jet_obb`'s rotation partial (12 → 15). **One rule settled four thresholds**: guard exactly what makes the DIVISION fail and nothing more — `_GEO_F64_TINY` = DBL_MIN — because a threshold that needs a scale chosen for it is the defect. The scale-free slab test returned a hit at x = 1.4 for a box spanning [0,1] with a UNIT direction, and an exit 18x too large from inside; `geo_ray_capsule` returned a point a full radius INSIDE the solid (t = 4 where the exit is 6) because `geo_ray_sphere` hands back only the FIRST root, so a cap root rejected by the half-space test meant the other was never considered — 32 of 600 interior origins also lost their exit entirely; and `geo_triangle_unit_normal` returned a FABRICATED (0,1,0) for a well-formed right triangle. `dt/d(rotation)` = [a_k × (c − p)]/f_k, derived by perturbing WITHIN the rotation group rather than freely, is a world-frame ANGULAR gradient so a step along it is a valid rotation by construction. ⚠ Two of 2.10.2's own repairs cost far more before being measured (an `is_inf` helper as a FUNCTION was +5.6% on a 90 ns routine; materialising a hit point per cap root was +92%), two fixtures were caught by COUNTERS rather than by failing (a surface sweep that was all misses, a rotation sweep where 3 of 4 configs missed), and a toolchain defect was nearly filed that does not exist — `var buf[N]` needs `&buf`, which `lib/fmt.cyr` has always done. **The stdlib is the specification.** 2.11.0 added tape-based reverse-mode autodiff — one sweep for an n-input gradient where forward-mode duals need n passes, measured at 11.2x on 16 inputs (63.7 us -> 5.70 us; not the theoretical 16x, because reverse pays to RECORD the tape and part of the rest is dual_* heap-allocating under an allocator that never frees). **Fourth release running, the feature's own oracle found defects in shipped code first**: reverse mode is validated AGAINST forward mode, so `autodiff.cyr` was swept before a line of tape code existed, and five defects came out — `1/1e-13` returned (0,0) where the truth is 1e13, `ln(-5)` returned a NaN value beside a CONFIDENT -0.2 derivative, `sqrt(-4)` was (NaN,NaN) because the guard ran after the sqrt, and `d/dx x^3 at -2` was NaN because `f64_pow` is `exp(n*ln(base))` and rejects negative bases (the stdlib's STATED implementation, so the defect was hisab inheriting it undocumented). ⚠ **The finite-difference sweep alone found NONE of them**: at every input where a guard fires the perturbed scalar is NaN too, so the sample is skipped and the guard never asked — a guard has to be interrogated DIRECTLY. The optimizer pairing needed no API change (a capturing closure holding the tape matches `fncall2(grad, x, out)`, re-verified on 6.5.18), and the solvers' own contract check came back CLEAN for the first time in five releases. 2.11.1 is an audit release, no feature: a full P(-1) sweep of the 2.11.0 tree — 6 dimensions, 115 checks, **52 findings reproduced, 21 CONFIRMED by an independent skeptic, 2 REFUTED, and 28 reproduced but NEVER VERIFIED** because the harness capped the verify phase at 24 (recorded in the report as the audit's own biggest process defect; with 2 of 24 refuted, an unverified finding is ~1-in-12 wrong, so **none may be acted on until re-run**). **24 of the 52 are one defect** — a guard comparing a quantity against a threshold that is wrong for it, then FABRICATING a plausible answer: `eigen_qr` wrong eigenvalues with rc = 0, `cqr_decompose` an R that is not upper triangular with rc = 0, `hvec3_angle` 0 rad ("parallel") for exactly perpendicular vectors, `m3`/`m4_inverse` the identity — the ninth through thirty-second instance of the class recorded in `complex.cyr:58` in 2.6.14, **and `cx_div` is IN THAT COMMENT and still fabricating zero**. Four repairs shipped (`hquat_inverse`/`hquat_normalize` returned the identity for any |q| < 1e-6 where the inverse is finite and exact; three caller-sized allocations in `num_ext` reproduced as SIGSEGV; `ad_tape_new` handed back a non-zero handle with a null body — 2.11.0's own code; `optimize.cyr`'s ALLOC_MAX-derived ceiling re-derived 5792 → 16384), the rest scheduled on the roadmap. ⚠ **A stale comment in one file nearly buried a real defect in another**: the first `num_tridiag_solve` probe was sized from `optimize.cyr`'s own `ALLOC_MAX = 256 MiB`, landed EXACTLY on the limit, allocated successfully, and the finding looked refuted — cyrius 6.4.51 raised it to 2 GiB. **A constant derived from a dependency is a measurement**, and it goes stale silently. ⚠ The quaternion sweep's first 12 decades killed 1 of 3 mutants (both thresholds a repair might plausibly have chosen sit BELOW where it looked; 20 decades kills all three), and asserting unit length would not have discriminated at all — the fabricated identity IS unit, so the assertion checks DIRECTION. **The audit's third critical is this repo's own suite**: 836 of the 3510 assertions its scan counted (23.8%) compare through `f64_to`, which TRUNCATES, and 38.4% of value-changing single-operator mutants survive all five suites — the measured reason 3507 assertions and 99% coverage saw none of the 52. **Writing a lesson beside the code that taught it does not fix that code and does not reach the other thirty-four modules; only a grep does.** 2.11.2 is the toolchain catch-up — cyrius **6.5.18 → 6.5.33**, sakshi 2.4.10 → 2.4.11, ganita 1.0.4 → 1.1.4 — and **every finding in it is a number that was written down once and then trusted**, the same shape as 2.11.1's ALLOC_MAX, three times over, none of them surfaced by a failing test (all 3514 assertions passed on BOTH sides of the bump). ⚠ The tree arrived **mid-sync**, vendored from ≈6.5.19, so ganita was **three releases stale behind a green `deps --verify`** — old-pin-vs-new-pin shows a tidy diff and misses it; only the pin's own snapshot finds it, and that is the third time ganita specifically has been caught by that shortcut. **(1)** `f64_pow`'s domain moved upstream, changing a hisab **public API** — `expr_eval((-2)^3)` returned NaN for hisab's entire history and returns a number now — and `_ad_pow` survives on **precision**, not domain: `(-2)^4` reads 15 through the stdlib and 16 through hisab because `f64_to` truncates. **(2)** The benchmark **instrument** changed: 6.5.19 measures and subtracts the clock floor, moving **44 of 72 rows >10% with not one of them a speedup** — `ease_in_out` 1,407 ns → **7 ns**, and four operations spanning **149x** in reality had been reported within **1.26x**, so a regression had room to hide. This was the **third** measurement-method change to move every row, so the marker became a `regime` COLUMN the trend filter enforces rather than a fourth paragraph of prose. **(3)** The **1 MB `input_buf` ceiling this project designs around has not existed since 6.5.22** (it is 16 MB; the bundle is 5.4%, not 85%), verified by compiling a 1,162,472 B source. ⭐ And a CI annotation had been handing developers `cyrius fmt $f > tmp && mv tmp $f`, which **truncates the file to 0 bytes** since 6.5.28 made fmt rewrite in place — printed only for already-drifting files, of which that same release produced **38 at once**. ⚠ Three times in that release my own *instrument* was wrong before the thing measured was: an extractor keyed on `fn` blamed six trailing comment blocks on their preceding functions, a `$?` captured `tail` instead of `cyrius test`, and a zsh `grep` failed a manifest gate that passes in the bash CI actually runs. **Check the probe before believing the probe.** 2.11.3 is the next toolchain catch-up — cyrius **6.5.33 → 6.6.1** (crossing a minor), sakshi 2.4.11 → 2.5.1, ganita 1.1.4 → 1.2.4 — and **it is the first bump in this project's history where the new toolchain MISCOMPILED the library**. Three of five suites SIGSEGV'd on the first run: a `#derive(accessors)` getter passed DIRECTLY as an `f64v_*` intrinsic argument makes the intrinsic's inline expansion read its **destination pointer from a stack slot that no instruction in the function ever writes** — `movupd %xmm0,(%rdx,%rsi,8)` with `%rdx = 12`, loaded from `-0x50(%rbp)`, while the src and scalar slots ARE written. **Bisected across ten toolchains: 6.5.70 clean, 6.5.71 broken**, which is the release that put derive accessors on the inline-replay path (`callq` 7 → 3) and thereby removed the call that had incidentally forced the spill — **so the intrinsic bug is older than 6.5.71 and that release only stopped hiding it**. Three functions of identical shape in ONE file discriminate it: a plain fn accessor is fine, a raw `load64` is fine, only the derived getter fails, so it is not "inlining" in general. Repaired by hoisting the accessors into locals — **which the sibling `m3_mul_vec3` has always done**, so mat4 was the lone inconsistency and the fix is consistency, not a hack. ⚠ It is **wrong code, not merely a crash**: it faults only because the garbage slot happens to be unmapped. Filed in the **cyrius repo**, not here — that is where the language agent reads them — with a repro that exits 0 on 6.5.70 and 139 on 6.6.1, so it proves its own bisect. **(2) Two tests failed because upstream FIXED the defects they had pinned**: `f64_pow(-2,4)` read 15 and now reads a bit-exact 16 (ganita 1.2.4 swapped `exp(n·ln|base|)` for binary exponentiation), voiding `_ad_pow`'s precision rationale; and `cx_exp(-inf)` returned NaN and now returns exactly +0 (6.6.1 gave `math.cyr`'s exp polyfill its missing infinity guard). **A suite that pins a dependency's bug as expected behaviour fails on the repair, not on a regression** — and the replacement assertion had to be made DISCRIMINATING, because +0 is also what the fabricated-guard class produces, so `exp(+inf)` is asserted beside it. **(3) The ceiling comment named the wrong limit entirely** — expanded source 8 MB → 24 MB, tokens 1,048,576 → 4,194,304, and the token cap bit FIRST on an 8.2 MB probe; measured as a PAIR (a 9,002,640 B source is rejected by 6.5.33 and clean on 6.6.1) because a one-sided "it compiled" proves only that the cap exceeds one file, which is the shortcut the retired 16 MB figure rested on. ⚠ The release also found **41 of 72 benchmarks reporting a `net` value BELOW the timer floor they subtract** (worst 0.01x), so its own `ray_aabb` −55.9% is evidence of nothing; the real speedups (bvh_query_ray −46.5%, gjk_epa −29.6%) are claimed only from the 23 rows where net ≥ 10x floor, with the flat numeric kernels as the control proving the instrument did not shift. ⚠ And **my own probe was wrong twice more** — a `$?` after a pipe captured `tail` rather than `cyrius test` (again), and a `[lib]` string match clobbered the manifest comment it was editing. **Check the probe before believing the probe.** 2.11.4 migrated hisab off ganita's deprecated aliases — and **2.11.3 had estimated that at 8 call sites when it is 536**, because the estimate came from the names in ganita's changelog paragraph rather than from the code; the deprecation block is 53 aliases and hisab was on 20 of them, `mat_set` alone accounting for 265. The 5 suites produce **byte-identical output** across the change (captured and diffed, not re-counted), because the aliases are literal one-line forwarders. ⭐ **They were not free**: cycc's `#inline` requires <= 2 parameters and `ganita_mat_get`/`_set` take 3 and 4, so every matrix access in the SVD and eigen inner loops paid a `call`/`ret` for nothing — `svd_golub_kahan_12` **-27.85%** and `eigen_qr_12` **-27.67%**, against a same-binary noise floor measured in the same session (median 2.10%, worst 9.13%), so ~3x the worst noise and landing on exactly the two benchmarks the mechanism predicts; **no other movement is claimed**. ⚠ **Two assertions had gone VACUOUS and neither could have announced it**: 2.11.3 fixed the two tests that FAILED when upstream repaired what they had pinned, but these two compare through a tolerance and a round, so they kept passing while their stated property evaporated — the stdlib `(-2)^3` truncates to -8 now, not -7, so the "truncation-discriminating" pair discriminates nothing. **An assertion written against a defect cannot fail when the defect is repaired; it just stops testing anything.** ⚠ And `_ad_pow`'s reason to exist has now collapsed TWICE — domain in 2.11.2, precision here (665 of 665 comparisons agree bit for bit) — with a third, narrower ground surviving that is the OPPOSITE of the second: squaring amplifies relative error, so `(-0.999)^1000` is 4 ulp from truth through repeated multiplication and 58 through ganita's binary exponentiation, while past its +-1024 window the ranking inverts. Filed rather than decided, recording that **no test exercises the one regime it still wins**. ⛔ **AND TWO OF 2.11.4's OWN INSTRUMENTS WERE WRONG BEFORE THE THING MEASURED WAS.** (1) `_ad_pow` tied the stdlib on every POSITIVE base at every exponent, which reads as redundancy — until the function's first line turned out to be a `base > 0` delegation, i.e. **the stdlib being compared with itself**. Running the loop on positive bases beats it on **44 of 48**, `0.9^1024` at 7 ULP against 137. The delegation is gone; and widening the loop exposed a **fabricated 1.0** that had been live for negative bases — `f64_to` SATURATES, `f64_to(1e300)` is i64::MIN, negating i64::MIN returns itself, so a bound checked AFTER the truncation reads false and `(-2)^1e300` returned 1.0 where the answer is +Inf. **The order of the guard is the guard.** (2) 2.11.3's "41 of 72 benchmarks report a net below the timer floor" filing was **REFUTED by measurement**: it compared a per-op net against a per-clock-pair floor, while `bench_run` sizes every batch so the clock is 1% of the window — and on a same-binary re-run the tier it dismissed is the QUIETER one (median 1.43% vs 2.10%). That bad ratio had suppressed most of 2.11.3's real result: `ray_aabb` **-54.8%**, `vec3_cross` **-54.0%**, `ray_triangle` -44.3%, each 15-25x its own noise. ⭐ **Being conservative is not the same as being correct — a wrong instrument suppresses real findings as readily as it invents false ones.** 2.11.5 took cycc **6.6.1 → 6.6.2**, which REPAIRS the wrong-code bug 2.11.3 filed — and establishes that **the filing was wrong about its scope in two ways**. It was filed as derive-specific and as a 6.5.71 regression and was NEITHER: all **21** `f64v_*`/`f32v_*`/`f32v8_*`/`f64v256_*`/`iv_*` handlers took `var vbase = GFLC(S)` without raising GFLC until every argument had been parsed, so any argument allocating a frame local bound it over the intrinsic's own destination — reachable via `callptr` since 6.0.70 and `#inline` since 6.5.63. **A first-bad-version is evidence about VISIBILITY, not about ORIGIN**, and the filing's own root-cause section said so while its header contradicted it. The severity was understated too: with `CYRIUS_REGALLOC_PICKER_CAP=0` it exits 0 and writes into the ARGUMENT OBJECT — silent wrong code, not merely a crash. `m4_mul_vec4`'s hoist is now unnecessary (verified: the unhoisted form passes all three suites) and is kept only for consistency with `m3_mul_vec3`; **its comment no longer claims to be load-bearing**, because a stale mandatory-workaround note costs the next reader the same investigation twice. ⚠ A second fix retires a landmine this repo has documented since 2.3.1 **with the wrong cause attached**: a bare SIMD intrinsic at top level compiled clean and SIGSEGV'd on every release, not because of SSE stack alignment as `vec4.cyr` claimed, but because these handlers stash operands in FRAME slots and top-level code has no frame; 6.6.2 refuses at compile time. ⚠ 6.6.2 also shows the one defect class a consumer build cannot catch — 6.6.0 **silently redefined `tag()` and `is_tag()` at unchanged arity** — checked rather than assumed here: hisab calls neither. **A name whose meaning changed must be RETIRED, not redefined.** No performance change is claimed; two rows that looked like regressions were single-run outliers, settled by comparing each benchmark against ITS OWN spread rather than a global band. **The standing lesson from the whole arc**: every defect of consequence was cheap to find and invisible because nothing looked — 2.6.x's defects all sat in modules with zero or near-zero coverage, and 2.9.x's sat behind gates nobody had aimed at their own output. The 6.4.66 bump (2.6.9) renamed `modules.tcyr`'s interval result vars `iv_add`/`iv_sub`/`iv_mul` → `iv_sum`/`iv_diff`/`iv_prod` because the originals collide with reserved cycc SIMD intrinsic names. **That reservation is by design and still stands on 6.6.1 — the rename is load-bearing and must not be reverted** (re-verified 2026-09-09 on 6.6.1: `var iv_add = 1;` gets `expected identifier, got reserved keyword 'iv_add' (cannot be used as an identifier; rename the variable/field/fn)`, while a renamed control fails for an unrelated reason, so the probe discriminates); what was a defect was the misleading `expected identifier, got unknown` diagnostic, and it is fixed (all 67 reserved names emit the new wording on 6.5.16 through 6.5.18; the issue closed and archived 2026-08-09). See `docs/development/issues/archived/2026-07-17-cyrius-interval-ident-lex.md`.

## Consumers

impetus (physics), kiran (engine), joshua (simulation), aethersafha (compositor)

## Quick Start

```bash
cyrius deps                              # resolve GIT deps into lib/ (sakshi)
cyrius lib sync                          # vendor the declared stdlib subset into lib/
cyrius build src/main.cyr build/hisab    # build CLI
cyrius test tests/hisab.tcyr             # run a test suite
cyrius bench tests/hisab.bcyr            # run benchmarks
cyrius fuzz                              # run fuzz harnesses (walks tests/, needs >= 6.5.6)
```

## Dependencies

- **Cyrius stdlib** — `syscalls`, `string`, `alloc`, `str`, `fmt`, `vec`,
  `io`, `args`, `assert`, `math`, `ganita`, `tagged`, `fnptr`,
  `bench`, `callback` (ships with Cyrius >= 6.2.11). `ganita` is the 6.2.x
  math umbrella — transcendentals (acos/asin/atan2/pow/sinh/cosh/tanh +
  inverses) plus the full `matrix`/`linalg` API; do **not** also list `matrix`
  or `linalg` (duplicate-definition collisions). `math` stays for the inclusive
  comparisons, clamp/lerp/min/max/sign and the exp/ln polyfills.
- **sakshi** 2.5.1 — structured logging (first-party)

No external deps. No FFI. No libc. All first-party, pinned in
`cyrius.cyml` and SHA-locked in `cyrius.lock`.

## Layout

```
src/main.cyr         — CLI smoke binary (prints version, exits — does NOT
                       include the library; library coverage is in tests/)
src/*.cyr            — the 35 math modules (the library source). Self-contained
                       (no `include` lines); stdlib resolves via [deps] stdlib
lib/                 — vendored stdlib + first-party deps ONLY — no project
                       source here. Since the 6.3.x CLI split, `cyrius deps`
                       resolves GIT deps (sakshi) and `cyrius lib sync` vendors
                       the declared stdlib subset; byte-check the result against
                       the PIN's own snapshot, never old-pin vs new-pin
dist/hisab.cyr       — full 35-module distlib bundle (905,723 B / 23,404 lines),
                       regenerated via `cyrius distlib`. Consumers pull this
                       single file via [deps.hisab] modules = ["dist/hisab.cyr"]
dist/hisab.deps      — leaf sidecar naming the 15 stdlib leaves the bundle
                       needs in scope; written by `cyrius distlib`, read by a
                       consumer's `cyrius deps`. TRACKED since 2.9.2
examples/            — small demos (basic_math.cyr)
tests/
  hisab.tcyr         — primary assertion suite
  foundation.tcyr    — vec/quat/mat foundations
  modules.tcyr       — per-module coverage
  edge_cases.tcyr    — degenerate inputs, boundary values
  abuse.tcyr         — negative indices, zero/huge dims, non-conformable
                       operands, the designed-0 return, canary checks
  hisab.bcyr         — benchmark harness
  hisab.fcyr         — fuzz harness
docs/
  architecture/      — module map, math reference
  development/       — roadmap, threat model, dep watch, port audit, issues
  guides/            — testing
  audit/             — dated audit reports
scripts/
  bench-history.sh   — run tests/hisab.bcyr, append CSV rows, rebuild
                       benchmarks.md
  check-constants.sh — decode + verify every hand-encoded hex f64 in src/
  check-measurements.sh — every measurement-shaped claim in a comment carries
                       a [measured: …] provenance marker
  measurement-baseline.txt — per-file unmarked-claim counts (ratchet floor)
  version-bump.sh    — bump VERSION + CHANGELOG header + src/main.cyr's
                       printed version string
benchmarks.md        — rendered benchmark trend table (repo root; generated)
bench-history.csv    — benchmark regression history (repo root)
cyrius.cyml          — package manifest (toolchain pin, [deps])
cyrius.lock          — SHA256 lockfile (after first `cyrius deps`)
VERSION              — single source of truth for version
```

## Development Process

### P(-1): Scaffold Hardening (before any new features)

0. Read roadmap, CHANGELOG, and open issues — know what was intended before auditing what was built
1. Cleanliness sweep: `cyrius lint <file>`, `cyrius fmt <file> --check`, `cyrius vet src/main.cyr`
2. Test + benchmark sweep of existing code: `cyrius test tests/*.tcyr`, fuzz harnesses pass
3. Baseline benchmarks: `./scripts/bench-history.sh`
4. Internal deep review — gaps, optimizations, security, logging/errors, docs
5. External research — domain completeness, missing capabilities, best practices, world-class accuracy
6. Cleanliness re-check — must be clean after review
7. Additional tests/benchmarks from findings
8. Post-review benchmarks — prove the wins
9. Repeat if heavy

### Work Loop (continuous)

1. Work phase — new features, roadmap items, bug fixes
2. Cleanliness check: `cyrius lint <file>`, `cyrius fmt <file> --check`, `cyrius vet src/main.cyr`
3. Test + benchmark additions for new code
4. Run benchmarks: `./scripts/bench-history.sh`
5. Internal review — performance, memory, security, throughput, correctness
6. Cleanliness re-check — must be clean after audit
7. Deeper tests/benchmarks from audit observations
8. Run benchmarks again — prove the wins
9. If audit heavy → return to step 5
10. Documentation — update CHANGELOG, roadmap, docs
11. Version check — VERSION matches CHANGELOG header (cyrius.cyml auto-syncs via `${file:VERSION}`)
12. Return to step 1

### Task Sizing

- **Low/Medium effort**: batch freely — multiple items per work loop cycle
- **Large effort**: small bites only — break into sub-tasks, verify each before moving to the next
- **If unsure**: treat it as large

### Refactoring

- Refactor when the code tells you to — duplication, unclear boundaries, performance bottlenecks
- Never refactor speculatively. Wait for the third instance before extracting an abstraction
- Refactoring is part of the work loop, not a separate phase. If a review reveals structural issues, refactor before moving on
- Every refactor passes the same cleanliness + benchmark gates as new code

## Key Principles

- **Numbers don't lie.** Never claim a performance improvement without before/after benchmark numbers. The CSV history is the proof
- **Tests + benchmarks are the way.** Aim for 80%+ coverage
- **Own the stack.** First-party deps only — sakshi for logging, cyrius stdlib for everything else
- **No magic.** Every operation is measurable, auditable, traceable
- **Direct syscalls** — no libc wrappers
- **Manual struct layout** when needed — `alloc()` + `load64`/`store64` with named offset constants
- **Bump allocator** for long-lived data; **freelist** for individually-lifecycled data
- **str_builder** for formatting — avoid temporary allocations
- **Enums for constants** — zero `gvar_toks` cost vs. `var` globals
- **Source files only need project includes** — stdlib + first-party deps auto-resolve from `cyrius.cyml`
- **Every buffer is a contract**: `var buf[N]` = N bytes
- **Programs call `main()` at top level**: `var exit_code = main(); sys_exit_group(exit_code);`
  — `sys_exit_group`, never `syscall(60, …)`/`syscall(SYS_EXIT, …)`: SYS_EXIT is exit(2) and
  ends only the *calling thread*, so a program that ever spawns one hangs. Test harnesses must
  also **clamp** first (`if (r > 0) { r = 1; }`) — `assert_summary()` returns the raw failure
  count and a wait status is 8 bits, so exactly 256/512/768 failures would otherwise exit 0
  and score PASS
- **`cyrius build` handles everything** — never shell out to `cycc` directly

## CI / Release

- **Toolchain pin**: `cyrius = "6.6.2"` in `cyrius.cyml`. CI and release both grep
  the manifest; no hardcoded versions in YAML
- **Toolchain install**: both workflows pipe the pin into the **upstream installer**
  (`curl -sSf .../cyrius/main/scripts/install.sh | CYRIUS_VERSION="$PIN" sh`), matching patra.
  **Never hand-roll the tarball extraction.** ⚠ The hand-rolled block this replaced laid the
  toolchain out FLAT as `~/.cyrius/{bin,lib}` — cyrius resolves the stdlib from
  `~/.cyrius/versions/<pin>/lib`, so it broke outright at 6.5.33 — and it also did **no checksum
  or signature verification** and swallowed every copy error with `|| true`, so it could not fail.
  The installer verifies the `.sha256` fail-closed (CVE-21) plus an Ed25519 signature over
  `SHA256SUMS` (CVE-13), and errors loudly on a tarball missing `bin/` or `lib/`
- **Toolchain-verify gate**: immediately after install, `cyrius version` must equal the manifest
  pin and `~/.cyrius/versions/<pin>/lib` must exist. Added 2.11.2 because the install failure
  surfaced two steps later as a path error out of `cyrius deps` — assert where it is diagnosable
- **Tag filter**: release triggers on `tags: ['v?[0-9]+.[0-9]+.[0-9]+']` (with or without `v` prefix)
- **Version-verify gate**: release asserts `VERSION == git tag` before building
  (cyrius.cyml auto-syncs via `${file:VERSION}`). CI's docs job additionally asserts the
  manifest still pulls `${file:VERSION}`, that `CHANGELOG.md` carries a literal
  `## [$VERSION]` header — `grep -qF --`, anchored, after the old unanchored
  `grep -q "$VERSION" CHANGELOG.md` matched `32,942,104 B` in an unrelated line and
  passed vacuously — plus the two version sites `${file:VERSION}` cannot reach:
  `src/main.cyr`'s printed string and `dist/hisab.cyr`'s bundle header
- **Lint gate**: `cyrius lint <file>` per source — warnings are errors. Since 6.2.11 lint
  exits 0 even with warnings, so the `^  warn ` grep over stdout is the load-bearing half
- **Fmt gate**: `cyrius fmt <file> --check` per source — drift fails the build.
  The flag goes AFTER the path; `cyrius fmt --check <file>` is a usage error. Check mode
  signals via exit code and writes nothing to stdout — never diff its stdout against the file
- **Vet gate**: `cyrius vet src/main.cyr`
- **Constant gate**: `./scripts/check-constants.sh` — decodes every hand-encoded hex f64 literal
  in `src/` and asserts it against the value in its own comment (156/156 verified, 1 skipped).
  Added 2.6.12 after seven mis-transcribed constant tables shipped; run it after touching ANY
  constant. Its own regex rejected `_` digit separators until the 2026-08-04 re-audit, silently
  skipping 35 of 145 declarations while reporting "110/110"
- **Measurement gate**: `./scripts/check-measurements.sh --diff origin/<base>` — pull requests
  only — every measurement-shaped claim a branch ADDS to a comment must carry a
  `[measured: …]` provenance marker, plus `--selftest` for detector recall. It cannot check the
  number, only that a later reader can re-derive it; a full-tree scan is pre-existing debt
  ratcheted by `scripts/measurement-baseline.txt`
- **Lock gate**: `cyrius deps --verify` against committed `cyrius.lock` (30/30)
- **Distlib gate**: `cyrius distlib` regenerates `dist/hisab.cyr` **and** `dist/hisab.deps`;
  CI fails on drift in either (run `cyrius distlib` locally before committing changes to the
  `src/*.cyr` modules named in `[lib]`, to `[lib]` itself, or on ANY version bump — the bundle
  header is stamped from `VERSION`). ⚠ HISTORY, kept because the shape recurs: 6.5.14–6.5.16 ran a
  bundle self-check that compiled the bundle ALONE under `_cc_allow_undef`, which downgrades
  undefined *functions* but cannot reach undefined *names* — hisab's bundle legitimately reads
  `F64_ONE`, so `distlib` exited 1 on a good bundle and both workflows tolerated exactly that
  signature for three toolchain releases. **Fixed upstream in 6.5.17**; the tolerance is removed and
  a non-zero `distlib` is a real failure again. See
  `docs/development/issues/archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md`
- **Consumer-build gate**: `cyrius check --with-deps dist/hisab.cyr` — compiles the bundle with
  the manifest's stdlib in scope, i.e. the way a consumer builds it. Strictly stronger than the
  broken self-check above
- **Build/ELF gates**: `cyrius build src/main.cyr build/hisab`, then the magic bytes are checked
- **Test/Fuzz/Bench gates**: every `tests/*.tcyr`, `tests/*.fcyr`, `tests/*.bcyr` runs
- **Docs + security jobs**: required root files present, version consistency (above), and a scan
  for dangerous patterns
- **Concurrency**: CI uses `cancel-in-progress: true` keyed on workflow + ref

## DO NOT

- **Do not commit or push** — the user handles all git operations (commit, push, tag)
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not add external dependencies — first-party only (sakshi + cyrius stdlib)
- Do not skip benchmarks before claiming performance improvements
- Do not skip fuzz verification before claiming a parser works
- Do not hardcode toolchain versions in CI YAML — read `cyrius.cyml`
- Do not commit `build/` — it's regenerated per-build
- Do not re-vendor stdlib or first-party deps into `src/` — `cyrius deps` (git deps) and
  `cyrius lib sync` (declared stdlib subset) manage `lib/`. Sync the declared subset, never a
  full snapshot — that over-vendors unused platform variants and breaks `deps --verify`
- Do not shell out to `cycc` directly — always go through `cyrius <subcommand>`
- Do not use `sys_system()` with unsanitized input — command injection risk

## Documentation Structure

```
Root files (required):
  README.md          — quick start, features, dependency stack, consumers, license
  CHANGELOG.md       — per-version changes (Added/Changed/Fixed/Removed)
  CLAUDE.md          — this file (development process, principles, DO NOTs)
  CONTRIBUTING.md    — fork, branch, cleanliness, PR workflow
  SECURITY.md        — supported versions, scope, reporting
  CODE_OF_CONDUCT.md — Contributor Covenant
  LICENSE            — GPL-3.0-only
  VERSION            — current semver
  cyrius.cyml        — package manifest (toolchain pin, deps, build)
  benchmarks.md      — rendered benchmark trend table (regenerated by
                       scripts/bench-history.sh; NOT hand-edited)
  bench-history.csv  — benchmark regression history, one row per run per
                       benchmark. The `stat` column names which statistic
                       `estimate_ns` holds; rows written before 2.9.2 have no
                       value there and are `max`

docs/ (required):
  doc-health.md      — living doc-currency ledger (fresh/stale/read-through/dated
                       per file); refresh the affected row whenever a doc is touched
  architecture/
    overview.md      — module map, data flow, consumers, dependency stack
    math.md          — (when applicable) mathematical reference for algorithms/formulas
  development/
    roadmap.md       — completed items, backlog, future features (demand-gated), v1.0 criteria

docs/ (when earned — not scaffolded empty):
  adr/
    NNN-title.md     — architectural decision records (when non-obvious choices are made)
  development/
    threat-model.md  — attack surface, mitigations (when security-relevant)
    dependency-watch.md — deps to monitor for updates / CVEs
    port-audit.md    — Rust-era → Cyrius parity audit
    issues/          — one file per real bug (in-tree or upstream toolchain);
                       shipped ones move to issues/archived/
  guides/
    usage.md         — patterns, philosophy, code examples (NOT YET EARNED —
                       only testing.md exists; see doc-health.md)
    testing.md       — test count, coverage, testing patterns
  audit/
    YYYY-MM-DD.md    — dated audit reports (security, perf, correctness)
```

Benchmarks live at the repo root (`benchmarks.md`, `bench-history.csv`), not under
`docs/` — there is no `docs/benchmarks/`. Do not create a second benchmark surface.

ADR format:

```
# NNN — Title
## Status: Accepted/Superseded
## Context: Why this decision was needed
## Decision: What we chose
## Consequences: Trade-offs, what changes
```

## CHANGELOG Format

Follow [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

## [Unreleased]
### Added — new features
### Changed — changes to existing features
### Fixed — bug fixes
### Removed — removed features
### Security — vulnerability fixes
### Performance — benchmark-proven improvements (include numbers)

## [X.Y.Z] - YYYY-MM-DD
### Added
- **module_name** — what was added and why
### Changed
- item: old behavior → new behavior
### Fixed
- issue description (root cause → fix)
### Performance
- benchmark_name: before → after (−XX%)
```

Rules:
- Every PR/commit that changes behavior gets a CHANGELOG entry
- Performance claims MUST include benchmark numbers
- Breaking changes get a **Breaking** section with migration guide
- Group by module when multiple changes in one release
- Link to ADR if a change was driven by an architectural decision
