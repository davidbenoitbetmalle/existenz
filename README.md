# Existenz: a Fair Global Currency

## Start here

**[Where this paper is most likely to be wrong: three open questions](https://github.com/davidbenoitbetmalle/existenz/discussions/2)**
(planetary-scale identity, solvency arithmetic, melting parameters that
have never been measured)

This project's first need is not praise but objections. Read the
[white paper](https://davidbenoitbetmalle.github.io/existenz/), then attack
it in the discussion above.

This repository holds the source and the published website of the Existenz
(EXZ) white paper: a protocol-level proposal for a fair global virtual
currency that recognizes every human existence, deliberately published with
its known limits stated openly.

The white paper is published as a single-page website on GitHub Pages:
[`https://davidbenoitbetmalle.github.io/existenz/`](https://davidbenoitbetmalle.github.io/existenz/)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). The project particularly welcomes
criticism: Section 11 of the white paper exists precisely because the
authors believe a proposal like this must be attacked before it can be
trusted.

## How it works

The operating rules on one screen: [HOW_IT_WORKS.md](HOW_IT_WORKS.md).

## The prototype

The monetary rules are being turned into an executable specification, next
to the paper. The global structure being built:

- `docs/existenz_wp.md`: the white paper, source of truth.
- `index.html`: the published single-page website, generated from the paper
  by `scripts/build_site.py` (site CI runs on `docs/` and `scripts/`
  changes).
- `prototype/`: a Rust workspace, zero external dependencies, integer-only
  (balances in milli-EXZ, local amounts in cents):
  - `crates/exz-core`: the deterministic rules engine, the founding rules
    as code. Equal daily issuance, accumulation cap, melting, the melting
    exemption floor and sub-wallets, the single-hop rule, settlement credit
    and the confirmation window, the three conversion caps, commission, the
    coverage-indexed global fund, rollout gate and ration queue, the bounded
    merchant circuit, and the solidarity scale rule (soft tanh table cap per
    merchant, zone cover quota with a 20% degressive headroom, accelerated
    overshoot melting). 24 rule and regression tests.
  - `crates/exz-sim`: the multi-zone simulation CLI over the engine. Four
    zones (a pilot at Annex G.5 scale, a thin adoption zone, an under-endowed
    stress zone, an oversubscribed dense zone), deterministic seeded RNG,
    CSV and SVG outputs, and a sanity check of the effective payout against
    the Annex G.1 projection.
- `.github/workflows/prototype.yml`: runs the test suite and a 14-day
  simulation on every change under `prototype/`.

What the prototype is NOT: no blockchain, no ledger, no cryptography, no
identity, no networking, no wallet UI. It exists to test the rules and tune
them by iteration, and it has already found and fixed engine bugs the paper
arithmetic missed.

Formulas, calibrations and simulation findings:
[prototype/README.md](prototype/README.md). Current state and open
questions: [PROJECT_STATUS.md](PROJECT_STATUS.md).
