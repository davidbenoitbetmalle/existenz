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

The operating rules on one screen:

> - **Issuance.** Every verified human existence receives the same 2 EXZ per
>   day; 1 EXZ is defined as the price of one basic meal.
> - **No hoarding.** Balances are capped per account and idle balances melt;
>   melted units recycle into the global solidarity fund. The currency is a
>   medium of subsistence and circulation, not a store of value.
> - **One hop.** Persons pay merchants directly (single-hop rule); merchants
>   settle into local currency once per day at a bounded, coverage-indexed,
>   abuse-resistant rate. No conversion channel for anyone else.
> - **Anchored value.** The conversion rate follows the local price of the
>   basic-needs basket: what is guaranteed to every holder everywhere is the
>   meal, not a currency amount.
> - **Solidarity scale** (proposed revision of Annex F.4, implemented and
>   tested in the prototype). Zones are classed by malnutrition prevalence,
>   and the class sets the maximum "solidarity table" per merchant: 100
>   covers in stable zones, 60 in vulnerable, 40 in alarming, 30 in critical.
>   A 100-cover restaurant fits nowhere. The zone's total accepted volume
>   follows its real need (its daily cover quota, counted with a degressive
>   20% headroom), while the harder the zone, the smaller the tables: the
>   flow must pass through a dense fabric of small local merchants, not be
>   captured by big ones where dependence is highest.
> - **Overshoot penalty.** A merchant declaring more covers than its table
>   scale loses 5% of its settlement credit per day until it comes back
>   within scale. Growing beyond the solidarity table is a cost, not an
>   asset.
> - **Merchant density.** Because tables shrink with severity, a zone needs
>   enough active small merchants for its volume; too few, too-large tables
>   silently throttle the whole zone.
> - **Governance.** Parameters belong to the account owners under a
>   two-thirds rule; founding stewardship is a transition whose measure of
>   success is its own obsolescence.

Formulas, class calibrations and simulation numbers: [prototype/README.md](prototype/README.md).
Full reasoning and known limits: the [white paper](https://davidbenoitbetmalle.github.io/existenz/).
