# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.2**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v2.13.0

Suite **3574** across five harnesses (hisab 416, foundation 351, modules 1798, edge_cases 233,
abuse 776), constant gate **159/159**, **72** benchmarks, **35** `[lib]` modules, toolchain
**6.6.2**, sakshi **2.5.1**, ganita **1.2.4**, and **zero** deprecated-alias call sites. All gates green:
`lint` 0 warnings and `fmt <file> --check` 0 drift across all 44 sources, `vet` 2 deps / 0 untrusted
/ 0 missing, `deps --verify` 31/31, `fuzz` 1/0, `coverage` 640/644 (99%) functions over 36/36 files,
distlib in sync.

**Per-release detail lives in `CHANGELOG.md`**, and the one-line-per-version record is the Release
History table at the foot of this file. What belongs *here* is the part that generalises.

> ⭐ **Verified 2026-09-09 against the tree, and 18 of 39 items did not survive it.** Every open item
> in this file was handed to an independent verifier told to *try to prove it already done*: 21 came
> back genuinely open, **15 rested on a premise that is now false**, and 3 were finished. The stale
> ones are struck through below with the proof that killed them, rather than deleted — several were
> false in a way that would have sent someone to do the **wrong work**, and that is worth keeping
> visible. Evidence base: [`audit/2026-09-09-roadmap-verification.md`](../audit/2026-09-09-roadmap-verification.md).
>
> ⚠ **This file had drifted into the exact failure it narrates.** Its own header said toolchain
> 6.5.33 while line 18 said 6.6.2 — contradicting itself fourteen lines apart. Corrected here, and
> the remaining counter drift is listed in the verification report rather than trusted.

### The standing lesson of the 2.7.0–2.11.2 arc

**Every defect of consequence was cheap to find and invisible because nothing looked.** The friction
impulse had been identically zero at every mu since before the 2026-08-04 audit, surviving because
every solver fixture used friction = 0. The delaunay ghost-direction invariant — the property a
CRITICAL was rebuilt around — had zero assertions. `cqr_decompose` overran its output buffer while
returning a success code. None needed cleverness; they needed something that looked.

**2.9.2 extended that from the code to the gates.** Three were green while checking nothing, and not
one was found by a test failing: CI's version gate ran an unanchored `grep` that matched a byte
count in a benchmark line, so a release with no CHANGELOG section would have passed; `src/main.cyr`'s
CLI version string was compared against nothing; and `bench-history.sh` recorded each benchmark's
**max** rather than its average, in every row it had ever written.

**2.10.0 onward turned it into a procedure that keeps paying.** Run the feature's own correctness
check against the code that already ships, *before* writing the feature — it found a defect in four
consecutive releases:

| release | the check | what it found in shipped code |
|---|---|---|
| 2.10.0 | degree −1 homogeneity in the ray direction | `geo_ray_sphere` returning a hit point 3.74 from the centre of a unit sphere |
| 2.10.1 | "what does the primal return when there is no face?" | `geo_ray_aabb`/`_obb` returning `+Inf`; a squared-vs-unsquared epsilon **2.10.0's own repair introduced** |
| 2.10.2 | the hit point must lie on the box | a scale-free slab test hitting at x = 1.4 for a box spanning [0,1] with a **unit** direction |
| 2.11.0 | reverse mode is validated against forward mode, so sweep forward mode first | five defects in the duals, incl. `ln(−5)` returning a NaN value beside a **confident** −0.2 derivative |
| 2.11.1 | *(no feature — a full audit instead)* a mechanical grep for the class the last four releases kept repairing | the same guard defect at ~24 more sites, `cx_div` among them — the module whose own comment records the lesson |

Three corollaries, each learned by being caught out:

* **A test written by the author of the code inherits the author's list of cases.** An independent
  derivation commissioned against 2.10.1's finished code confirmed all 37 partials and 0 of 24,237
  FD comparisons failed — its entire value was in the **enumeration**, naming two degeneracies the
  design had not.
* **A guard cannot be reached by differencing the function it guards.** At every input where one
  fires, the perturbed scalar is NaN too, so the sample is skipped. 2.11.0's FD sweep reported clean
  while skipping exactly the five rows that mattered.
* **A threshold that needs a scale chosen for it is the defect.** One rule — *guard exactly what
  makes the division fail and nothing more* — settled six thresholds across 2.10.2 and 2.11.0.
* **A lesson written beside the code that taught it does not reach the other thirty-four modules.**
  2.11.1's audit found the 2.6.14 squared-epsilon class at ~24 further sites, including `cx_div` —
  which is *named in the comment recording the lesson* and still fabricates zero. Only a grep
  applies a rule tree-wide; a review applies it where someone happened to look.

⚠ **Four separate measurements stated in committed text did not reproduce** during this arc, each
caught by an adversarial reader rather than a gate, and two had already propagated into three or four
files. `scripts/check-measurements.sh` closes that class and is enabled, PR-only and scoped to the
claims a branch *adds*. **Kept** — the open question is now only whether it should run on push as well as on PRs; see **Open items** below.

## Release train

What each release **delivers to a consumer**. Released versions are not listed here — their record
is the CHANGELOG and the Release History table at the foot of this file. Defects are not roadmap
items: they are tracked in `issues/` and discharged as a **precondition** of the release they gate.

| Release | Deliverable | Gated on |
|---|---|---|
| ~~**2.12.0 — the safety release**~~ | ✅ **SHIPPED.** The abort tier (4 entry points) + the allocation tier — which was **4 sites, not the 2 this row estimated**. ⭐ Two were found by refusing to trust the list: one by a **mutant that did not die**, one by grepping for the shape after the first was repaired. | — |
| ~~**2.13.0 — the suite release**~~ | ✅ **SHIPPED.** 842 truncating assertion sites -> **16**; `foundation.tcyr` 89% -> **0%**. ⭐ The migration was self-verifying, and the four sites that failed on conversion were four different real defects. Named tolerance helpers added to all five suites. | 2.12.0 |
| ~~**2.14.0 — the epsilon release**~~ | ✅ **SHIPPED, PARTIALLY.** ⛔ **This row said "~20 sites"; the census found 136 guards and 97 confirmed defects in 23 modules** — the estimate came from the 2026-08-11 audit's *confirmed* table, which is a list of instances someone reproduced, not a census of the class. **23 confirmed sites repaired, 58 mutants killed, +83 assertions.** ⭐ Three repairs were NOT on the census list and came from grepping for the shape. **74 remain, enumerated with evidence.** | 2.13.0 |
| ~~**2.15.0 — the epsilon tier, remainder**~~ | ✅ **SHIPPED.** **73 of the 74 repaired**, 81 mutants installed and **73 killed**; 8 survivors documented with their reasons rather than tidied away. ⛔ **1 DEFERRED — `su2_log`, and it is a formula change, not a threshold**: it divides by θ² and θ³, and at θ ≤ 2.2e-162 both `θ*θ` and `1−cos θ` are exactly 0, so lowering the guard would make the coefficient `0/0 = NaN`. ⭐ **Three siblings of it were never on the census list** (`so3_log`, `se3_exp`, `se3_log`) and were found by grepping for the shape. | 2.14.0 |
| ~~**2.16.0 — the small-angle series**~~ | ✅ **SHIPPED.** All four maps repaired; 7 mutants, 5 killed, 2 documented equivalences that exist BECAUSE the repairs made each other redundant. ⛔ **The log maps had a larger defect the guard was hiding**: `acos` of a value that rounds to exactly 1.0 below θ ≈ 1.5e-8, so the whole rotation was lost — measured, exactly 0 from 2^-28 down. `atan2` recovers it bit-exactly. ⭐ **Round-trip floor 2^-26 → 2^-537, 511 decades.** | 2.15.0 |
| **Scaled norms on the exp side** | ⛔ Found in 2.16.0 and NOT closed by it: `su2_exp` and `so3_from_axis_angle` still take their norms as a naive sum of squares, which flushes to zero below ~2^-511 — so they collapse the rotation at **2^-538**, before the repaired log maps ever see it. Same class as `cx_div` and the same fix (scale by the largest component first). | 2.16.0 |
| **SVD factor reconstruction** | ⛔ Found in 2.15.0 and NOT closed by it: `‖A − U S Vt‖_F` is exactly 0 for block ratios 1e-2…1e-5, then **4.17e-7 at 1e-6**, decaying proportionally to c. The singular VALUES stay correct — it is `U` and `Vt` that stop reconstructing the small block, so nine threshold repairs do not touch it. 4.17e-7 reproduces the census's own figure for a half-repaired bidiagonalisation. | 2.15.0 |
| **3.0.0** | `Result<T,E>` API — breaking. ⚠ Re-scope before planning: it is the **v6.6.0 value form**, not the boxed form this file was written against | 2.16.0 |

### ⛔ Why the epsilon tier is no longer first, and why its old gate is gone

**The old gate was unexecutable.** 2.12.0 used to be gated on "the 28 unverified audit findings re-run
first — this gates every item below it". **The 28 were never enumerated.** `audit/2026-08-11-v2.11.0-full.md`
records them only as a count (`7 critical, 16 high, 22 medium, 7 low` across all 52); its only
per-finding tables are explicitly the *confirmed* and *fixed* sets. There is no ledger in `docs/`,
none in git history (the report was committed whole and never revised), none in scratch. The 2
REFUTED are also unidentified, so a re-run cannot even subtract the known-bad ones.

⭐ **And it would not have bound anything even if it could be run.** The epsilon-tier sites come from
the table headed "**Confirmed** instances" — the 21 skeptic-confirmed set, which is *disjoint* from
the 28 unverified. The alloc and abort sites were reproduced from a cold start on HEAD **with
discriminating controls**, which is strictly stronger evidence than the skeptic pass the gate
demanded. The suite tier is not a `src/` change at all. **A gate that blocks four tiers it has no
evidential relationship to is not caution, it is a deadlock.** Retired.

**The new ordering is forced by a measurement, not a preference.** Every epsilon repair is a change
to a threshold below 1.0. **843 of 3247 assertion sites compare through `f64_to`, which truncates** —
`assert_eq(f64_to(1.9999), f64_to(1.0))` **passes**, so an absolute error of 0.9999 is invisible to
them. Repairing thresholds first would mean landing sub-1.0 behaviour changes into a suite that
structurally cannot see them. The abort and alloc tiers do not have this problem: their failure mode
is a signal (a SIGSEGV, an abort, a clobbered canary), not a digit, so they are provable *through*
the truncation and can go first.

⚠ **What replaces the retired gate**: either commission a **fresh sweep of the 2.11.5+ tree** — the
better buy, since the findings are five releases stale and the 2026-08-11 audit never reached
`symbolic*`, `lie*`, `spatial`, `color`, `noise_simplex`, `einsum`, `tensor`, the SIMD paths or any
cross-module interaction — or verify **per-site inside the epsilon tier**, where the prescribed
scale-covariance assertion *is* the verification and the sites sit at known `file:line`.

⚠ **One reconciliation is owed before 2.14.0**: 21 CONFIRMED − 4 repaired in 2.11.1 = **17 confirmed
findings** that the tiers below never account for *by count* (the epsilon tier is sized "~20 sites",
mixing confirmed with unverified). A confirmed finding can fall between buckets and nothing would
notice. Reconcile the tier lists against the audit's confirmed table, by row, once.

---

## Open items

Everything still owed, in one place. Each carries why it has not been done, because "deferred with a
reason" and "forgotten" are indistinguishable once the reason is lost.

### The 2026-08-11 audit backlog

The full P(-1) sweep of the 2.11.0 tree is
[`audit/2026-08-11-v2.11.0-full.md`](../audit/2026-08-11-v2.11.0-full.md): 6 dimensions, 115 checks,
**52 findings reproduced**, 21 CONFIRMED by an independent skeptic, 2 REFUTED, 1 already known. 2.11.1
executed four repairs; the rest is here, **reordered 2026-09-09** — see the release train for why the
epsilon tier is no longer first.

