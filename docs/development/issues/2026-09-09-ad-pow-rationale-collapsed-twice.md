# 2026-09-09 — `_ad_pow`'s reason to exist has now collapsed twice, and what survives is narrow

**Component:** `src/autodiff.cyr`, `_ad_pow` (and `dual_pow`, which is its only caller).
**Severity:** Low — no wrong answer. This is a *should it still be here* question, raised because
both of the reasons written beside the code are now false.
**Status:** 🟡 **OPEN — filed, not decided.** 2.11.4 rewrote the comment to say what is actually
true; it did not change the code.

## The two dead rationales

`_ad_pow` computes `base^n` by repeated multiplication for integral `n`, deferring to the stdlib
otherwise. It has been justified twice, and upstream has invalidated both:

| release | stated reason | killed by |
|---|---|---|
| original | **DOMAIN** — ganita 1.0.4's `f64_pow` was literally `exp(n·ln base)`, so every negative base was NaN and `d/dx x³` at −2 was NaN where the truth is 12 | ganita **1.1.4** special-cased zero base, zero exponent, and negative base with integral exponent |
| 2.11.2 | **PRECISION** — 1.1.4 took the magnitude from `exp(n·ln\|base\|)` and applied the sign, so `f64_pow(-2,3)` was −7.99999999999999982 and `(-2)^4` read **15** through the truncating `f64_to` against **16** here | ganita **1.2.4** replaced that path with **binary exponentiation** for integral `\|n\| <= 1024` |

Measured on 1.2.4: **665 of 665 comparisons agree BIT FOR BIT** — integral `n` in −12..12 over
bases −12..12, plus half-exponents over bases −20..19. `(-2)^4` now reads 16 through both paths.

## ⭐ What survives is real, measured, and the OPPOSITE of the second reason

Squaring amplifies relative error geometrically, so binary exponentiation gives up accuracy exactly
where it buys speed. Against a 60-digit reference:

| input | `_ad_pow` | ganita 1.2.4 | winner |
|---|---|---|---|
| `(-0.999)^1000` — inside the ±1024 window (binary exponentiation) | **4 ulp** | 58 ulp | `_ad_pow` |
| `(-0.999)^1100` — past the window (exp/ln fallback) | 3 ulp | **0 ulp** | ganita |
| `(-0.999)^1101` — past the window, odd exponent | 3 ulp | **1 ulp** | ganita |

So the honest summary: repeated multiplication is **bit-identical** for small `n`, **more accurate**
than the stdlib for large `n` inside the window, and **less accurate** past it — while costing
**O(n)** multiplies against the stdlib's **O(log n)**.

[measured: scratch harness, not reproducible in-tree, 2026-09-09; ganita 1.2.4]

## The question

Is a narrow accuracy edge at large integral exponents — a regime `dual_pow` may never be called in —
worth a linear loop and a second implementation of `pow`? Three ways to close this:

1. **Keep it, and pin the surviving ground with a test.** There is currently no assertion anywhere
   that exercises the case `_ad_pow` actually wins (large `n` inside the window). Without one, the
   only reason it still exists is untested.
2. **Delete it and call the stdlib**, accepting 58 ulp at `n = 1000` — and measure whether any hisab
   caller reaches that regime first.
3. **Narrow it** — defer to the stdlib below some `n` and loop only above it.

⚠ **Do not close this by reading the ulp table alone.** It was taken at one base (−0.999, chosen
because it neither overflows nor underflows over 1100 multiplies). Whether the crossover sits at
`n = 100` or `n = 900` is unmeasured, and option 3 needs that number.

## Why it is filed rather than fixed

2.11.4 is the deprecated-alias migration. Changing which `pow` a differentiable primitive calls is a
numerical-behaviour change to a public entry point (`dual_pow`), and it belongs in its own cycle with
its own before/after. **The comment beside the code now states the measured truth rather than a
rationale that stopped being true two releases ago** — that was the in-scope part.
