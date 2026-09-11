# EPA certification is ASYMMETRIC between `gjk_epa_3d` and `mpr_penetration` — the seed differs, not the certificate

*(Original title, retained because it is what the body argues and what 2.9.3 refuted: "EPA's certified exit is never taken — `strict` tests the SEED tetrahedron, but the lower-bound argument is about the FINAL polytope".)*

**Status:** 🟢 **CLOSED 2026-09-10 (2.19.0). The trade was DECLINED on measurement in 2.18.0, and
2.19.0 then repaired the thing this file was circling — which was never the certificate.**

⛔ **THE SEED TRADE IS 36,000x LESS ACCURATE, WHICH INVERTS THIS FILE'S OWN PREMISE.** Against the
exact closed form `ra + rb - |c1 - c2|`, `gjk_epa_3d` is 1.41e-16 mean / 5.57e-16 worst; the
"upgrade" this file proposes is 1.70e-14 / 2.03e-11. The mechanism was already written in the
source: `_epa_polish`'s own comment says the depth is a `min` over probed directions and every probe
is an upper bound, so the polish **can only lower an upper bound** — and certifying is an EARLY-OUT
that skips it. Two of this file's four claims are false as written.

⭐ **AND RE-MEASURING THE TRADE FOUND SOMETHING LARGER THAN THE TRADE.** `mpr_penetration` and
`gjk_epa_3d` disagreed by up to **1.65e-05** on ordinary overlapping spheres (mean 1.37e-08 against
1.41e-16), both public, one-line truth — and `_ag_mprpen` compared only their RETURN CODES, so
nothing in the suite compared their depths. **The cause was the POLISH BUDGET, not the certificate
and not either mechanism this file argues.** 38 of `_EPA_POLISH_ITER`'s 64 rounds are mandatory
halvings from 0.25 rad to `EPSILON_F64`, leaving 26 for moves, and the portal seed starts further
out so it was the one running out. 2.19.0 set it to 128: the two entry points now agree to the last
bits, **1.65e-05 -> 6.3e-16**, with no measurable cost (256 changes nothing further).

⚠ **Kept as a record of the method, not the conclusion.** Every substantive claim in the body below
has now been refuted by measurement — first the "certified exit is never taken" claim in 2.9.3, then
both proposed repairs in 2.18.0. **A filing is a hypothesis, and this one was wrong three times.**

*(Previous status: 🟡 OPEN — RE-DIAGNOSED 2026-08-09 (2.9.3), BOTH proposed repairs
measured and rejected.)* Read the *Re-diagnosis* section before the body: this file's central claim
is wrong, and its Proposed fix produces incorrect depths.
**Placement:** still wants its own cycle, but the cycle is now about the *sphere family*, not the
certificate — see Re-diagnosis §3.
**Discovered:** 2026-08-06, closing the 2.9.0 mutation sweep's U6–U11 "coverage gap".
**Severity:** Low, downgraded from Medium. Not a wrong answer, and **not** a dead code path either
(that was the error) — it is an inconsistency in *which* path two public entry points take to reach
the same answer.

## Re-diagnosis (2.9.3)

### 1. ⛔ "The certified exit is never taken" is FALSE — it only measured one entry point

The 12/12 fallback below is real for `gjk_epa_3d`. It is **0/12 for `mpr_penetration`**, which
computes the same quantity through the same `_epa_refine`:

```
gjk_epa_3d      box polish increments:  12   (certifies none)
mpr_penetration box polish increments:   0   (certifies all 12, returns 1 on all 12)
```

So the certified path is not unreachable, `_epa_polish` is not "the primary path", and the
**"Why this explains U6–U11" section is not established** — the exit those six repairs improve does
run, on every `mpr_penetration` overlap.