- [ ] ~~**Re-verify the 28 findings that were never sent to a skeptic.** … **This gates every item
      below it.**~~ ⛔ **GATE RETIRED 2026-09-09 — it could not be executed and it bound nothing.**
      The 28 are **not enumerated anywhere**: the audit report records them only as a count, and its
      only per-finding tables are explicitly the *confirmed* and *fixed* sets. No ledger in `docs/`,
      none in git history, none in scratch; the 2 REFUTED are unidentified too, so a re-run cannot
      even subtract the known-bad. ⭐ And the tiers it claimed to gate draw from the *confirmed* set,
      which is **disjoint** from the 28. Replaced by a choice, recorded in the release train: a fresh
      sweep of the current tree, or per-site verification inside the epsilon tier where the
      scale-covariance assertion is itself the proof. **Keeping it would have blocked the audit's own
      highest-value item behind a task with no input.**
- [x] ✅ **The allocation tier — SHIPPED IN 2.12.0, and it was FOUR sites, not the 2 this item
      predicted.** `detect_islands`, `solve_gmres`'s Hessenberg, `solve_gmres`'s `_lext_copy` and
      `solve_bicgstab`'s `_lext_copy`. ⭐ The last two were not on any list: one surfaced because a
      **mutation test failed to kill** (the guard's stated overflow justification was unreachable —
      because an EARLIER unchecked alloc failed first), the other by grepping for the shape rather
      than assuming the tier was complete.

  ⚠ **A mechanical sweep for the whole class now exists and the number is bigger than anyone
  wrote down: 90 unchecked parameter-sized allocations tree-wide, of which 9 are PUBLIC entry
  points.** Two are repaired here; **seven remain** and are listed below rather than folded in
  silently. Original text: ⚠ The third was `halfedge_from_triangles`,
  which is **already owned by the abort tier below** and needs a *different* repair (its
  `vec_push` aborts rather than returning 0). Both real sites reproduced as SIGSEGV on HEAD with
  a discriminating control: `solve_gmres`'s Hessenberg (`linalg_ext.cyr:390` — quadratic in
  `restart`; m=16385 asks for 2,147,762,880 B, just over `ALLOC_MAX`, and the same n with
  `restart = 2` exits 0) and `detect_islands` (`collision_mesh.cyr:1263/1264/1338`). Mechanical,
  low risk, identical repair to the three `num_ext` sites closed in 2.11.1. **One API decision:**
  `detect_islands` returns a vec, not an rc, so it has no channel to report a failed allocation
  through — decide before repairing. **Scheduled: 2.12.0.**
- [x] ✅ **The abort tier — SHIPPED IN 2.12.0.** All four repaired, each reproduced with a
      discriminating control and mutation-proven. ⛔ One of the four was WORSE than an abort: The roadmap never named the three `collision_core` sites; they are named now, all
      reproduced live on HEAD. `convex_hull_2d` (`collision_core.cyr:488`), `triangulate_polygon`
      (`:653`) and `sequential_impulse` (`:262`) each index a vec with the **caller's `n`** and never
      compare it against `vec_len`; `halfedge_from_triangles` (`collision_mesh.cyr:1029`) hits
      `VEC_CAP_MAX` and calls `_vec_die`. A library must not end its caller's process.

  ⛔ **`sequential_impulse` does not even abort — it returns success after writing out of
  bounds.** Its zeroing loop runs `for zi < n` writing 16 bytes per contact **before the first
  `vec_get`**, so with `iterations = 0` it never touches the vec at all. Reproduced: an **empty**
  contacts vec, `n = 1000`, a 64-byte `out_impulses` and a 64-byte canary → **all 8 canary words
  clobbered, rc = 0, no diagnostic**. That is ~16 KB written past a 64-byte buffer with a clean
  exit — CWE-787 with no crash signal.
  ⚠ **The abuse suite has a canary block for this exact function** (`abuse.tcyr:1139-1146`) and it
  misses this: it tries `n` = 4, 0 and −2, every one of which is **in bounds** for the buffer it
  allocates. A canary only proves what its `n` reaches.
  **Repairs are O(1) `vec_len` comparisons — but the zeroing loop must be bounded too, not just
  the `vec_get` loop.** **Scheduled: 2.12.0.**
- [ ] **The remaining 7 public entry points with an unchecked parameter-sized allocation.**
      ⭐ **This item exists because 2.12.0 stopped trusting the tier's list and ran the grep.** The
      class is: `alloc(<caller parameter> * 8)` with no check that the result is non-zero. Sweep
      result — **90 sites tree-wide, 9 of them PUBLIC entry points**; 2 repaired in 2.12.0, these 7
      remain:

  | entry point | allocation |
  |---|---|
  | `tensor_new` (`tensor.cyr`) | `alloc(rank * 8)` |
  | `geodesic_state_new` (`diffgeo.cyr`) | `alloc(dim * 2 * 8)` |
  | `geodesic_rk4` (`diffgeo.cyr`) | `alloc(dim * 8)` |
  | `parallel_transport` (`diffgeo.cyr`) | `alloc(dim * 8)` |
  | `opt_conjugate_gradient` (`optimize.cyr`) | `alloc(n * 8)` |
  | `opt_bfgs` (`optimize.cyr`) | `alloc(n * 8)` |
  | `opt_lbfgs` (`optimize.cyr`) | `alloc(n * 8)` |

  ⭐ **THE TRIAGE WARNING BELOW WAS RIGHT, AND FOLLOWING IT SAVED SIX UNNECESSARY GUARDS.**
  `tensor_new` (`rank > 8`), `geodesic_state_new` / `geodesic_rk4` / `parallel_transport`
  (`_DG_MAX_DIM = 16`), `opt_bfgs` (three layers) and `opt_lbfgs` (both `n` and `m`) are each already
  capped **ahead of the allocation**, by bounds up to 8.4 million times tighter than an
  `ALLOC_MAX`-derived one would be. A guard sitting behind an existing stricter guard is dead code,
  not safety.
  ⛔ **One real defect, and it was not the class the sweep was looking for**: `opt_conjugate_gradient`
  and its neighbour `opt_gradient_descent` let `n * 8` wrap to a small **positive** value, so `alloc`
  SUCCEEDS and the existing null check is defeated. Repaired in 2.12.0 with a round-trip test rather
  than a cap — a cap was measured and rejected because it refuses a conforming caller at n = 1e7.

  Original text:
  ⚠ **Triage before repairing — several are probably FALSE POSITIVES.** Some of these functions
  already cap their dimension with a `_MAX` enum, which would make the allocation unreachable;
  the sweep's heuristic cannot see that. **Do not add a guard to a site that cannot fail** — this
  repo's roadmap has been damaged more by claims that did not survive the tree than by missing
  work. Each site needs: reproduce with a small buffer and a large claimed dimension (⚠ not a
  probe that allocates the dimension itself and crashes in the PROBE — that error was made twice
  during 2.12.0), a discriminating control, then a bound **derived** from `ALLOC_MAX` or the
  function's own existing cap.
  **Scheduled: 2.12.1**, since it is the same mechanical repair as the tier just shipped.

- [x] ✅ **The suite tier — SHIPPED IN 2.13.0, and the justification was measured, not argued.**
      Re-counted at HEAD: **843 of 3247 assertion sites (26.0%)** compare through `f64_to`, which
      **truncates** — the audit's 836 has grown, not shrunk. ⚠ The often-quoted "23.8%" divided a
      *static* numerator by a *dynamic* denominator (3510 executed assertions); against static sites
      it is 26%.
      ⭐ **The load-bearing claim is provable by execution, not prose**: `assert_eq(f64_to(1.9999),
      f64_to(1.0))` **passes**, so an absolute error of 0.9999 is invisible.
      ⚠ It is not evenly spread, and that changes how to attack it — `foundation.tcyr` is **89%**,
      `edge_cases` 49%, `hisab` 33%, `modules` 19%, `abuse` 12%. This is far more a single-file
      problem than a tree-wide one. Only **21 of 843** scale their comparison first, and **three of
      five suites define no tolerance helper at all** (`hisab`, `modules`, `abuse`) — so the campaign
      is *add the helper, then convert*, not *convert*.
      Also in this tier: 38.4% of value-changing single-operator mutants survive all five suites, and
      29 public functions are covered only by `assert_neq(f(...), 0)`.
      ⚠ **Fresh evidence it is still biting**: 2.11.4 found **two assertions that had gone silently
      vacuous** — they compared through a tolerance and a round, so when upstream repaired the defect
      they pinned, they kept passing while the property they claimed to test evaporated.

- [ ] **The epsilon tier — the count is wrong, the grep it prescribes has never been run, and it
      is LAST rather than first. Scheduled: 2.14.0.**
      ⚠ "~20 sites" came from a *review*. The mechanical grep the item itself asks for returns
      **123 `f64_(lt|gt|le|ge)(…, EPSILON_F64)` comparison guards across 24 of 35 modules**
      (`geo_advanced` 22, `linalg_precision` 17, `lie` 12). Not all are defects — **nobody has
      triaged which guarded quantities actually scale**, and that triage is the first task, not the
      repair. ⚠ The DBL_MIN rule 2.10.2 settled reaches **three of thirty-five modules** (geo, autodiff, quat)
      under three different names — propagated once, by 2.11.1, and never given a shared constant. ⛔ `cx_div` is named in `complex.cyr:58`'s own lesson comment **and
      still fabricates zero**, which is the whole point of the standing lesson above. Named sites: `eigen_qr`, `cqr_decompose`,
      `solve_bicgstab`, `solve_gmres`, `m3_inverse`/`m4_inverse`, `cmat_inverse`, `svd_golub_kahan`,
      `cx_div`/`cx_inv`/`cx_powf`, `hvec3_angle`, `hvec2/3/4_normalize`, `cga_blade_inverse`,
      `m4_transform_point`, `calc_bspline`/`calc_nurbs`, `calc_monotone_cubic`, `num_newton`,
      `num_tridiag_solve`, `geo_barycentric_coords`, `geo_ray_triangle`, `sectional_curvature`,
      `hisab_inverse_lerp`/`hisab_remap`. Each gets a **scale-covariance** assertion bracketing every
      threshold a repair might plausibly have chosen. ⚠ **Not one commit** — each site is a behaviour
      change on a documented entry point, and this tree's history says these land in mutation-proven
      bites.

⚠ ~~The audit was correctness-shaped and did **not** reach `symbolic*`, `lie*`, `spatial`, `color`,
`transforms`, `noise_simplex`, `einsum`, `tensor` … or any cross-module interaction — and nothing in
it audited **performance regressions** or the **benchmark harness**.~~ **Mostly stale, and it
contradicts its own source.** `transforms` is listed as not-reached while **line 58 of the same
report files a finding against `hisab_inverse_lerp`** (`src/transforms.cyr:119`). Ten of the eleven
named modules were audited eight days earlier by the 2026-08-03/08-04 sweeps, whose §5 is titled
"Recorded so the next audit does not re-plough it". The benchmark-harness clause was discharged by
2.11.2, 2.11.4 and 2.11.5.
⭐ **TWO clauses survive, and an earlier draft of this correction silently dropped one of them.**
The ellipsis in the struck quote above elided `the SIMD f64v_* paths`, which is still a real gap —
the 2026-08-03 sweep reached only `hvec3_*`, never the 27 `f64v_*` call sites across mat4/vec4/quat/
mat3/vec3. ⚠ **Deleting a clause while striking a list is how a gap stops being tracked**, which is
the same failure the strike was correcting.
The other survivor is **"any cross-module interaction"**. ⚠ An earlier draft said no audit report
"has ever contained that string" — false: both full sweeps name it, in their own *did NOT reach*
sections. It has been declared out of scope twice, which is a stronger claim than never mentioned. Those two are where the next sweep starts.

### EPA — one routine, three entangled questions

~~All three touch `gjk_epa_*` and none should be done alone: they share a benchmark and a live filing.~~
⚠ **Refuted 2026-09-09 — they are not entangled.** `_epa_degenerate_normal` has a single caller behind
a double cold guard, and none of the **four** EPA benchmarks can enter that branch, so the squared-epsilon
sites do **not** share a benchmark with the other two. The three questions are independent and one of
them is not a defect at all.

- [ ] **The three squared-epsilon sites.** `geo_advanced.cyr:1056/1060/1067` compare
      `hvec3_length_sq` against `EPSILON_F64` — the same squared-vs-unsquared mistake 2.10.1 and
      2.10.2 repaired at six sites in `geo.cyr`. **Deferred, not dismissed**: tightening them is a
      narrowphase behaviour change on a routine with a measured cost, so it needs its own
      before/after rather than riding along with a geometry release.
      ⚠ **The stated blocker is wrong; the two real ones are:** `geo_advanced.cyr` has **no local tiny
      constant** — a repair needs `_GA_F64_TINY = DBL_MIN` plus registration in
      `scripts/check-constants.sh` — and `grep -rn "degenerate_normal" tests/` returns **zero**, so a
      repair currently has nothing to prove itself with. Write the test first.
      The disposition table for all nine sites is in
      [`issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md`](issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md).
