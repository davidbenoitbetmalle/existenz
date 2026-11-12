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
- **`crates/exz-sim`**: a multi-zone simulation CLI over the engine. Three
  zones by default (a pilot at Annex G.5 scale, a thin adoption zone, and a
  deliberately under-endowed stress zone), deterministic seeded RNG, daily
  spend/circuit/convert loop, CSV and SVG outputs, and a stdout summary that
  compares the effective per-person-per-year payout against the Annex G.1
  projection.

What this is NOT: not a blockchain, not a ledger, no cryptography, no
identity, no networking, no wallet UI. The white paper remains the source of
truth; this prototype exists to test the rules and tune them by iteration.

## How to run

```bash
# run the 19 rule tests (one per founding rule, plus regression tests)
cargo test --manifest-path prototype/Cargo.toml

# run the 90-day, 3-zone simulation (release build recommended: 100k persons)
cargo run --release --manifest-path prototype/Cargo.toml -p exz-sim -- \
  --days 90 --seed 1 --out ./out
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
| `reserve_horizon_days` | 90 (H) | Annex G.3 |
| `payout_avg_days` | 7 | Annex G.3 |

## What the simulation showed (honest findings)

1. **The Annex G.1 projection holds at pilot scale.** With N = 100,000,
   u = 0.60, v = 0.80, p = $0.60, the simulated effective payout is
   217.98 USD per person per year against the 216.55 USD projection
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

5. **An open design question:** queued conversion requests survive the
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
