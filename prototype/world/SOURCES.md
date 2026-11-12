# Data sources of world_fabric.csv

`world_fabric.csv` is a derived table: one row per country, with the
population, the prevalence of undernourishment (PoU) and the PoU reference
year.

## Sources

- **Prevalence of undernourishment (PoU)**: FAO, SDG Indicator 2.1.1
  (prevalence of undernourishment, percent), accessed through the
  Our World in Data grapher `prevalence-of-undernourishment`
  (https://ourworldindata.org/grapher/prevalence-of-undernourishment).
  Value used: the latest year available per country (2023 or 2024).
- **Population**: UN World Population Prospects, accessed through the
  Our World in Data grapher `population`
  (https://ourworldindata.org/grapher/population).
  Value used: the latest year available per country (2023).

Both Our World in Data graphs are published under CC BY 4.0; the derived
table here credits them and is under the same license terms.

## Method

- Latest available year per country for each indicator, inner join on the
  ISO country code.
- 170 countries, 7.87 billion people covered. Countries lacking a PoU value
  in the FAO series are absent (about 0.3B people); they would be added the
  same way when data exists.
- FAO reports many low-prevalence countries as "< 2.5%" (recorded as 2.5).

## Assumptions added by the prototype (not from the data)

- Meal price: 0.60 USD PPP per meal, uniform, per the white paper's Annex G
  (humanitarian food-basket costing practice). A regional refinement from
  the cost-of-a-healthy-diet series is planned future work.
- Utilization bands (u) per malnutrition class and the rollout endowment
  mix are prototype assumptions, documented in `../README.md`.