- [ ] ⚠ **The seed-upgrade trade — DO NOT ACT ON THE NUMBER BELOW; it is void.** The "+19.4%" was
      priced against a `gjk_epa_sphere_box` baseline of **125.6 µs** which now reads **78.5 µs**
      (cycc 6.6.x accessor inlining). Its own linked filing has said since 2026-09-09: *"Do not
      rescale the old percentage — re-measure both halves."* Both the baseline and the added work
      moved for the same reason and neither was measured after the move, so **the price of this trade
      is currently unknown** and the decision cannot be taken. Re-measure first. Original text:
      Giving `_epa_seed_gjk` the strictness upgrade `_epa_seed_portal`
      already performs closes the certification asymmetry between two public entry points and makes
      six previously-unmutatable EPA repairs observable — at a measured **+19.4% on
      `gjk_epa_sphere_box`**, for a change that alters no returned answer. 2.9.3 measured it and
      declined. A judgement call, not a defect; the change is one contained edit.
- [x] ~~**Sphere-family non-convergence**~~ ⭐ **ROOT CAUSE ESTABLISHED 2026-09-09 — and it is not a
      defect, so there is nothing to repair.** Holding the seed at the strict portal seed and varying
      **only curvature** gives a monotone series: a smooth sphere hands off **12** times, a 12-vertex
      icosahedron inscribed in that same sphere **4**, a box **0**. Certification requires
      `best_d - lo <= 1e-10 * max(...)` within `EPA_MAX_ITER = 64`; on a strictly curved boundary the
      achievable relative gap at ~132 faces is ~**1.5e-2**, seven orders away. **A polytope expansion
      cannot certify a smooth surface** — the handoff is the algorithm working, not failing.
      ⚠ **And the objection quoted against it is measured on the wrong shape**: the "+55% pays and
      gains nothing" verdict cites `gjk_epa_sphere_box`, a sphere-vs-**box** pair, which certifies
      **0 of 12** under the strict seed — i.e. the benchmark carrying the objection is one that
      *gains*. Original text, kept for the record: the genuinely open half of that filing and independent of
      the certificate wording: spheres do not certify *even with a strict seed*, and they pay the
      upgrade's cost without gaining anything. **Root cause not established.** This is what the
      filing is actually about now.
      → [`issues/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md`](issues/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md)

### Decisions owed

- [ ] **`scripts/check-measurements.sh`: keep it or delete it.** The gate runs PR-only on the claims
      a branch *adds*, detector recall 21/22 with 0/15 decoys, but paragraph-level false positives
      sit at ~21% and **522** unmarked pre-existing claims mean it cannot be widened past the diff
      without a marking campaign first. It has now earned its keep several times over — it caught
      the arc's fourth non-reproducing measurement, and it has forced provenance markers onto every
      claim added since 2.9.3. The open question is whether ~21% FP is tolerable on a gate people
      must not learn to ignore.
      ⭐ **The deciding question is MOOT, and the answer is keep.** The gate is wired
      `if: github.event_name == 'pull_request'` (`ci.yml:238`) and **this repo has zero pull
      requests** — 270 workflow runs since 2026-03-22, every one `event: push`, no merge commits,
      one branch. The job has reported `skipped` in all 51 runs since it was wired, 2.11.5 included.
      **Nobody has ever been shown a false positive**, so the ~21% FP rate has never cost anything.
      ⚠ And a full-tree scan today reports **2 BROKEN PROVENANCE MARKERS** which **no other gate
      catches**, in the mode CI does not run. ⛔ **An earlier draft of this bullet misdiagnosed
      them**: it said both "cite paths that have since moved to `archived/`", but
      `tests/modules.tcyr`'s marker already names an `archived/` path and that file exists. The real
      cause is that `check-measurements.sh` resolves marker paths **from the repo root** while hisab
      writes them relative to `docs/development/` — so the missing prefix is `docs/development/`, and
      the implied fix would have left both markers broken. A wrong diagnosis on a two-line repair is
      worse than none. The real
      decision is smaller and different: **should it run on push, not only on PRs?**
- [ ] **Whether `dual_*` should gain a vector layer.** 2.10.0 answered "no dual vectors" for the
      *geometry*, on measured grounds (allocation per op, one seed per pass), and 2.11.0's
      reverse-mode tape removed the pressure entirely for the many-input case — one sweep instead of
      n passes. What remains is whether a *small fixed-size* vector dual is worth it for callers who
      want a Jacobian rather than a gradient. **No consumer has asked** — and now measured:
      **0 call sites of `dual_*` or `ad_*` across all 14 hisab-consuming repos.** The scalar API this
      would extend has no external users either, and the one in-tree Jacobian contract
      (`optimize.cyr:652`) is already satisfied by per-root reverse sweeps. Leave parked.

### Documentary

- [ ] **Two items inside the archived `triangulate_polygon` filing** — the 2.7.x CHANGELOG entries
      state the prune's divergence direction backwards on at least one reproducer, and the
      neighbour-refresh locals shadow the winding loop's `pn`/`nn`. Neither changes behaviour; both
      were recorded rather than buried when the file was archived.
      ⭐ **Both confirmed 2026-09-09, and both citations in this bullet were wrong.** The direction
      claim reproduces on 6.6.2 — `(0,0),(1,2),(1,1),(0,2),(2,1)` gives old **9** indices (COMPLETE)
      → new **6** (PARTIAL), i.e. the CHANGELOG has it backwards, and the *same release* already
      states it correctly 630 lines apart (`CHANGELOG.md:3508`). ⚠ The shadow is at
      **`collision_core.cyr:762,764`**, not the `740,742` this bullet and the archived filing both
      cite — and the winding loop has no `nn` at all, so only `pn` is a true shadow. A citation that
      drifted is how a five-minute fix becomes an investigation.
      → [`issues/archived/2026-08-04-perf-triangulate_polygon.md`](issues/archived/2026-08-04-perf-triangulate_polygon.md)

### Toolchain, tracked upstream

**One filing is open here** (`docs/development/issues/`, **26** filings archived beside it, plus a README). ⚠ **Cyrius bugs are filed in the CYRIUS repo**, not this one — `cyrius/docs/development/issues/` is where the language agent reads them; only hisab's own items and hisab's record of a live upstream workaround belong here:

| filing | state |
|---|---|
| ~~upstream: `2026-09-09-hisab-derive-accessor-simd-dst-slot.md`~~ | ✅ **FIXED in cycc 6.6.2 and archived upstream** — the pin is 6.6.2, so this row is discharged. ⚠ The workaround hoist in `src/mat4.cyr` is **no longer load-bearing** (verified: removing it passes hisab 416 / foundation 351 / abuse 741, rc=0) and is kept only for consistency with `m3_mul_vec3`. ⛔ **This row also propagated a framing upstream REFUTED**: it was filed as derive-specific and as a 6.5.71 regression and was **neither** — all 21 SIMD handlers were affected, reachable since 6.0.70; 6.5.71 only removed the call that had been forcing the spill. **A first-bad-version is evidence about visibility, not origin.** |
| `2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md` | ⚠ **hisab's own, not a toolchain item** — the EPA tier above. |

#### Capabilities 6.5.19–6.5.33 opened, filed rather than taken in 2.11.2

A toolchain bump is not the place to adopt features, so these were verified as available and left.
Each is a real, checked capability, not a hunch — the ones that were checked and found **not** to
apply are named too, so a later reader knows they were examined rather than skipped.

| item | state |
|---|---|
| **Negative enum values** (6.5.32, `enum E { NONE = -1; }`) | `src/error.cyr` carries **11** negative error codes as `var` globals with the `(0 - N)` workaround, and CLAUDE.md's own principle is "enums for constants — zero `gvar_toks` cost vs. `var` globals". 562 `HSB_ERR_` references across `src/` + `tests/`. ⛔ **"Gate first" is BACKWARDS — the gate does not check these constants at all.** `check-constants.sh`'s value regex requires a >=12-digit hex literal; all 12 `HSB_ERR_` lines fail it and contribute **0 of the 159 verified**. Converting moves them from a form cycc is *silent* about (a duplicate `var` global is last-wins, and the wrong value ships) into one it **diagnoses by file:line**. So convert freely; optionally widen `check-constants.sh:237`'s `GLOBAL_DECL` afterwards so the duplicate scan stays fail-closed. 12-line edit; the 562 call sites reference by name and are unchanged. |
| **`CYRIUS_PKG_VERSION`** (6.5.21) | Would let `src/main.cyr` stop hardcoding its printed version — the one version site `${file:VERSION}` cannot reach, and the site the 2.9.1 → 2.9.2 bump silently missed. ⚠ **Weigh against the gate it removes**: CI asserts that literal independently today. ⚠ **That caveat is stale**: the included-file case was fixed in 6.5.34 and re-verified on 6.6.2 — `CYRIUS_PKG_VERSION` resolves from the entry file **and** from an included file. ⛔ Separately, `scripts/version-bump.sh:21` and `ci.yml:285` both still assert *"Cyrius has no build-time string interpolation"*, which has been **false since 6.5.21**. Fix that sentence whether or not the symbol is adopted. |
| **`bench_run` auto-batching** (6.5.19) | Already in force — it is what moved the 44 rows. The **36 `bench_batch()` call sites are deliberately unchanged**: they now buy a FIXED window rather than escape the floor, which is still worth having when comparing two runs at identical batch sizes. Re-evaluate only if a reason appears. |
| **A `load` column for `bench-history.csv`** (6.5.25 documents the signature) | Upstream records that a **~4% move in both metrics together** is box-wide contention, not a code change. hisab records neither load nor any second metric, so it cannot currently tell the two apart. ⚠ **Nothing technical blocks it; what is missing is a RULE THAT READS IT.** Upstream ships a prose ~4% heuristic and has no load column of its own, and the guard hisab actually adopted in 2.11.5 — re-run the identical binary and compare each row to **its own** spread — already settles the cases this would. A load snapshot at run *start* does not describe load *during*. ⭐ **The distinct and more useful item is a per-benchmark noise band** recorded in `benchmarks.md`, replacing its global +-10% filter; 2.11.5 did that by hand. File that instead. |
| **`_sym_render_f64` is a hand-copied `fmt_float_buf`** | 6.5.30's carry fix is now vendored in `lib/fmt.cyr`, making hisab's private copy a redundant duplicate carrying its own now-stale rationale comments. ⛔ **"Pure debt, zero output change" is REFUTED.** Over 30,149 probed inputs, **30 diverge**, all at \|val\| >= 2^63: `sym_to_latex(1e19)` gives `-9223372036854775808.000000` today versus a malformed `-9223372036854775808.-9223372036854775808` after deletion — and `symbolic_ext.cyr:211` routes \|val\| >= 1e15 down that branch **on purpose**. ⚠ The 41,998-input equivalence run is **not reproducible from the tree** (`git log -S"41,998"` finds only the commit that filed the claim). This is a **behaviour decision now, not a cleanup**. |

#### Closed by the 2.11.3 bump — both re-tested on 6.6.1, both fixed upstream

| filing | outcome |
|---|---|
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | 🟢 **Archived.** Carried four months as "deliberately never re-tested" because the reproducer destroys a source file. 6.6.1 guards it: `error: refusing to write build output over a .cyr source file`, exit **1**, source byte-identical across three argument shapes. ⚠ The exit code needed a second look — `cyrius … \| tail` reports 0 because `$?` after a pipe is *tail's*. |
| `2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md` | 🟢 **Archived.** The underscore discriminator is gone and **`lint` — the half this was left open for — now catches it**. Control run: a *clean* uncalled no-underscore fn still passes `build`/`lint`/`vet`, so the probe discriminates rather than merely reddening. `vet` still exits 0 on unparseable input, deliberately not claimed as a defect: it is the dependency auditor, not a syntax gate. |

#### Opened by the 2.11.3 bump

