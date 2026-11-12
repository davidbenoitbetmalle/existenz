# How Existenz works

The operating rules of the protocol on one screen. Full reasoning, known
limits and references: the [white paper](https://davidbenoitbetmalle.github.io/existenz/).
Formulas, class calibrations and tested numbers: [prototype/README.md](prototype/README.md).

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
> - **Solidarity scale** (Annex F.4, implemented and
>   tested in the prototype). Zones are classed by malnutrition prevalence,
>   and the class sets the maximum "solidarity table" per merchant. A
>   100-cover restaurant fits nowhere. The zone's total accepted volume
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

Proposed zone classes (calibrations open to debate):

| Zone class | Prevalence of undernourishment | Max covers per merchant |
| --- | --- | --- |
| stable | below 5% | 100 |
| vulnerable | 5 to 15% | 60 |
| alarming | 15 to 25% | 40 |
| critical | above 25% | 30 |