The cause is the **seed**, not the certificate. `_epa_seed_portal` tries a strictness upgrade — swap
`v0` for the support point opposite the far face, keep it only if the result strictly encloses the
origin (`geo_advanced.cyr:757-766`) — and `_epa_seed_gjk` never did. The portal path therefore
arrives certifiable and the GJK path arrives one rounding step short. **The real defect is a
certification asymmetry between two public entry points.**

### 2. ⛔ The Proposed fix below produces WRONG DEPTHS — measured

"Drop the seed test and rely on `lo > 0`" was implemented. Boxes go **12 → 0** fallbacks, as this
file predicts. But **four exact-tangency pairs then report a nonzero depth for a touch whose true
depth is 0** — `_ag_baddepth` in the 110-pair agreement sweep goes **0 → 4**. `lo > 0` is therefore
*not* the property the certificate needs: at exact tangency `lo` can be marginally positive from
rounding, and the seed test was the thing catching it. **The seed test is load-bearing.** Do not
apply this file's Proposed fix.

### 3. The cheaper alternative works but costs 19%, and is not obviously worth it

Giving `_epa_seed_gjk` the portal's strictness upgrade: **every assertion in the suite passes**
(including every exact-MTV depth on both entry points) and boxes go **12 → 6** fallbacks. But it adds
one Minkowski support evaluation per seed and measures **+19.4% on `gjk_epa_sphere_box`** — 150.4 µs
against 125.6 µs, three runs each, non-overlapping, well clear of the ~7% noise floor. Spheres pay
the extra support call and *still* fall back, which points at a separate root cause in the sphere
family (they do not certify even with a strict seed).

**No returned answer changes either way** — the polish already recovers the depth, and the two entry
points agree to 1e-9. So the trade is: pay 19% on a narrowphase hot path to make an internal code
path consistent between entry points and to make six repairs mutatable. That is a judgement call
rather than a defect fix, and 2.9.3 did not take it. **The code is unchanged.**

### What is actually left

The sphere-family non-convergence (§3) is the open question, and it is independent of the
certificate wording. Anything done here still needs the exact-MTV bar — which is *in* the suite now
(`collision_core: narrowphase depth == exact MTV`, plus the 110-pair agreement sweep), so a future
attempt can be scored in-tree rather than against an external harness.
**Affects:** hisab 2.8.3 onward.

## Summary

`_epa_refine` sets `certified = 1` only when

```
if (f64_gt(lo, 0) == 1 && strict == 1) { certified = 1; }     src/geo_advanced.cyr:906
```

`strict` is `_epa_tetra_strict(seed)` (`:853`), evaluated **once, on the GJK seed tetrahedron**,
requiring all four face gaps to exceed `EPSILON_F64 * lmax`.

**That condition never holds in practice.** Measured over 24 overlaps across two shape families —
boxes and spheres, swept from shallow to deep — the certified exit was taken **0 times**. Every
call fell back to `_epa_polish`.

So the polish is not a fallback. It is the primary path, and the EPA refinement loop is a seed
generator for it.

## The measurement

`_GA_EPA_POLISH_COUNT` (added 2.9.1) counts handoffs. With the shipped condition:

| fixture family | calls | fell back to polish |
|---|---|---|
| box/box, 12 offsets | 12 | **12** |
| sphere/sphere, 12 radii | 12 | **12** |

Dropping **only** `strict` from the certificate — one edit, `strict == 1` removed, `lo > 0` kept:

| fixture family | calls | fell back to polish |
|---|---|---|
| box/box, 12 offsets | 12 | **0** |

So `lo > 0` is satisfied routinely and `strict` is the sole blocker. (Mutation applied and
reverted; `src/geo_advanced.cyr` md5-verified back to `176ae917fb2142a3de5aa75589574623`.)

## Why it never holds

GJK terminates as soon as its simplex **contains** the origin — which routinely leaves the origin
within rounding distance of a face. `_epa_tetra_strict` then demands a margin of
`EPSILON_F64 * lmax` on all four faces of that same simplex, and rejects it.