| item | state |
|---|---|
| ~~**Give the benchmark harness resolution**~~ | 🔴 **REFUTED in 2.11.4, not built.** The premise was mine and it was wrong: the `net/floor` ratio compares a **per-op** net against a **per-clock-pair** floor, while `bench_run` sizes every batch so the clock is 1% of the window. Measured on a same-binary re-run, the tier it condemned is the **quieter** one (median 1.43% vs 2.10%). ⚠ It had suppressed most of 2.11.3's real result — `ray_aabb` −54.8%, `vec3_cross` −54.0% — so **a wrong instrument suppresses real findings as readily as it invents false ones**. The guard that needs no threshold: re-run the identical binary and measure the spread. |
| ~~**Migrate off ganita's deprecated aliases**~~ | 🟢 **DONE in 2.11.4** — 536 call sites over 20 of the 53 deprecated names, not the **8** this row estimated. ⚠ **The estimate was scoped from ganita's changelog paragraph rather than from its surface**, and was wrong by 67x; the deprecation block is a 53-entry table further down the same file. Suites byte-identical across the change; `svd_golub_kahan_12` **−27.85%** and `eigen_qr_12` **−27.67%** because the aliases were real `call`/`ret` pairs (`#inline` needs ≤2 params; `ganita_mat_get`/`_set` take 3 and 4). |
| ~~**A `load` column for `bench-history.csv`**~~ | ⛔ **SUPERSEDED — see the capability row above, which supersedes it with the opposite disposition.** Two rows for one item, disagreeing, is exactly what this file exists to prevent. What should be filed instead is a **per-benchmark noise band** in `benchmarks.md`, replacing its global ±10% filter: distinct from a load column (one separates box contention, the other replaces the filter), and 2.11.5 and 2.12.0 both did it by hand. Original text: this release saw 40 of 72 rows move in the same direction at once, and distinguishing "box-wide contention" from a real change had to be done by hand, using the flat numeric kernels as an ad-hoc control. A recorded second metric would make that mechanical. |

---
## Optional, demand-gated

- ~~**GPU compute via soorat** (feature-gated) — no consumer has asked.~~ ⛔ **DELETE — this row is
  pre-port and cannot be executed as written.** It was authored **2026-03-27**, for *Rust-era* hisab
  (`Cargo.toml` deleted 2026-04-15; `cyrius.cyml` added 2026-04-26), and "feature-gated" names a
  Cargo `[features]` key **the Cyrius package format does not have**. soorat today is 42 `.rs` files,
  **0 `.cyr`**, no `cyrius.cyml` — depending on it needs FFI, which `CLAUDE.md` forbids outright.
  ⭐ A superseding route already exists if a driver ever appears: **mabda** is ported and folded into
  the toolchain (`cyrius/lib/mabda.cyr`, opt-in include).
- **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4) — consolidation onto stdlib, not a fix.
  ⛔ **"exactly one hand-rolled sort" is wrong by 6x — there are SIX ordering routines**, in five
  files: `collision_core.cyr:466`, `spatial.cyr:114`, `num_ext.cyr:309`, `linalg_ext.cyr:905`,
  `linalg_precision.cyr:816` and `:1265`. Three are near-identical descending-magnitude selection
  sorts whose own comments concede they have already disagreed once.
  ⭐ **So the wait-for-the-third-instance gate is DISCHARGED** — this is the sixth instance, not the
  first. ⚠ The same wrong count survives at `CHANGELOG.md:4275`, beside the already-refuted
  "Cyrius has no closures" at `:4279` (and again at `:3848`); `dependency-watch.md:184` was
  corrected 2026-09-09. ⚠ **Every line number in the original of this sentence was stale — and so
  were the ones that first replaced them.** A remediation instruction whose citations have drifted
  sends the next reader to the wrong line, which is worse than giving no citation at all.
  ⚠ **This entry has now been wrong twice, in opposite directions, and both times by not running
  anything.** It first read "and Cyrius has no closures" — false since v6.3.8, propagated to four
  files. It was then corrected to a *measured* block: on 6.5.16 a capturing closure SIGSEGVed when
  passed through a function and called there, which is exactly this shape. **That was fixed in
  6.5.17 and re-verified on 6.5.18** (42 both directly and across the boundary), so the closure
  block is gone too.
  What actually remains is the plain API mismatch: `vec_sort_by` invokes its comparator as
  `fncall2(cmp, elem_a, elem_b)` — element *values* — whereas hisab sorts *indices* by dereferencing
  each into a separate `points` vector. A capturing comparator can now close over `points`, so this
  is doable — re-verified on 6.6.2, a capturing comparator works across a fn boundary. ⚠ The
  deferral reason recorded here ("this is the first instance") is **false**; what remains is only
  that 2.6.15 already fixed the *complexity* of the two hot sorts, so this is consolidation for
  consistency, not for speed. **Unpark it: the gate it was waiting on has been met three times over.**

- **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`. ⚠ **Correct the citation: 2.3.1 measured 1.6–6.5x (median ~2.3x), not
  "5–8x"** — and much of that win was accessor-call elimination, which does not exist here.
  ⚠ **The stated gate is not the blocker.** Benchmarks are cheap (72 working labels in the harness).
  The real blocker is that **nobody knows the `n`**: every test calls these solvers at n = 1, 2, 3,
  where a 2-wide dot cannot win, so authoring a benchmark today would reproduce the exact failure
  mode `CLAUDE.md` records twice this arc — measuring the instrument instead of the operation.
  **Needs a consumer-sourced `n` first**, and ten consumers are now live to ask.
  ⚠ Secondary: these buffers are exactly `alloc(n*8)` and `f64v_*` **over-reads on odd `n`**, so each
  kernel needs the pair+scalar-tail hybrid, not a one-line swap.

---

## 3.0.0 -- Error-handling migration (breaking)

The integer-error-code convention (`src/error.cyr`: functions return 0 / a
negative `ERR_*` code) predates the stdlib `Result<T,E>` (`lib/result.cyr`,
v5.8.28) and `?` propagation (v5.8.29). Migrating is a library-wide signature
change — breaking for consumers (impetus, kiran, joshua, …) — so it lands as
a major, with a migration guide, not a 2.x patch.

> ⛔ **RE-SCOPE BEFORE PLANNING: this section was written against a `Result` that no longer exists.**
> It cites the v5.8.28 **boxed** form. `lib/result.cyr` at the current pin is the **v6.6.0 value
> form**: `var r = f();` is a hard compile error, `payload()` is gone, and every call site becomes
> `var t, v = f()`. ⚠ And there is **no incremental path** — `?` on a plain i64 fn **compiles clean
> and SIGSEGVs** (exit 139 where 6 is correct) with no diagnostic.
>
> ⭐ **A free win is available at 2.x instead, and should be taken first**: `#must_use` exists (199
> uses in the tree) and is on **zero** of the nine heaviest fallible modules — `num_ext` 61 `HSB_ERR`
> returns / 0 `#must_use`, `linalg_precision` 48/0, `optimize` 31/0. That is most of the value of
> `Result` for none of the breakage. ⚠ Caveat: it is a *compiler* warning, not a lint warning, so
> CI's `^  warn ` grep will not gate it — wire that before relying on it.

- [ ] Wrap fallible returns in `Result<T,E>` (keep `ERR_*` codes as the `E` payload) — **value form**
- [ ] Adopt `?` to replace manual `-1`-return + check chains
- [ ] **Do first, at 2.x**: `#must_use` on the fallible surface (47 fallible fns, 162 `return HSB_ERR` sites)
- [ ] Migration guide + deprecation window for the old integer-code API
- [ ] **Public / private function surface.** hisab currently signals intent by naming convention
      alone — a leading `_` means "internal" and nothing enforces it. Two consequences already
      visible in the tree: `geo_diff.cyr` reaches `geo.cyr`'s helpers across a module boundary
      because nothing distinguishes "public API" from "implementation detail", and the 2.10.1 split
      had to be named `geo_ray_aabb_face` rather than `_core` specifically so a cross-module call
      would not be reaching for an underscore. A major is the right place to draw that line, because
      marking a function private is a **breaking change for anyone already calling it** — the same
      reason the `Result<T,E>` migration lands here.

  **Sized by measurement, not estimated: 269 `_`-prefixed functions across `src/`, of which 17
  are called from a different module**, in six pairs:

  | defined in | called from | count |
  |---|---|--:|
  | `calc` | `calc_ext` | 3 |
  | `calc` | `noise_simplex` | 2 |
  | `geo_advanced` | `collision_core` | 4 |
  | `lie` | `lie_ext` | 4 |
  | `num` | `num_ext` | 2 |
  | `symbolic` | `symbolic_ext` | 2 |

  So a blanket "`_` means private" would break six real call paths, every one of them a
  `X` → `X_ext` pair where the split is an artefact of file size rather than of API design.
  ~~Decide: the visibility marker; whether those six pairs get a shared-internal escape hatch or
  the helper is promoted to public; and whether the distlib bundle needs the distinction at all.~~

  ⭐ **ALL THREE DECISIONS ARE ALREADY ANSWERED — cycc 6.6.2 ships and ENFORCES visibility.**
  Verified on the pin: a file whose first line is a bare `private`, containing `fn _secret` and
  `pub fn public_api`, gives `error: '_secret' is private to its file` for a cross-file call to
  the former and `ok` for the latter. A per-item `private` is refused with its own diagnostic.
  The stdlib already uses it at scale (322 `pub fn` in `yukti.cyr`; `regex.cyr` is file-private).

  - **The marker**: nothing to decide. It is file-level `private` + per-fn `pub`.
  - **The escape hatch**: ⚠ **still a real decision — an earlier draft of this correction wrote
    "nothing to decide", and that was wrong.** Privacy is per FILE, so marking `X` private breaks
    every cross-module call into it, and the bundle does **not** paper over it (next bullet). Each
    of the six `X` → `X_ext` pairs forces a choice: mark the helper `pub` — promoting an
    implementation detail to public API — or merge the pair into one file, or leave `X` public.
    Three options, six sites. It is no longer a *marker* decision, but it is still a decision.
  - **The bundle**: ⛔ **this file answered it WRONG.** A real `cyrius distlib` bundle built from
    `private`-marked modules exported **only** its `pub fn`s and rejected `_a_helper` from a
    consumer. So concatenation is *not* a free pass — it preserves the distinction.
    ⚠ **Landmine**: `distlib` passes `private` through verbatim into the single 23,405-line
    bundle, and `cyrius check --with-deps dist/hisab.cyr` stays **green** on a bundle **no
    consumer can call**. The existing gate cannot see this failure.

  ⚠ **Re-scope before doing it — this is now a mechanical pass, not a decision, and it is 3x
  bigger than sized above.** Privacy is per-file, so marking one module private breaks **24
  symbols against one suite, 21 of them non-underscore**; the tests reach **52 distinct `_`
  functions across 235 sites**. Safe order: **`pub fn` everywhere first** (verified a no-op and
  non-breaking — can land in a 2.x), add a consumer-call gate, then flip `private` last.
  **The `pub fn` half does not need 3.0.0 and should not wait for it.**

---

## Parked / deferred (revisit when a driver appears)

Evaluated during earlier arcs and consciously deferred — recorded so they
aren't silently lost (full rationale in the CHANGELOG). Items that have since **shipped** are
removed from this list rather than struck through; their record lives in the CHANGELOG and the
Release History table below.
- **SIMD `cross`** (from 2.3.1) — needs lane shuffles; `f64v_shuffle`/`permute`/`blend`/`swap` are all
  undefined on 6.6.2 (probed), so still correctly parked. ⭐ Now with a number instead of an assertion:
  the best shuffle-free formulation measures **38 ns vs 25 ns scalar (+52%)**.
  ⚠ **`lerp` was never gated on shuffles at all and should be unparked** — measured on 6.6.2 with the
  existing n=2-pair + scalar-tail hybrid: **25 ns → 19-20 ns, bit-identical results, zero shuffles**.
  ⚠ The "full rationale in the CHANGELOG" this list cites **does not exist**: 0 hits for shuffle/lane.
- **`#pure` annotations** (from 2.3.4) — ~~unsafe CSE interaction with hisab's allocate-a-fresh-result
  convention~~. ⚠ **Premise refuted on 6.6.2: there is no CSE to be unsafe.** Three identical `#pure`
  calls emit `calls: 3`; two `#pure` allocating calls return distinct pointers; and the binaries are
  **byte-identical** (same sha256) with and without the annotation. `#pure`'s entire effect in cycc is
  two warnings. ⛔ **Stronger objection than the one recorded**: `alloc()` carries no `#alloc`, so
  annotating hisab's 267 allocation sites `#pure` would assert a falsehood with no compiler backstop.
  Stays parked — for the right reason now.
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4) — correctly parked, and now measured rather than
  predicted: checked slice indexing is **3.8x** raw `load64`, and the "unchecked escape hatch" is
  still **3.2x** — it discards the safety *and* keeps 85% of the cost. the toolchain's `~/.cyrius/versions/<pin>/lib/simd.cyr` has **zero**
  slice-taking forms, so slices provably cannot cover the SIMD hot paths at all.
