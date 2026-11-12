# Existenz prototype: executable specification and simulation

This directory contains the first working base of the Existenz protocol, as a
Rust workspace with two crates and zero external dependencies:

- **`crates/exz-core`**: the deterministic rules engine. It is an executable
  specification of the founding rules of the white paper: equal daily issuance
  (Section 4), accumulation cap (5.2), melting (5.1), the melting exemption
  floor and sub-wallets (Annex C), the single-hop rule, settlement credit,
  confirmation window, the three conversion caps, commission (Annex F), the
  coverage-indexed global fund, rollout gate and ration queue (Annex G), and
  the bounded merchant circuit (Annex H.2). Integer-only: balances in
  milli-EXZ, local amounts in cents.
- **`crates/exz-sim`**: a multi-zone simulation CLI over the engine. Four
  zones by default (a pilot at Annex G.5 scale, a thin adoption zone, a
  deliberately under-endowed stress zone, and an oversubscribed dense zone
  where the solidarity scale rule visibly throttles counting), deterministic
  seeded RNG, daily spend/circuit/convert loop, CSV and SVG outputs, and a
  stdout summary that compares the effective per-person-per-year payout
  against the Annex G.1 projection. In world fabric mode (`--world`), zones
  are generated from real country data (population, FAO undernourishment
  prevalence): 170 countries, 426 zones, 2.1M simulated agents representing
  7.87 billion people on a laptop.

What this is NOT: not a blockchain, not a ledger, no cryptography, no
identity, no networking, no wallet UI. The white paper remains the source of
truth; this prototype exists to test the rules and tune them by iteration.

## How to run

```bash
# run the 24 rule tests (one per founding rule, plus regression tests)
cargo test --manifest-path prototype/Cargo.toml

# run the 90-day, 4-zone simulation (release build recommended: 100k persons)
cargo run --release --manifest-path prototype/Cargo.toml -p exz-sim -- \
  --days 90 --seed 1 --out ./out

# natural-scale run: 170 countries -> 426 zones, 2.1M simulated agents
# representing 7.87B people; lower --agents-per-zone on small machines
cargo run --release --manifest-path prototype/Cargo.toml -p exz-sim -- \
  --days 90 --seed 1 --world prototype/world/world_fabric.csv \
  --agents-per-zone 5000 --out ./out_world
```

Outputs land in `out/`:

- `exz_sim.csv`: one row per day and zone (spent, converted, commission,
  queued, melted, cap overflow, reverted, coverage ratio).
- `exz_sim.svg`: two panels, coverage ratio per zone (reference line at 1.0)
  and daily converted payout per zone.

Determinism: same `--seed`, same run, byte-identical outputs.

## Parameters and where they come from

`exz_core::Params` (defaults in `lib.rs`) maps one-to-one to the white paper:

| Field | Default | Paper section |
| --- | --- | --- |
| `daily_issuance_mexz` | 2000 (2 EXZ) | Section 4, Scenario B |
| `cap_days` | 2 | Section 5.2 |
| `melt_bps_per_day` / `melt_idle_days` | 167 / 1 | Section 5.1 |
| `floor_days` | 14 | Annex C.1 |
| `subwallet_max_days` | 60 | Annex C.2 |
| `settlement_window_days` | 7 | Annex F.2 |
| `confirm_days` | 1 | Annex E.6 |
| `circuit_fraction_bps` / `circuit_window_days` | 2000 / 7 | Annex H.2 |
| `commission_bps` / cap | 300 / 50 allowances | Annex F.5 |
| `tier_initial_allowances` / `integrity_alpha_bps` | 100 / 2000 | Annex F.4 |
| `zone_headroom_bps` | 2000 | revised F.4 (proposed) |
| `overshoot_melt_bps_per_day` | 500 | revised F.4 (proposed) |
| `reserve_horizon_days` | 90 (H) | Annex G.3 |
| `payout_avg_days` | 7 | Annex G.3 |

## The solidarity scale rule (revised F.4, proposed)

A 100-cover restaurant must not fit a protocol addressed to people in need
and small merchants. The prototype implements a three-step rule, proposed as
a revision of Annex F.4 (the paper is NOT yet modified; this awaits review):

1. **Merchant table cap (soft, per merchant).** Declared covers P saturate at
   the zone class's solidarity table size P_max:
   `P_eff = P_max x tanh(P / P_max)`, rounded to the nearest cover and
   clamped by the declared count. Small tables are counted in full, a
   100-cover restaurant at a 30-cover table counts 30, with no cliff.
   Reference values: tanh fixed point matches libm within 1e-10
   (tanh(1) = 0.7616, tanh(2/3) = 0.5828, tanh(5/3) = 0.9311,
   tanh(10/3) = 0.9975).
