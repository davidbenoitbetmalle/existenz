# Project Status

An honest statement of where Existenz stands, in the same spirit as Section 11
of the white paper. This page is updated as the project progresses.

## What exists today

- The white paper (v3): a complete protocol-level specification covering
  monetary rules, identity, offline verification, merchant settlement,
  governance and a solvency model, published with its known limits stated
  openly (Sections 9 and 11, Annexes A to G).
- A specification of extended offline operation (Annex E.11): day-chain
  elapsed-time proof, tolling, opportunistic Bluetooth relay sync.
- An order-of-magnitude solvency model (Annex G): at 60% utilization and 80%
  conversion, roughly 1,750 B USD per year, about 1.7% of world GDP; pilot of
  100,000 people at roughly 22 M USD per year, about 220 USD per person.
- A rules-engine prototype and multi-zone simulation (`prototype/`): the
  monetary rules as an executable specification, 24 tests, zero dependencies,
  integer-only. The Annex G.1 pilot projection is verified by simulation
  within 0.7%, and the solidarity scale rule (proposed revision of Annex
  F.4: soft table cap, zone cover quota, accelerated overshoot melting) is
  implemented and simulated. See [prototype/README.md](prototype/README.md).

## What is specified but not yet built

- No reference implementation of the full protocol exists. The prototype
  covers the monetary rules only: no identity, no cryptography, no ledger,
  no networking.
- RHUP (Annex A) specifies an identity web of trust, liveness attestations
  and zero-knowledge credentials; no cryptographic implementation, even
  prototypical, has been written or audited.
- The cardless presence mode (Annex E.4) depends on on-device one-to-many
  identification against a zone registry; the accuracy of such a system on
  low-quality captures is an assumption, not a measurement.
- The day-chain (E.11) assumes a secure element able to run the daily
  hash-chain computation; no hardware partner has been engaged.

## What is acknowledged as unsolved

- Identity at planetary scale is priced in the design rather than solved
  (Section 11.3).
- Full-utilization conversion cannot be an unconditional same-day guarantee;
  it is a rationed, coverage-indexed public service (11.1, Annex G).
- An EXZ-rich fund is not a solvent fund; solvency requires external inflows
  (11.2, G.3).
- State opposition, governance realism and merchant-economy dependence are
  open questions, not engineering tasks (11.4 to 11.6).

## What the project is looking for

- An economist or economic historian to stress-test the monetary mechanics
  (melting parameters, cap levels, the BNB index) against complementary
  currency field data.
- A cryptographer to review RHUP (Annex A) and the offline credential design
  (Annexes E and F), and to advise on the zero-knowledge liveness proofs.
- A governance specialist to critique the merchant voting design (Annex D).
- Field partners (humanitarian operators, complementary currency networks)
  for a bounded pilot, far below the 100,000-person scale of Annex G.
- Developers, when the specification has been sufficiently challenged: the
  first code contributions will be executable models of the melting
  mechanics, the day-chain and the BNB index sampling.
- Readers willing to attack the document: open an issue per weakness found.
  Critique is the most valuable first contribution.

## How to contribute

Read the [white paper](https://davidbenoitbetmalle.github.io/existenz/),
then open an issue or a pull request against `docs/existenz_wp.md`.
See [CONTRIBUTING.md](CONTRIBUTING.md).