- ~~**`defer`** (from 2.3.4) — N/A under the bump/arena model.~~ ✅ **Decided in 2.3.4 and re-derived
  2026-09-09: 0 `_free`/`_destroy`/`_close`-shaped fns in `src/`, 0 `sys_open`/`sys_close`, 339
  `alloc(` with zero frees.** `defer` works fine in the stdlib (8 uses — 6 closing a file descriptor, 2 unlinking a temp file); hisab
  has no call site to attach one to. **Not a deferral — an answered question. Delete this row.**

*Retired 2026-08-03:* **stdlib `mat_new` overflow guard** (parked from 2.5.3) — shipped in
**2.6.11** via ganita 1.0.4 at the 6.5.6 pin. `mat_new_guarded` is retained as the stricter 16M
entry point, so nothing further is owed.

---

## Consumers

⛔ **~~None are live yet — all four come online later.~~ THIS IS WRONG IN BOTH DIRECTIONS, and it is
the most consequential stale claim in this file.**

**Ten repos consume `dist/hisab.cyr` today, SHA-locked**: svara, naad, goonj, dhvani, attn11, ghurni,
prani, garjan, prakash, nidhi. svara's `cyrius.lock` pins hisab commit `1bc71e3` (tag **2.11.2**) and
`svara/src/spectral.cyr:246` calls `num_fft`. Meanwhile **impetus, kiran, joshua, hisab-mimamsa and
kana have no `cyrius.cyml` on any branch** — they are Rust repos needing a *port*, not a scheduling
decision. ~~`README.md:7` is wrong the opposite way, listing the Rust repos as users.~~ ✅ Corrected 2026-09-09.

⚠ **No live consumer has built 2.11.3 or later.** All ten sit at 2.11.1/2.11.2, behind the
6.5.33 → 6.6.2 toolchain bump **and** the 536-site ganita alias migration.
`cyrius check --with-deps dist/hisab.cyr` proves the bundle compiles against **this** manifest's
ganita — not against theirs. **Highest-value action in this section: get one live consumer onto
2.11.5.**

**The struct-layout contract** — construct via the documented constructor, read via the accessors,
size arrays with `sizeof(T)`; never a hardcoded byte count, never a hand-computed offset.
⛔ **But "32 assertions … make any such change trip a gate" is FALSE.** All 32 are `sizeof(T) > 0` or
`sizeof(T) % 8 == 0`. `ColContact` going 64 → 72 bytes passes both, **identically, on both sides of
the change they were written to catch**. There is not one `assert_eq(sizeof(T), <n>)` anywhere in the
tree, and six public structs carry no assertion at all — including **`HVec3`, the type live consumers
touch most** (54 `hvec3_new` sites). The commit that declared the contract "now enforced" enforced
nothing. **A gate that cannot fail is not a gate**, and this one has been quoted as protection for
five releases.

| Consumer | Domain | Surface it will use |
|----------|--------|---------------------|
| **impetus** | physics | GJK/EPA, MPR, PGS, sequential-impulse, inertia, spatial |
| **kiran** | engine | projections, BVH, k-d tree, frustum |
| **joshua** | simulation | DOPRI45, BDF, symplectic, optimize |
| **aethersafha** | compositor | projections, compositing, color |
| **abaco** | expression eval | symbolic integrate/LaTeX/patterns, interval |
| **svara** | vocal synthesis | complex, FFT, easing |
| **hisab-mimamsa** | physics | tensors, Lie groups, diffgeo, CGA |
| **kana** | quantum | tensors, Lie groups, complex LA, spinors |

**Known caveat, and first integration has already happened (see above):** `gjk_intersect_3d` costs
~+55% on the no-hit path since 2.9.0 — the price of it no longer missing 134 genuine interior
overlaps per 4,386 evaluations. The no-hit path is the broadphase-common case, so if a consumer finds
that cost unacceptable the answer is a cheaper pre-filter, not reverting the correctness fix.
⚠ **Re-derived on 6.6.2 post-ganita, same-binary ABBA: box +62%, sphere +57% — this file understates
it.** ⛔ But "a cheaper pre-filter" cannot be built where this row implies: the entry point receives
only two support-function pointers, so there is **no cheaper information inside it to filter with**,
and hisab already ships the broadphase (`src/spatial.cyr`). **This is caller-side work, not a hisab
roadmap item** — and with ten live consumers, someone can now actually be asked whether it bites.

---

## Release History