2. **Zone cover quota with degressive headroom.** Covers count at full
   weight up to the zone's daily quota Q; beyond Q the marginal weight
   decays linearly to zero at Q x (1 + 2h), so the zone's total counted
   covers never exceed Q x (1 + h) with h = 20%: twenty percent above the
   quota, reached progressively, no cliff.
3. **Accelerated overshoot melting.** A merchant declaring more than
   P_max x (1 + h) covers is in overshoot; while in overshoot its confirmed
   settlement credit melts at 5% per day, and the melted units return to the
   commons fund. Growth beyond the solidarity table becomes a cost, not an
   asset.

Proposed malnutrition classes (sim comments only, calibrations to debate):

| Class | Prevalence of undernourishment | P_max |
| --- | --- | --- |
| stable | below 5% | 100 |
| vulnerable | 5 to 15% | 60 |
| alarming | 15 to 25% | 40 |
| critical | above 25% | 30 |

Structural consequence, the merchant density rule. If covers per merchant
per day is d = N x u / M, aggregate counting capacity is
N x u x tanh(r)/r with r = d / P_max, and the efficiency tanh(r)/r decreases
in r. So the tables do not throttle a zone when
`M >= N x u / (r* x P_max)` where tanh(r*)/r* = v. At the pilot scale
(N = 100k, u = 0.60, P_max = 60, v = 0.80): M >= 1053 merchants; the pilot
fabric (1500 merchants) passes, which is why its payout still matches the
G.1 projection. Too few, too-large tables silently tax the whole zone.

## The world fabric: a natural-scale test bed

`exz-sim --world` turns the four hand-made zones into a planet: it reads
`world/world_fabric.csv` (real population and FAO prevalence of
undernourishment per country, latest year, via Our World in Data, CC BY; see
`world/SOURCES.md`) and generates one zone per ~25M-person chunk of each
country. Each zone is:

- **classed by its country's PoU**, which sets the solidarity table P_max
  (revised F.4) and a utilization band for u (u ramps with PoU inside the
  class band; the band itself is an assumption pending field data),
- **endowed as a rollout stage**: 55% of zones at full projected coverage,
  15% at 75%, 20% at 50%, 10% at 30% (the ration queue does visible work),
- **stocked with merchants by the structural density rule**
  M >= N x u / (r* x P_max) with r* solved from tanh(r*)/r* = v, then a
  health factor in [0.6, 1.6]: about a third of fabrics are deliberately
  under-tabled, and pay for it,
- **a scaled replica**: ~5000 simulated agents per zone, each standing for
  its share of the real population chunk; totals are scaled back to the
  represented population for reporting.

Reference run (seed 1, 90 days, 426 zones, 2,130,000 agents, 7.87B people
represented, ~43 s within 1 GB):

| Class | Zones | People | P_max | Effective vs projected (USD/p/yr) | G.1 invariant |
| --- | --- | --- | --- | --- | --- |
| stable | 200 | 3.65B | 100 | 68.42 vs 91.73 | 84/84 |
| vulnerable | 151 | 2.93B | 60 | 123.19 vs 170.58 | 49/49 |
| alarming | 51 | 0.90B | 40 | 185.98 vs 248.19 | 19/19 |
| critical | 24 | 0.38B | 30 | 215.04 vs 308.34 | 8/8 |

Planetary, scaled: 861.2 B USD/year actually paid, against 1177.2 B USD/year
if every zone were fully endowed (the gap is the ration working on the 45%
of zones in partial rollout). Cross-check with the paper: Annex G's
1,750 B USD/year assumes uniform u = 0.60; this fabric's PoU-weighted
utilization is u_eff = 0.415, and 1750 x 0.415/0.60 = 1210 B, consistent
with the 1177 B full-coverage projection.

World-fabric findings, honestly:

1. **The G.1 invariant holds zone by zone where it must**: on every
   fully-endowed, comfortably dense zone, the effective payout stays within
   15% of the Annex G.1 projection (160/160 at seed 1, 153/153 at seed 2,
   152/152 at seed 3; worst deviation 7.3%, Somalia). Class averages sit
   below projection because they mix in the under-endowed and under-tabled
   zones, by construction.
2. **Zone cover quotas never bind when set by the rule** (Q = N x u): the
   quota is the anti-capture backstop; everyday bounding is done by the
   tanh table caps (312k to 653k cap events per class over 90 days) and by
   the ration queue.
3. **Under-tabled fabrics pay.** Zones whose merchant count falls below the
   density rule melt settlement credit continuously through the overshoot
   rule (2.5 to 17.4 B EXZ per class over 90 days, scaled): the accelerated
   melting acts as a real tax on fabrics that try to concentrate the flow in
   too few tables.
