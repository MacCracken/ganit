# 2026-08-04 incircle repros — archived 2026-09-09 (v2.11.3)

`2026-08-04-incircle-repro.tcyr` and `2026-08-04-incircle-repro-realistic.tcyr` were left in the
**open** `issues/` directory after both of their parent filings were archived
(`2026-08-04-incircle-precision.md`, fixed in 2.9.1; `2026-08-06-ghost-incircle-g1-g3-assume-ccw.md`,
fixed in 2.9.3). They were orphans: a reader opening the open-issues directory saw two files
implying live defects.

Re-run on cyrius 6.6.1 / hisab 2.11.3 before moving them, rather than moved on the assumption that
an archived parent means a fixed child:

```
2026-08-04-incircle-repro.tcyr             mixed-magnitude disagreements with EXACT (of 40): 0
2026-08-04-incircle-repro-realistic.tcyr   spread 1e13: wrong of 30 = 0
```

Both report **zero** disagreements, so the predicates they were written to indict now agree with
the exact reference on every case. Kept rather than deleted — they are the only fixtures in the
tree that exercise `_col_in_circumcircle` at mixed magnitudes, and they are cheap to re-run.

⚠ Note for a future reader: both scripts end with `println(fmt_int(count))`, and `fmt_int` **prints**
its argument and then returns a value that `println` prints as well. A count of 0 therefore renders
as `00`, and a count of 7 would render as `70` — read the FIRST digits, not the whole line. That is
a defect in the fixture's output, not in the predicate.