| Version | Date | Lines | Files | Highlights |
|---------|------|-------|-------|-----------|
| 2.13.0 | 2026-09-09 | 23,528 | 36 | **The suite release: the suite could not see an error of 0.9.** The audit's self-declared highest-value item, justified by measurement rather than prose — on the 2.12.0 tree `hvec2_add` could return `a + b + 0.9`, a core public function wrong by nearly a whole unit, and **all 3572 assertions still passed, zero failures across five suites**. `assert_eq(f64_to(v), 3)` accepts every v in [3,4) because `f64_to` TRUNCATES; **842 of 3105 sites (27.1%)** compared floats that way and now **16 (0.5%)** do, every survivor a deliberate rounding or scaling test. `foundation.tcyr` went **89% -> 0%**. ⭐ **The migration verified itself** — a bit-exact compare FAILS at any site whose value was not already exact, so a mechanical rewrite became a search: 809 converted and **all but four passed at once**, meaning the truncation had been pure slack. ⛔ **Those four were four different defects, each passing only because of truncation**: `srgb_to_linear(-1)` asserted as 0, actually **-1/12.92**; `sh_evaluate_l2` on a zero direction asserted as 0, actually **-0.0333**, and its message was wrong too — a zero direction leaves **two** bands nonzero (Y00 and Y20), not one; the PGS solver checked against 0.090/0.636 where the line above it says **1/11 and 7/11**; and an antiparallel cross product returning **negative zero**, which `f64_to` mapped to the same integer as +0. A fifth truncation was deliberate and is now pinned on both sides. Named tolerance helpers added to all five suites — `abuse.tcyr` had no named constant at all, spelling its bound as a raw hex literal at the call site. **Measured strengthening**: `m4_get + 1e-6` 29 -> **84** failing, `hvec3_cross.x + 1e-6` 76 -> **88**. ⚠ Two of those numbers were nearly reported wrong — the first harness's mutation silently failed to apply and returned `0 failing`, which reads exactly like "the suite is blind"; it now refuses to report unless it can grep a marker proving the mutant is installed. **Prove the mutant is installed before believing the count.** 3574 |
| 2.11.5 | 2026-09-09 | 23,319 | 36 | **cycc 6.6.2: the wrong-code bug is fixed, and the filing was wrong about its scope.** Toolchain 6.6.1 -> 6.6.2, no hisab behaviour change, suite 3538/3538, lock 30 -> 31 (new `lib/boxed.cyr` via `tagged`). hisab's filed reproducer now exits **0** on 6.6.2 and **139** on 6.6.1, and the three suites that used to SIGSEGV pass with `m4_mul_vec4`'s hoist REMOVED — so the workaround is retained for consistency with `m3_mul_vec3` only, and its comment no longer claims to be load-bearing. ⛔ **The filing was wrong twice**: it was filed as derive-specific and as a 6.5.71 regression and was NEITHER — all **21** SIMD handlers took `var vbase = GFLC(S)` without raising GFLC until every argument was parsed, so any argument allocating a frame local bound it over the destination; reachable via `callptr` since 6.0.70 and `#inline` since 6.5.63. **A first-bad-version is evidence about VISIBILITY, not ORIGIN** — and the filing's own root-cause section said so while its header contradicted it. Severity was understated too: with `CYRIUS_REGALLOC_PICKER_CAP=0` it exits 0 and writes into the ARGUMENT OBJECT. ⚠ **A second fix retires a landmine this repo documented since 2.3.1 with the WRONG CAUSE attached** — a bare SIMD intrinsic at top level compiled clean and SIGSEGV'd on every release, not from SSE stack misalignment as `vec4.cyr` claimed but because the handlers stash operands in FRAME slots and top-level code has no frame; 6.6.2 refuses at compile time, verified 6.6.1 OK-then-139 against 6.6.2 compile-error. ⚠ 6.6.2 also shows the one class a consumer build cannot catch — 6.6.0 **silently redefined `tag()`/`is_tag()` at unchanged arity** — checked rather than assumed: hisab calls neither. **A name whose meaning changed must be RETIRED, not redefined.** **No performance change claimed**: two rows read as regressions (+22.85%, +14.01%) and were single-run outliers — the FIRST 6.6.2 run matched 6.6.1 within 1% — settled by comparing each benchmark against ITS OWN spread rather than a global band that one 50x-noisier row had widened. 3538 |
| 2.11.4 | 2026-09-09 | 23,275 | 36 | **Off the deprecated ganita aliases, and the 28% that was hiding behind them.** ganita 1.2.4 marks the bare `mat_*`/`f64_*` spellings deprecated (migration window only); all hisab call sites now use the `ganita_*` names, `dist/hisab.cyr` included. ⚠ **2.11.3 estimated this at 8 call sites. It is 536** — the estimate was scoped from the names ganita's changelog paragraph happened to mention rather than from its surface, where the deprecation block is a **53-entry table**; hisab was on 20 of them, `mat_set` alone 265. **Scope a migration from the dependency's surface, never from its release notes.** The 5 suites produce **byte-identical output** across the change — captured and diffed, not re-counted — which is the right oracle because the aliases are literal one-line forwarders. ⭐ **They were not free**: cycc's `#inline` requires <= 2 parameters and `ganita_mat_get`/`_set` take 3 and 4, so all 68 matrix accesses in `linalg_precision.cyr`'s SVD and eigen inner loops paid a `call`/`ret` for nothing — `svd_golub_kahan_12` **-27.85%**, `eigen_qr_12` **-27.67%**, against a same-binary noise floor measured in the same session (median **2.10%**, worst **9.13%**), i.e. ~3x the worst noise and landing on exactly the two benchmarks the mechanism predicts; the other 21 trustworthy rows drifted +0.07..+6.99% inside that band with zero matrix calls between them, so **no other movement is claimed**. ⚠ **Two assertions had gone VACUOUS**: 2.11.3 fixed the two tests that FAILED when upstream repaired what they had pinned, but these compare through a tolerance and a round, so they kept passing while the property evaporated — the stdlib `(-2)^3` truncates to -8 now, so a pair labelled "truncation-discriminating" discriminates nothing. **An assertion written against a defect cannot fail when the defect is repaired; it just stops testing anything.** ⚠ And **`_ad_pow`'s reason to exist collapsed a second time** — domain died in 2.11.2, precision here (665/665 comparisons now bit-identical) — leaving a third ground that is the OPPOSITE of the second: squaring amplifies relative error, so `(-0.999)^1000` is **4 ulp** from truth through repeated multiplication and **58** through binary exponentiation, inverting past ganita's +-1024 window. Filed rather than decided, recording that no test exercises the regime it still wins. 21 present-tense API comments corrected; ~45 historical mentions deliberately left, with the convention stated once in `src/f64_util.cyr`. 3532 |
| 2.11.3 | 2026-09-09 | 23,245 | 36 | **The toolchain catch-up that hit a wrong-code regression.** cyrius **6.5.33 -> 6.6.1** (forty-odd releases, crossing a minor), sakshi 2.4.11 -> **2.5.1**, ganita 1.1.4 -> **1.2.4**. ⛔ **The new toolchain miscompiles hisab, and the bug is upstream's, not this repo's**: three of five suites SIGSEGV'd on the first run. A `#derive(accessors)` getter passed directly as an `f64v_*` intrinsic argument makes the intrinsic read its **destination pointer from a stack slot nothing ever writes** — `movupd %xmm0,(%rdx,%rsi,8)` with `%rdx = 12`, loaded from `-0x50(%rbp)`, a slot read once and written zero times, while the src and scalar slots ARE written. **Bisected: 6.5.70 clean, 6.5.71 broken** — the release that put derive accessors on the inline-replay path (`callq` 7 -> 3), removing the call that had incidentally forced the spill, so **the intrinsic bug is older than 6.5.71 and that release only stopped hiding it**. Three functions of identical shape in ONE file discriminate it: a plain fn accessor is fine, a raw `load64` is fine, only the derived getter fails — so it is not 'inlining' in general. Fixed by hoisting the accessors into locals, **which `m3_mul_vec3` has always done**; mat4 was the lone inconsistency. Mutation-proven both ways, restore confirmed by grep. ⚠ **Wrong-code, not merely a crash** — it faults only because the garbage slot happens to be unmapped. Filed upstream in the cyrius repo. **(2) Two tests updated because upstream FIXED the defects they had pinned**: ganita 1.2.4 replaced `exp(n*ln\|base\|)` with binary exponentiation for integral exponents (upstream saw `pow(7,2) = 48.99999999999999296`), so `f64_pow(-2,4)` went from **15** to a bit-exact **16** and `_ad_pow`'s precision rationale is void — the two now agree bit for bit; and 6.6.1 gave `math.cyr`'s exp polyfill its missing infinity guard, so `cx_exp(-inf)` went from NaN to exactly **+0**, which is correct. ⚠ **The replacement had to be made discriminating**, because 0 is also what the fabricated-guard class produces — `exp(+inf)` is now asserted beside it. **(3) The manifest's ceiling comment named a limit that does not gate this path**: expanded source **8 MB -> 24 MB**, tokens **1,048,576 -> 4,194,304**, and the retired '16 MB input_buf' described `_SRC_CAP` instead. Measured as a PAIR — a 9,002,640 B source is rejected by 6.5.33 and clean on 6.6.1 — because a one-sided 'it compiled' proves only that the cap exceeds one file. ⭐ **And the token cap bit first.** **Performance**: geometry/collision up sharply from the same accessor inlining — bvh_query_ray_200x4k **-46.5%**, bvh_degenerate_4k -30.4%, gjk_epa_sphere_box -29.6%, grad_fwd_16 -24.4% — claimed ONLY from the 23 benchmarks whose net is >= 10x the timer floor, with the flat numeric kernels (svd +0.1%, eigen_qr +0.4%) as the control proving the instrument did not shift. ⚠ **41 of 72 benchmarks report a net BELOW the floor they subtract** (worst 0.01x), so this release's `ray_aabb` -55.9% is evidence of nothing; filed, not fixed. ⚠ **The enum Critical the launcher warns about was checked, not assumed**: 0 of 936 enum constants tree-wide reach 2^62. 3532 |
| 2.11.2 | 2026-08-21 | 23,225 | 36 | **The toolchain catch-up, and the three stale constants it found.** cyrius **6.5.18 → 6.5.33** (fifteen releases), sakshi 2.4.10 → 2.4.11, ganita 1.0.4 → 1.1.4. No feature work. **Every finding is a number written down once and then trusted** — the same shape as 2.11.1's ALLOC_MAX, three times over, and **none was found by a failing test**: all 3514 assertions passed on BOTH sides of the bump. ⚠ **The tree arrived mid-sync and the missing half was the half that mattered**: `lib/` had been vendored from ≈6.5.19, so alloc/assert/atomic/bench were current while **ganita (1.0.4, −202 lines)**, fmt and three syscalls variants were behind — old-pin-vs-new-pin shows a tidy diff and misses it; only comparing against **the pin's own snapshot** finds it, the third time ganita specifically has been caught this way. A brace-aware, comment-stripping extractor puts the whole stdlib delta at **four functions** (a first pass keyed on `fn` alone said seven — six were the trailing comment block attributed to the preceding function; **the instrument was wrong before the measurement was**, three times today). **(1) `f64_pow`'s domain moved under us**: ganita 1.1.4 fixed negative-base-integral-exponent upstream, and `symbolic.cyr`'s `expr_eval` sits directly on it — `(-2)^3` returned **NaN for hisab's entire history** and returns a number now; `f64_pow(0,0)` was NaN, is 1. 12 assertions pin the new domain, and **discrimination was measured, not asserted**: re-run against a checkout still on 1.0.4, **10 of the 12 fail** and the 2 that pass are the 2 controls. `_ad_pow` survives on **precision** rather than domain — the upstream fix takes its magnitude from exp(n·ln\|base\|), so `(-2)^4` reads **15** through the stdlib and **16** through hisab, because `f64_to` truncates. **(2) The benchmark instrument changed**: 6.5.19 measures the clock floor and subtracts it, and `bench_run` sizes its own batches — **44 of 72 rows moved >10% and not one is a speedup** (`ease_in_out` 1,407 ns → **7 ns**, 100% instrument). ⚠ And the old numbers **FLATTENED** them: four operations spanning **149x** in reality were reported within **1.26x**, so a regression had room to hide — 2.10.0 moved 17 benchmarks off that floor and **these four were still on it**. `bench-history.csv` gains **`regime`/`floor_ns`** and the trend filter enforces them: `stat` alone would have admitted all 44. **Third instance of a measurement-method change moving every row** (2.9.2, 2.10.0, now), and the first two were answered with prose the trend table could not read — so per CLAUDE.md's third-instance rule it is now a column, and it is **derived** from whether the harness printed its own floor, not declared. **(3) The bundle ceiling does not exist**: 6.5.22 raised `_SRC_CAP` **1 MB → 16 MB** (it had been refusing sigil, mabda and drishti outright), so the bundle is **5.4%** of input_buf, not 77% — verified by compiling a 1,162,472 B source, 111% of the retired cap. ⭐ **And a CI annotation was handing developers a command that destroyed the file**: `cyrius fmt $f > tmp && mv tmp $f` was correct until 6.5.28 made fmt rewrite in place and print nothing — reproduced, src/vec2.cyr **2,233 B → 0 B, rc=0** — and it only ever printed for an already-drifting file, of which the same 6.5.28 fmt fix produced **38 at once**. Also: 38 files reformatted (proven leading-indentation-only, no line-count change), and three `syscall` write lengths in `examples/basic_math.cyr` over-read by one byte each, **printing a NUL into the shipped example's output**. 3526 |
| 2.11.1 | 2026-08-11 | 23,182 | 36 | **The audit release: one rule, and the four sites that broke it.** A full P(-1) sweep of the 2.11.0 tree — 6 dimensions, 115 checks, **52 findings reproduced**, 21 CONFIRMED by an independent skeptic, 2 REFUTED, 1 already known, and **28 never verified because the harness capped the verify phase at 24** (recorded as the audit's own biggest process defect, not buried: 2 of the 24 that WERE checked came back refuted, so an unverified finding is ~1-in-12 wrong and none may be acted on until re-run). **24 of the 52 are the same defect** — a guard comparing a quantity against a threshold that is wrong for it, then FABRICATING a plausible answer: `eigen_qr` wrong eigenvalues with rc = 0, `cqr_decompose` an R that is not upper triangular with rc = 0, `hvec3_angle` 0 rad ('parallel') for exactly perpendicular vectors, `m3/m4_inverse` the identity. This is the ninth through thirty-second instance of a class first written down in `complex.cyr:58` in 2.6.14 — and `cx_div` is IN THAT COMMENT and still fabricating zero. **Writing a lesson beside the code that taught it does not fix the code and does not reach the other thirty-four modules; only a grep does.** Four repairs executed: `hquat_inverse`/`hquat_normalize` compared a SQUARED magnitude against the unsquared EPSILON_F64, returning the identity for any \|q\| < 1e-6 where the inverse is finite and exact; three caller-sized allocations in `num_ext` stored through `alloc`'s 0 return (**reproduced as SIGSEGV**, now HSB_ERR_ALLOC); `ad_tape_new` handed back a non-zero handle with a null body — 2.11.0's own code; and `optimize.cyr`'s ALLOC_MAX-derived ceiling was re-derived 5792 → 16384. ⚠ **A stale comment nearly buried a real defect**: the first `num_tridiag_solve` probe was sized from `optimize.cyr`'s own `ALLOC_MAX = 256 MiB`, landed EXACTLY on the limit, allocated successfully, and the finding looked refuted — cyrius 6.4.51 raised it to 2 GiB. **A constant derived from a dependency is a measurement.** ⚠ The quaternion sweep's first 12 decades killed only 1 of 3 mutants (both rejected thresholds sit BELOW where it looked; 20 decades kills all three), and asserting unit length would not have discriminated at all — the fabricated identity IS unit, so the assertion checks DIRECTION. The audit's third critical is the suite itself: **836 of the 3510 assertions its own scan counted (23.8%) compare through `f64_to`, which TRUNCATES**, and 38.4% of value-changing mutants survive all five suites — a measured reason why 3507 assertions and 99% coverage saw none of the 52. 3514 |
| 2.11.0 | 2026-08-11 | 23,104 | 36 | **Reverse-mode autodiff, and the five forward-mode defects it found first.** Tape-based: one node per op recording both local partials and both input indices, and because inputs always have strictly smaller indices a single descending loop is a valid reverse order — no sort, no visited set. **grad_fwd_16 63.7 us -> grad_rev_16 5.70 us, 11.2x** for a 16-input gradient (not the theoretical 16x: reverse pays to RECORD the tape, and part of the rest is dual_* heap-allocating under an allocator that never frees). The pairing with optimize.cyr needed NO API change — a capturing closure holding the tape matches fncall2(grad, x, out), re-verified on 6.5.18 because that shape SIGSEGVed on 6.5.16. **Reverse mode is validated AGAINST forward mode, so autodiff.cyr was swept before a line of tape code existed**, and five defects came out: 1/1e-13 returned (0,0) where the truth is 1e13; ln(-5) returned a NaN value beside a CONFIDENT -0.2 derivative; sqrt(-4) was (NaN,NaN) because the guard ran after the sqrt; d/dx x^3 at -2 was NaN because f64_pow is exp(n*ln(base)) and rejects negative bases — the stdlib's STATED implementation, so the defect was hisab inheriting it undocumented. ⚠ **The finite-difference sweep alone found NONE of them**: at every input where a guard fires the perturbed scalar is NaN too, so the sample is skipped and the guard never asked. A guard has to be interrogated DIRECTLY. 14 mutants, 14 kills — two survived the first pass (the fixture's variables happened to BE nodes 0 and 1, so the ids indirection was untested) and one failure was the ASSERTION's fault, demanding 1e-8 where gradient descent only promises \|\|g\|\| < 1e-6. The solvers' own contract check came back CLEAN, the first time in five releases. 3507 |
| 2.10.2 | 2026-08-11 | 22,707 | 36 | **The primal defects the jets were sitting on.** All three 2.10.1 filed, plus the OBB rotation partial it deferred. **One rule settled four thresholds** — guard exactly what makes the DIVISION fail and nothing more (`_GEO_F64_TINY` = DBL_MIN); a threshold that needs a scale chosen for it IS the defect. The slab parallel test was scale-free: a box spanning [0,1] with a **unit** direction (9e-13, 0, 1) returned a hit at **x = 1.4**, and the same ray from inside a tall box gave an exit **18x too large**; `geo_ray_new` normalizes and did not protect. `geo_ray_capsule` returned a point a **full radius inside the solid** (t = 4 where the exit is 6, hit point ON THE AXIS) because `geo_ray_sphere` hands back only the FIRST root, so a cap root rejected by the half-space test meant the other was never considered — 32 of 600 interior origins also lost their exit entirely. Fixed by splitting the quadratic onto `_geo_sphere_roots` (both roots, one copy) and deleting the `cyl_missed` fallback outright. `geo_triangle_unit_normal` returned a **fabricated** (0,1,0) for a well-formed right triangle with 1e-4 legs. `dt/d(rotation) = [a_k x (c - p)]/f_k`, derived by perturbing WITHIN the rotation group rather than freely, verified over 36 FD comparisons at worst 1.9e-10 — a world-frame ANGULAR gradient, so a step along it is a valid rotation by construction. ⚠ Two repairs cost far more before being measured (an is_inf HELPER +5.6% on a 90 ns routine; materialising a hit point per cap root +92%), two fixtures were caught by COUNTERS rather than by failing, and a toolchain defect was nearly filed that does not exist — `var buf[N]` needs `&buf`. 13 mutants, 13 killed. 3469 |
| 2.10.1 | 2026-08-10 | 22,529 | 36 | **The branchy primitives, and two more defects the design check found first.** Jets for aabb, obb and capsule — all six `geo_ray_*` are now differentiable. Each primal split onto a face/branch-reporting variant with the plain entry point a one-line wrapper, so the value path is unchanged BY CONSTRUCTION: the `f64_max`/`f64_min` calls are byte-for-byte what they were and every added line sits behind `if (out != 0)`. aabb 3.1x its primal for 12 partials, obb 1.7x for 12, capsule 1.38x for 13 — the branchy three are CHEAPER relative to their primals than the smooth three, because the primal work they reuse is larger. **Before any feature code**, asking "what does the primal return when there is no face?" found `geo_ray_aabb` and `geo_ray_obb` returning **+Inf** for a degenerate direction (4 of 6 siblings honoured the contract; the same 4-vs-2 shape as 2.10.0) and `geo_ray_sphere`/`_capsule` comparing a **squared** length against the unsquared `EPSILON_F64` — the sphere reporting a MISS for any \|d\| < 1e-6 where t = 4e7 is exact, the capsule silently returning a CAP hit instead of the cylinder hit, wrong by 0.942%. ⚠ **The sphere's guard was introduced by 2.10.0's own repair**, and 2.10.0's homogeneity sweep could not have caught it: its scales are 2, 0.5 and 7, none within five orders of magnitude of the threshold the same commit added. A THIRD came from an independent derivation commissioned against the finished code — the tie flag saw edges and corners but not a zero-width slab or a tangential clip, so a flat box returned the whole gradient in the WRONG SLOT with tie = 0 on half of all configurations; the derivation confirmed all 37 partials (0 of 24,237 FD comparisons failing) and its value was entirely in the ENUMERATION. Three defects in the PRIMAL are filed OPEN for 2.10.2, including a scale-free slab test that returns a hit at x = 1.4 for a box spanning [0,1] with a UNIT direction. The capsule seam is proven C1 by a CONVERGENCE-RATE assertion after a single-offset one failed on a fixture fault. 25 mutants written, 24 killed — the survivor proved a branch update was dead code (cyl_t1 <= cyl_t2 always; 932 randomised quadratics, 0 violations). jet_obb 1.083 us -> 872 ns (-19.5%). Toolchain 6.5.17 -> 6.5.18, zero stdlib delta. 3453 |
| 2.10.0 | 2026-08-10 | 21,855 | 36 | **Differentiable geometry — and the primal defect it found first.** `src/geo_diff.cyr`: jets for plane, sphere and triangle returning the full gradient from ONE evaluation, allocation-free, as a post-pass on the shipped primal so no intersection algorithm is duplicated. Sphere 7 partials at 335 ns vs its 93 ns primal (3.6x); triangle 15 partials at 934 ns vs 280 ns (3.3x) — against the ~4,200 ns forward-mode duals would need for the same 15. **Before any autodiff was written, the design's own homogeneity check was run against the EXISTING primitives and failed**: `geo_ray_sphere` was not degree -1 in the ray direction, returning a t whose hit point sat 3.74 from the centre of a unit sphere, with `geo_ray_capsule` inheriting it through its end caps. Silent wrong output in shipped geometry, found by a check for a feature that did not exist yet. The same pass found 17 of 60 benchmarks measuring `clock_gettime` rather than the operation (ray_sphere read 1,466 ns, is 79 ns) — the fourth non-gating gate of the arc, and the only reason the repair's +17.6% was visible. Nine mutants caught across the two jets. Toolchain 6.5.16 -> 6.5.17, which fixed all three defects hisab filed upstream. 3398 |
| 2.9.3 | 2026-08-10 | 21,498 | 35 | **The open filings, and the two whose own proposed fixes were wrong.** Five of six internal filings closed. Two real defects: `delaunay_2d` silently DROPPED input points on any set mixing scales (one vertex at ~6e5 with five inside ~5e-17 of the origin returned 4 triangles where the exact hull says 6, and used 5 of 6 points) — fixed with an adaptive exact `orient2d` on RAW coordinates, 300/300 against exact rationals where the float winding scores 0/300; and `_col_dl_incircle` answered differently depending on VERTEX ORDER, `g1` being the last helper reading the CCW storage convention instead of forming the winding. Three guards were policing nothing: the k-d balance guard had no regression assertion (deleting it broke nothing checked) and no benchmark on its own input class (now 564.8 us guarded vs 3.397 ms unguarded, 6.0x); the ear prune's only benchmark was its BEST case (reflex-heavy is 3.6x, and the small-n regression is +15% measured against the pre-prune body); and `_col_point_in_tri` documented itself as strict-interior while being boundary-inclusive. **TWO FILINGS ARGUED FOR REPAIRS THAT MEASUREMENT REFUTED** — an exact determinant of pre-differenced operands fixes nothing (the information is gone before it runs), and dropping EPA's seed test produces wrong depths at exact tangency (`_ag_baddepth` 0 -> 4). Both were implemented in full before being rejected. The EPA filing's central claim was also false: it measured one of two entry points. 3376 |
| 2.9.2 | 2026-08-09 | 21,262 | 35 | **The toolchain bump, and three gates that were not gating.** Toolchain 6.5.9 → **6.5.16** (seven releases) + sakshi 2.4.8 → **2.4.10**. No library source change — the bundle diff is the version header alone; all 30 vendored files byte-match the 6.5.16 snapshot, `deps --verify` 30/30. `scripts/bench-history.sh` recorded the **max** of each benchmark, not the average, in **44 of 55** rows and in every row it has ever written — the parser scavenged the last `<num><unit>` from a line that ends in `max=`. Re-anchored on the harness's named fields, CSV gains `stat`/`avg_ns`/`min_ns`/`max_ns`/`iters`, and `benchmarks.md` compares only same-`stat` rows at ±10% — the measured noise floor is a 3.5% median avg spread over three back-to-back runs of one binary, against −93%..+742% swings on the max rows. **No performance claim in this release**: it is the first correct baseline. CI's version gate was an unanchored `grep` — `2.9.2` matched `32,942,104 B` — and `src/main.cyr`'s hardcoded CLI string was checked by nothing; both now assert against `VERSION`. cyrius 6.5.14's `distlib` self-check rejects any bundle reading a stdlib constant (hisab's reads `F64_ONE`), so `ci.yml` and `release.yml` were both RED; now tolerated by exact signature plus `cyrius check --with-deps`. `dist/hisab.deps` tracked, 15 stdlib leaves. 3351 |
| 2.9.1 | 2026-08-06 | 21,262 | 35 | **The deferred tier.** Four latent ghost in-circle defects, all found by deriving an exact-rational oracle rather than by a failing test. `_col_dl_ic_g1`'s M^1 tie-break was the only one wrong on REACHABLE CCW input (228 of 463,086) — it vanished whenever the edge was parallel to `u_k` and could not separate "between a and b" from "beyond b"; replaced by a betweenness test. `_col_dl_ic_g2`'s tie branch applied winding twice; `_col_dl_ic_g3` reduced to the constant 1 with two independent proofs. `mpr_intersect`/`mpr_penetration` re-measured BEFORE being touched, which changed the answer — they were still running the pre-2.9.0 containment test. A FOURTH non-reproducing measurement (the "36%" claim, actually ~28% and distribution-dependent) corrected across four files. Toolchain 6.5.8 → 6.5.9. 3351 |
| 2.9.0 | 2026-08-05 | 21,094 | 35 | **A narrowphase a physics engine can build on.** All five exit criteria met. Scoped as the narrowphase repair, which had already shipped in 2.8.3 — the real content was the tail that mechanisms exposed: `tests/abuse.tcyr` found **11 defects on public entry points** (`cqr_decompose` overran `out_R` while returning `HSB_ERR_NONE`); a disposition column caught `sequential_impulse` marked FIXED when the friction impulse was identically zero at every mu; a 54-mutation sweep on the integrated tree found the delaunay ghost-direction invariant had **zero** assertions. `gjk_intersect_3d` stopped missing 134 genuine interior overlaps (+55% no-hit cost, stated). Struct-layout contract set. The audit's entire low tier, 5 of 5, had never been scheduled. 1818 → 3294 |
| 2.8.4 | 2026-08-05 | 20,778 | 35 | DCT/DST dispatch heuristic. The 23x `num_dst_1024` gap was **not a defect**: 1.6x is DST-I's irreducible 2(n+1) DFT and 14.6x is that n = 1024 makes that extension non-power-of-two. Sibling sizes added to the bench so the parity cliff is visible. The real defect was `_numx_use_fft` billing DST-I for post-processing transcendentals it never evaluates — four sizes dispatched to the slower path. |
| 2.8.3 | 2026-08-05 | 20,437 | 35 | **The eight-track audit-repair merge.** Exact MTV (`mpr_penetration` 66/455 with 216 wrong signs → 862/862 against a reference in exact rationals, worst rel. error 1.6e-9); `time_of_impact` fixed-step sampler → real conservative advancement; `gjk_epa_3d` out-param contract on every return path. Two tracks solved the same problems incompatibly and only one of each could land. Toolchain 6.5.6 → 6.5.8. 1818 → 2263 |
| 2.6.15 | 2026-08-03 | 17,100 | 34 | **P3 + P4 closeout — the 2026-08-03 audit is fully discharged.** Both O(n^2) hot paths rewritten, benchmarked before and after (rows in `bench-history.csv`): `halfedge_from_triangles` twin pairing 190.1 ms -> **1.2 ms** (-99.4%) via an open-addressed hash of directed edges, and `convex_hull_2d`'s insertion-sort pre-pass 22.1 ms -> **2.1 ms** (-90.3%) via heapsort (chosen over merge sort because its scratch would leak per call under the bump allocator; the comparator is inlined because Cyrius has no closures). Both verified output-identical to the old code. Levenberg-Marquardt's 4 loop-invariant buffers hoisted and its symmetric J^T J computed once; L-BFGS's 3 per-iteration buffers hoisted. Six documentation drifts corrected (BVH mis-attributed to `spatial.cyr`, Fletcher-Reeves vs Polak-Ribiere+, DST-II vs DST-I, `expr_eval` "aborts", `hodge_star_2form_4d`'s three contradictory `sign` docs, `collision_core`'s header naming a deleted file). **`num_ext` + `symbolic_ext` brought in — all 34 modules are now covered by a suite**; coverage 57% -> **59%**, files 34/35. 1127 |
| 2.6.14 | 2026-08-03 | 17,000 | 34 | **P1 closeout — the memory-safety tier.** Three of the four defect groups **crashed the process** pre-fix: reverting `complex.cyr`, `calc_ext.cyr` or `optimize.cyr` individually makes the suite exit 139. Capped constructors (`cmat_mul`, `cmat_kronecker`, `cmat_identity`, `cmat_inverse`) stored through `cmat_new`'s documented 0-on-failure return — reachable from operands *under* the cap, since `cmat_mul` of 1000×1 by 1×1000 asks for 1e6 and `cmat_inverse` doubles the width. `opt_bfgs`/`opt_lbfgs` sized `n×n` / `m×(2n+1)` buffers from an unbounded dimension (added `_OPT_MAX_DIM`, alloc checks, `HSB_ERR_ALLOC`). `calc_bspline`/`calc_nurbs` indexed control points **negatively** when `n_pts <= degree`. Adaptive Simpson bounded depth but not work (2^50 evaluations on a NaN integrand). `kdtree_build` recursed O(n) deep on coincident points — fine at 40k, **SIGSEGV at 60k**. `_opt_armijo` accepted NaN. `cx_div`/`cx_inv`/`cx_powf` used a 1e-6 cutoff instead of 1e-12 (squared vs unsquared tolerance). `FLOAT_RENDER_BUF` was 32 against a scratch reach of 45. Four fBm entry points returned NaN for `octaves <= 0`. **+`calc_ext`, `noise_simplex` into the suites**; coverage 54% → **57%**. 1093 |
| 2.6.13 | 2026-08-03 | 16,900 | 34 | **P0 closeout** of the 2026-08-03 audit — every remaining critical/high correctness defect, all in modules with **zero test coverage**. `svd_golub_kahan` composed the left Householder reflectors forward, returning U **transposed** so `A != U·S·Vᵀ` (8 of 9 entries wrong); now accumulated backward. `eigen_qr` applied `Gᵀ T G` instead of `G T Gᵀ` — the transpose of the rotation that zeroes the bulge — so it **never converged for n ≥ 3** (NO_CONVERGENCE on a symmetric 3×3 at 100k iters) and paired wrong eigenvectors at n = 2. `su2_exp`/`su2_log` moved to half-angle, restoring `su2_to_rotation_matrix(su2_exp(ω)) == so3_exp(ω)` and fixing `se3_exp`, which had built R and t from angles a factor of 2 apart. `einsum` accepted only labels `a`–`h`, so **every example in its own header** was silently mis-parsed — it returned the trace (5, not 19) then segfaulted; alphabet widened to `a`–`z` with real validation. Three interval enclosure-soundness violations fixed (`ivl_sin` under-approximated across extrema, `ivl_sqrt` returned `[0,NaN]` which `ivl_contains` treated as universal, `ivl_div` missed `-0.0`). **4 of the 8 never-tested modules brought into the suites** (`einsum`, `lie_ext`, `mat3`, `linalg_precision`); coverage 50% → **54%**. 1063 |
| 2.6.12 | 2026-08-03 | 16,600 | 34 | **Audit sweep — repair release.** Full P(-1) audit of all 34 modules found **70 verified defects (2 critical)**; see `audit/2026-08-03.md`. Closed the critical tier: **seven hand-encoded constant tables** did not encode their documented values — **47 constants re-derived** from exact rationals. DOPRI45 had 23 of 30 tableau constants wrong (Σb = 0.636, not 1) and was **not a consistent integrator at any order**; BDF-4 drifted 1%/step; Yoshida-4 moved a free particle 85.9% of the correct distance; Gauss-5 carried a 246 ppm error floor; plus sRGB breakpoints, spherical harmonics, `_SIMPLEX_G2`, the slerp threshold. Also fixed `num_is_prime` (i64 overflow reported real primes composite above 3.03e9) and `solve_bicgstab` (`f64_from` on a bit pattern made the tolerance 4.34e18 — it never iterated). Added **`scripts/check-constants.sh`**, a CI gate verifying all 110 hex f64 literals against their comments. Replaced the loose assertions that let it all ship (dopri45 asserted only `1 < y < 2`). 981 |
| 2.6.11 | 2026-08-03 | 16,600 | 34 | Toolchain 6.4.69 → **6.5.6** (a **minor** jump across 24 releases) + sakshi 2.4.6 → **2.4.7**. No executable library change — the bundle diff is the version header plus one `mat_new_guarded` doc comment, zero code lines. **Security:** the vendored `lib/ganita.cyr` was stale at **1.0.3** (the 6.4.69 pin already shipped 1.0.4); re-vendoring closes the tracked CWE-190 in stdlib `mat_new` — `mat_new(-5, 3)` **segfaulted** on 1.0.3, returns null on 1.0.4 (measured, same compiler, only ganita swapped). **Fixed:** all four `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768 failures truncated to 0 and scored PASS — now clamped. Adopted the 6.5.6 `sys_exit_group` epilogue. `cyrius fuzz` now discovers `tests/*.fcyr` (1 passed — previously never run). Smoke string 2.6.10 → 2.6.11. Tracked issues re-verified still-live (interval-ident-lex, for-empty-clauses). +4 assertions pinning the upstream `mat_new` contract. 961 |
| 2.6.10 | 2026-07-21 | 16,600 | 34 | Toolchain 6.4.66 → **6.4.69** (clean 3-patch bump; sakshi unchanged at 2.4.6, already latest). No library source change — bundle byte-identical bar the header. Vendored stdlib picks up three upstream fixes: `fmt` hex-high-bit + `fmt_float_buf` non-finite guard (linked by `symbolic`; byte-identical for finite values), `math` float-parse DoS hardening, agnos-only `sys_reboot` widening. Smoke string 2.6.9 → 2.6.10. Tracked issues re-verified still-live (interval-ident-lex, for-empty-clauses); no new fixes. 957 |
| 2.6.9 | 2026-07-17 | 16,600 | 34 | Toolchain 6.3.11 → **6.4.66** + sakshi 2.4.2 → **2.4.6**. Infrastructure + test-only fix — no library source change; bundle byte-identical bar the header. Fixed a pre-existing `tests/modules.tcyr` compile failure (`iv_add`/`iv_sub`/`iv_mul` collide with reserved cycc SIMD intrinsic names; renamed `iv_sum`/`iv_diff`/`iv_prod`), restoring the suite to 312/312. Smoke string 2.6.7 → 2.6.9. New interval-ident-lex issue filed; for-empty-clauses still open. 957 |
| 2.6.8 | 2026-07-06 | 16,600 | 34 | Collision hardening for co-compilation with the sandhi/TLS stack: `symbolic` float-render scratch moved `var buf[N]` → `alloc(N)` (dodges the "array size must be enum constant" path under `tls`/`dynlib` co-compile); bare error constants namespaced `ERR_*` → `HSB_ERR_*` (values unchanged) to stop a last-wins global collision on consumers. 957 |
| 2.6.7 | 2026-06-30 | 16,600 | 34 | Toolchain 6.2.11 → **6.3.11** + sakshi 2.1.0 → **2.4.2**. Infrastructure-only — no library source change; bundle byte-identical bar the header. `lib/result.cyr` `_die` agnos-portability fix; smoke version string 2.3.3 → 2.6.7. for-empty-clauses still open on 6.3.11 (no new fixes). 957 |
| 2.6.6 | 2026-06-15 | 16,600 | 34 | Toolchain 6.0.14 → **6.2.11**. Stdlib math reorg: transcendentals + matrix/linalg → new `ganita` umbrella; `math` gains NaN-correct `f64_le`/`f64_ge` (dropped local copies). `[deps]`: +ganita −matrix −linalg. 3 of 5 tracked toolchain bugs fixed (archived). 957 |
| 2.6.5 | 2026-05-30 | 16,600 | 34 | Diffgeo arc COMPLETE — P(-1)/security audit (posture solid) + `math.md §2` differential-geometry reference. Docs-only, 957 |
| 2.6.4 | 2026-05-29 | 16,600 | 34 | Diffgeo arc — higher-order forms (`wedge_2_1`/`wedge_3_1`); 8 wedge antisymmetry/grading assertions. 957 |
| 2.6.3 | 2026-05-29 | 16,580 | 34 | Diffgeo arc — geodesic deviation / Jacobi (`geodesic_deviation`); 6 sphere/flat/linearity assertions. 949 |
| 2.6.2 | 2026-05-29 | 16,560 | 34 | Diffgeo arc — parallel transport (`parallel_transport`, RK4); 4 flat/sphere length-preservation assertions. 943 |
| 2.6.1 | 2026-05-29 | 16,540 | 34 | Diffgeo arc — Weyl conformal-curvature tensor (`weyl_tensor`); 5 space-form/trace-free assertions. 939 |
| 2.6.0 | 2026-05-29 | 16,520 | 34 | Diffgeo arc — sectional curvature (`sectional_curvature` from Riemann); 5 space-form/sphere assertions. 934 |
| 2.5.4 | 2026-05-29 | 16,500 | 34 | CGA arc closeout — P(-1)/security audit (posture solid) + `architecture/math.md` equation catalogue. Docs-only, 929 |
| 2.5.3 | 2026-05-29 | 16,500 | 34 | CGA arc — `mat_new_guarded` (CWE-190 real-matrix guard); 4 assertions. 929 |
| 2.5.2 | 2026-05-29 | 16,490 | 34 | CGA arc — blade projection/rejection (`cga_project`/`cga_reject` + blade inverse); 10 assertions. 925 |
| 2.5.1 | 2026-05-29 | 16,480 | 34 | CGA arc — dual + pseudoscalar inverse (`cga_pseudoscalar`/`cga_dual`); 6 GA-identity assertions. 915 |
| 2.5.0 | 2026-05-29 | 16,470 | 34 | CGA arc — contraction operators (`cga_left_contraction`/`cga_right_contraction`); 8 GA-identity assertions. 909 |
| 2.4.6 | 2026-05-29 | 16,460 | 34 | Security/hardening audit — posture solid, no new vuln; 6 alloc-guard tests + threat-model refresh. 901 |
| 2.4.5 | 2026-05-29 | 16,460 | 34 | Collision arc COMPLETE — contact solver fixed (impulse was always 0); solve_pgs verified; 7 assertions. 895 |
| 2.4.4 | 2026-05-28 | 16,460 | 34 | Collision arc — MPR narrowphase fixed (separated pairs were false +ve); 10 assertions. 888 |
| 2.4.3 | 2026-05-28 | 16,450 | 34 | Collision arc — half-edge mesh audited (no bug; twin/boundary wiring correct); 11 assertions. 878 |
| 2.4.2 | 2026-05-28 | 16,450 | 34 | Collision arc — `delaunay_2d` audited (no bug; cocircular-robust); 8 empty-circumcircle assertions. 867 |
| 2.4.1 | 2026-05-28 | 16,450 | 34 | Collision arc — `triangulate_polygon` audited (no bug); 13 tiling/count assertions added. 859 |
| 2.4.0 | 2026-05-28 | 16,450 | 34 | Collision arc — `convex_hull_2d` fixed (broken insertion sort + undefined `f64_le`/`f64_ge`); 13 assertions added. 846 |
| 2.3.4 | 2026-05-28 | 16,424 | 34 | Layout/idiom modernization — `alloc(sizeof(T))`+derived setters (13 modules), enum-const grid/buffer sizes, `#must_use` on core API. Codegen-identical, 833/833 |
| 2.3.3 | 2026-05-28 | 16,195 | 34 | Safety/numerical audit — no bugs; fixed wrong `>>` comment + 8 invariant tests. 833/833 |
| 2.3.2 | 2026-05-28 | 16,195 | 34 | Bounded einsum scratch via reused arena — 3960 → 176 B/call (~22×). Memory-only, 825/825 |
| 2.3.1 | 2026-05-28 | 16,195 | 34 | SIMD hot paths (`f64v_*`) for vec/mat/quat — vec4 dot 6.5×, m4_mul 4.5×, m3_mul 3.2×. Bit-identical, 825/825 |
| 2.3.0 | 2026-05-28 | 16,195 | 34 | Cyrius 6.0.14 toolchain; library source moved to `src/`; sakshi resolution repaired; CI aligned to abaco (fmt/security/version gates). No behavioral change |
| 2.2.0 | 2026-04-15 | 15,676 | 33 | SE(3), SO(3), adjoint, BCH, spatial structures, MPR, impulse solver, simplex noise, einsum, Golub-Kahan SVD |
| 2.1.0 | 2026-04-15 | 13,715 | 30 | Golub-Kahan SVD, QR eigen, complex QR, simplex noise, einsum |
| 2.0.0 | 2026-04-15 | 11,943 | 27 | Cyrius port from Rust. P(-1) audit. |
| Rust 1.4.0 | 2026-03-30 | 33,612 | 65 | Final Rust release. Available via pre-2.0 git tags. |

---

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | parses and evaluates | -- |
| `hvec3_cross(a, b)` | -- | vec3.cyr |
| `geo_ray_sphere(ray, sphere)` | -- | geo.cyr |
| `calc_integral_simpson(&f, a, b, n, out)` | -- | calc.cyr |
| `num_newton(&f, &df, x0, tol, max, out)` | -- | num.cyr |
| `sym_integrate(expr, var)` | -- | symbolic_ext.cyr |
| `sym_to_latex(expr)` | -- | symbolic_ext.cyr |

Hisab should never depend on abaco. Abaco may optionally depend on hisab.
✅ **Verified 2026-09-09**: 0 hits for `abaco` in `cyrius.cyml`, `cyrius.lock`, `dist/hisab.deps`,
`dist/hisab.cyr` and `src/`. The only git dep is sakshi 2.5.1, and `deps --verify` already fails any
unreviewed dep — **no new gate is owed here.** ⚠ The apparent contradiction between `eval("sin(pi/4)")`
being abaco's while hisab exposes `expr_eval` is not one: `src/symbolic.cyr:254` takes a **tree**, not
a string. hisab has no tokenizer at all.

⚠ **This table is frozen at the 2.2.0 surface.** It has no row for `expr_eval` — whose **domain
changed** in 2.11.2 (`(-2)^3` returned NaN for hisab's entire history and returns a number now), and
whose named consumer is abaco — and none for autodiff (forward duals + the reverse tape), the six
`geo_diff` jets, CGA, or Lie. A boundary table that lags the surface it describes is how a consumer
learns the boundary from a compile error instead.