4. **Replica guard**: zones whose density ratio resolves to fewer than 8
   merchants are floored to 8, because a one-merchant replica is a monopoly
   artifact a real fabric would not have. With FAO's PoU floor of 2.5% no
   zone in the current fabric triggers it; it is a guard, not a lever.
5. **Machine limits, stated**: 2.13M agents in ~43 s within a 1 GB cgroup;
   4.26M agents (10k per zone) exceed it. Scaling is linear in agents x
   days; the engine itself is unchanged and the 24 rule tests are untouched.

## What the simulation showed (honest findings)

1. **The Annex G.1 projection holds at pilot scale.** With N = 100,000,
   u = 0.60, v = 0.80, p = $0.60, the simulated effective payout is
   218.16 USD per person per year against the 216.55 USD projection
   (0.7% deviation), with the fund paying exactly the steady-state daily
   payout D. The stress zone, endowed at half coverage, settles into the
   transparent ration as designed: partial same-day payment within the
   inflow budget, the rest queued.

2. **Person-level melting is vacuous with the proposed values** (already
   documented in the tests): the exemption floor (14 days = 28 EXZ) exceeds
   the accumulation cap (2 days = 4 EXZ), so the cap alone does the
   anti-accumulation work. Melting only bites on settlement credit past the
   operational window. The paper's Section C values should be revisited if
   melting is meant to act on persons.

3. **Two engine bugs were found by the simulation and fixed** (each now
   covered by a regression test): the coverage ratio compared the allocation
   against a per-conversion-event average instead of the daily payout total,
   and the ration budget reset at each conversion request instead of being
   shared across the day. Both made under-covered zones overspend their
   allocation. This is exactly the kind of iteration the prototype is for.

4. **The bounded circuit raises total conversion demand above D.** The G.1
   formula counts each unit once (N x u x v x 2p), but the Annex H.2 circuit
   re-sends unconverted merchant credit to other merchants, part of which
   converts later. A zone endowed at exactly H x D with inflow D therefore
   lives slightly above its budget and keeps a standing queue, with merchant
   credit aging out at the 7-day window. Zone endowments should either
   include a margin or count the circuit share in D.

5. **The solidarity scale rule bites exactly where it should, and exposes a
   calibration coupling.** In the dense zone (10k persons, 300 merchants at
   30-cover tables, quota Q = 2,000 covers/day deliberately below demand),
   the degressive band throttles counting at Q + 20% (14,698 zone-quota cap
   events over 90 days) and the effective payout falls to 85.45 USD per
   person per year against a 169.18 projection: the policy lever works. But
   because the endowment was computed from the unthrottled projection, the
   zone ends over-covered (final coverage ratio 2.80) with the surplus
   parked in the fund. When a class quota throttles a zone, the endowment
   formula H x D_projected over-allocates; endowments should be computed
   from the throttled projection, or the surplus should be explicitly
   recycled.

6. **Overshoot melting fires in under-tabled fabrics, gently.** In the thin
   and stress zones a few merchants receive more unique visitors per day
   than P_max x 1.2; their settlement credit melts at 5% per day while the
   overshoot lasts (565 EXZ over 90 days in thin, 53 in stress, both small
   against the zones' volume). In the pilot and dense zones no merchant
   overshoots. The penalty is real but proportionate; whether 5%/day is the
   right rate needs adversarial testing.

7. **An open design question:** queued conversion requests survive the
   7-day reversion of the underlying settlement credit. A merchant can in
   principle be paid from the queue days after its backing credit reverted
   to the fund. The queue and the operational window interact; the paper
   should say explicitly which one prevails.

## Next steps

- Identity and presence stubs (Annex E), to test the declaration ritual and
  fabrication penalties with adversarial actors.
- A hash-chain day ledger (append-only, deterministic) to make the engine
  replayable as a ledger runtime candidate.
- Wallet-shaped integration tests, and WASM compilation of exz-core.
- Parameter sweeps (u, v, endowment coverage) to tune the rollout gate.
- Adversarial actors against the scale rule: cover-splitting across
  sub-merchants, recycled unique visitors, quota-band gaming via conversion
  order.
- Shock scenarios on the world fabric: famine (u doubles locally), food
  price spike, merchant die-off, refugee influx; multi-seed statistical
  acceptance in CI.
- Regional meal prices from the cost-of-a-healthy-diet data instead of the
  uniform Annex G meal price.
- Revision of Annex F.4 in the white paper itself, pending review: the
  implemented rule is the working proposal, the paper text is unchanged.