The deeper problem is the **object being tested**. The comment at `:903-905` states the reasoning:

> The same applies when the SEED was not strictly enclosing: the lower-bound argument assumes the
> origin is inside the polytope, so a seed that only just contains it cannot certify anything.

But the lower-bound argument is about the polytope **at the moment of certification**, not about
the seed. EPA only ever *adds* vertices, so an expanded polytope encloses the origin at least as
well as the seed did. Testing the seed's margin is therefore both the wrong object and needlessly
conservative — it condemns every run for a property of its starting state.

## Why this explains U6–U11

The 2.9.0 mutation sweep found six EPA repairs whose reversion produced **bit-identical** output,
and could not classify them as coverage gaps. This is why: the exit those repairs improve is
unreachable, so nothing downstream of it can be observed. It was not that the fallback *sometimes*
masked them — the certified path never runs at all.

## Proposed fix

Evaluate strict enclosure against the **current** polytope at the point of certification rather
than against the seed, or drop the seed test and rely on `lo > 0` (which is itself the
positive-lower-bound condition the argument needs).

**Do not land this without the exact-reference harness.** Certification changes which value is
returned — the certified EPA depth instead of the polished one — on every overlapping pair. That is
a narrowphase behaviour change and must be scored against the 862-configuration exact-MTV reference
(15-axis OBB SAT in `Fraction`, closed forms for spheres) that 2.8.3 established, with the same
zero-wrong-sign bar. It should also be benchmarked: skipping the pattern search on every overlap is
plausibly a real win, and this repo does not accept a performance claim without before/after
numbers.

## Note

2.9.1 pins the current state in `tests/modules.tcyr` — assertions that all 12 box and all 12 sphere
overlaps fall back, plus a negative control proving the counter is not unconditional. **When this
is fixed those assertions will FAIL, and that is the intended signal.** Update them then and say so
in the CHANGELOG.

---

## ⚠ 2026-09-09 (v2.11.3) — THE 19.4% FIGURE IS STALE. Re-measure before judging the trade.

Nothing about the diagnosis above changed, and the code is still unchanged. But the **cost side of
the trade-off was measured against a baseline that no longer exists**, and this file is the only
place that number is recorded — so a later reader would weigh a 2026-09 decision with a 2026-08
price tag.

cyrius 6.5.71 put `#derive(accessors)` getters on the inline-replay path, and hisab's narrowphase is
built almost entirely out of those getters. `gjk_epa_sphere_box` moved on the toolchain bump alone,
no source change:

| | `gjk_epa_sphere_box` |
|---|---|
| baseline when 19.4% was measured (2.9.3) | **125.6 us** |
| baseline at 2.11.2 (cyrius 6.5.33) | 111.5 us |
| baseline at 2.11.3 (cyrius 6.6.1) | **78.5 us** |

[measured: bench-history.csv, benchmark `gjk_epa_sphere_box`, runs 2026-08-21 and 2026-09-09]

⛔ **Do not rescale the old percentage — re-measure both halves.** It is tempting to reason that the
repair adds a roughly constant amount of work (one extra Minkowski support evaluation per seed), so
against a baseline that fell 125.6 -> 78.5 us the same absolute addition would now cost ~32% rather
than 19.4%, making the trade *worse*. That is an inference, not a measurement, and it is probably
wrong in detail: the added support evaluation is itself accessor-heavy geometry and will have been
sped up by the same inlining, so the ratio may be roughly preserved. **The two halves moved for the
same reason and neither was measured after the move.** The honest statement is that the percentage
is unknown as of 2.11.3.

**Close condition unchanged**, with one addition: whoever takes the sphere-family cycle must
re-measure the strict-seed variant on the *current* pin before deciding, rather than reading 19.4%
off this file. The whole 2.11.3 release was an exercise in numbers that were written down once and
then trusted; this is one of them, caught here rather than after the decision.
