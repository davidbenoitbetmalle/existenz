//! exz-sim: multi-zone simulation over the exz-core rules engine.
//!
//! Not a blockchain and not a forecast: a deterministic, seeded replay of the
//! protocol rules at zone scale, to observe how issuance, the accumulation
//! cap, the conversion tiers (Annex F.4), the circuit bound (Annex H.2) and
//! the coverage ration (Annex G.3) interact day by day. Three zones are
//! simulated by default:
//!
//! - pilot:  the Annex G.5 scenario (100k persons, u = 0.60, v = 0.80),
//!           endowment and inflow sized for full coverage.
//! - thin:   a sparse adoption zone (5k persons, u = 0.30, v = 0.50).
//! - stress: an under-endowed zone (allocated = 0.5 x H x D, inflow = 0.3 x D)
//!           where the ration queue and the coverage rule do visible work.
//! - dense:  an oversubscribed fabric (demand far above its class cover
//!           quota) where the degressive quota band visibly throttles the
//!           zone's counting at quota + 20%.
//!
//! Outputs a CSV (one row per day and zone), an SVG chart, and a stdout
//! summary with the per-person-per-year payout compared to the Annex G.1
//! projection.
//!
//! World fabric mode: `--world <file.csv>` generates one or more zones per
//! country from real data (population and FAO prevalence of undernourishment
//! via Our World in Data, CC BY), classes each zone with the proposed revised
//! F.4 table, calibrates merchant density with the structural rule, and
//! scales every simulated agent to the real population share it represents.

use exz_core::{projected_daily_payout_cents, CapReason, DayStats, Params, World};
use std::env;
use std::fs;
use std::path::Path;

/// Deterministic splitmix64: same seed, same run, replayable forever.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next_u64() % n
        }
    }
    fn bps_hit(&mut self, bps: u64) -> bool {
        self.below(10_000) < bps
    }
}

/// Static description of one rollout zone.
struct ZoneSpec {
    id: u64,
    name: String,
    persons: u64,
    merchants: u64,
    meal_price_cents: u64,
    /// u: probability that a person spends the daily allowance today.
    spend_bps: u64,
    /// v: fraction of confirmed credit the merchant converts daily.
    convert_bps: u64,
    /// Share of persons who never spend (accumulation-cap pressure).
    hoarder_bps: u64,
    /// Probability that a merchant with credit circuits part of it today.
    circuit_bps: u64,
    /// Solidarity table size of the zone's class (revised F.4): P_max.
    scale_cap: u64,
    /// Daily cover quota of the zone (revised F.4): Q.
    quota_covers: u64,
    /// Endowment = allocated_coverage x H x D_projected.
    allocated_coverage: f64,
    /// Daily inflow = inflow_frac x D_projected.
    inflow_frac: f64,
}

impl ZoneSpec {
    fn plausible_population(&self) -> u64 {
        (self.persons + self.merchants - 1) / self.merchants
    }
}

fn default_specs() -> Vec<ZoneSpec> {
    // Malnutrition-class profiles (revised F.4): the class sets the table
    // size P_max and the zone's daily cover quota Q. Proposed classes:
    //   PoU < 5% (stable):       P_max = 100
    //   PoU 5-15% (vulnerable):  P_max = 60
    //   PoU 15-25% (alarming):   P_max = 40
    //   PoU > 25% (critical):    P_max = 30
    // Q is the zone's expected served covers (N x u), except where the
    // class deliberately rations the fabric (dense).
    vec![
        // Pilot, Annex G.5 scale, vulnerable class: dense fabric of small
        // tables, one merchant per ~67 inhabitants.
        ZoneSpec {
            id: 1,
            name: "pilot".to_string(),
            persons: 100_000,
            merchants: 1_500,
            meal_price_cents: 60,
            spend_bps: 6_000,
            convert_bps: 8_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            scale_cap: 60,
            quota_covers: 60_000,
            allocated_coverage: 1.0,
            inflow_frac: 1.0,
        },
        // Thin adoption, alarming class.
        ZoneSpec {
            id: 2,
            name: "thin".to_string(),
            persons: 5_000,
            merchants: 45,
            meal_price_cents: 40,
            spend_bps: 3_000,
            convert_bps: 5_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            scale_cap: 40,
            quota_covers: 1_500,
            allocated_coverage: 1.0,
            inflow_frac: 1.0,
        },
        // Under-endowed, critical class: the ration queue must appear.
        ZoneSpec {
            id: 3,
            name: "stress".to_string(),
            persons: 20_000,
            merchants: 1_000,
            meal_price_cents: 80,
            spend_bps: 9_000,
            convert_bps: 10_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            scale_cap: 30,
            quota_covers: 18_000,
            allocated_coverage: 0.5,
            inflow_frac: 0.3,
        },
        // Oversubscribed fabric, critical class with a quota deliberately
        // below demand: the degressive band caps counting at Q + 20%.
        ZoneSpec {
            id: 4,
            name: "dense".to_string(),
            persons: 10_000,
            merchants: 300,
            meal_price_cents: 50,
            spend_bps: 5_000,
            convert_bps: 9_000,
            hoarder_bps: 200,
            circuit_bps: 3_000,
            scale_cap: 30,
            quota_covers: 2_000,
            allocated_coverage: 1.0,
            inflow_frac: 1.0,
        },
    ]
}

const CHART_COLORS: [&str; 4] = ["#d95f18", "#3a6ea5", "#6b4fa0", "#3a7a5a"];

struct Args {
    days: u64,
    out: String,
    seed: u64,
    /// World fabric mode: path to the country CSV.
    world: Option<String>,
    /// Real population chunk represented by one zone (world mode).
    pop_per_zone: u64,
    /// Simulated agents per zone (world mode); each represents its share.
    agents_per_zone: u64,
}

fn parse_args() -> Args {
    let mut args = Args { days: 90, out: "./out".to_string(), seed: 1, world: None, pop_per_zone: 25_000_000, agents_per_zone: 10_000 };
    let argv: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--days" if i + 1 < argv.len() => {
                i += 1;
                if let Ok(d) = argv[i].parse() {
                    args.days = d;
                }
            }
            "--out" if i + 1 < argv.len() => {
                i += 1;
                args.out = argv[i].clone();
            }
            "--seed" if i + 1 < argv.len() => {
                i += 1;
                if let Ok(s) = argv[i].parse() {
                    args.seed = s;
                }
            }
            "--world" if i + 1 < argv.len() => {
                i += 1;
                args.world = Some(argv[i].clone());
            }
            "--pop-per-zone" if i + 1 < argv.len() => {
                i += 1;
                if let Ok(v) = argv[i].parse() {
                    args.pop_per_zone = v;
                }
            }
            "--agents-per-zone" if i + 1 < argv.len() => {
                i += 1;
                if let Ok(v) = argv[i].parse() {
                    args.agents_per_zone = v;
                }
            }
            _ => {}
        }
        i += 1;
    }
    args
}

fn main() {
    let args = parse_args();
    let (specs, metas, n_countries) = match &args.world {
        Some(path) => world_specs(path, args.seed, args.pop_per_zone, args.agents_per_zone),
        None => (default_specs(), Vec::new(), 0),
    };
    let params = Params::default();
    let commission_bps = params.commission_bps;
    let horizon = params.reserve_horizon_days;
    let allowance = params.daily_issuance_mexz;
    let days = args.days;
    let n_zones = specs.len();

    // Projected daily payout D per the Annex G.1 formula, used to size the
    // endowment (H x D) and the daily inflow of each zone fund.
    let d_proj: Vec<f64> = specs
        .iter()
        .map(|s| {
            projected_daily_payout_cents(
                s.persons,
                s.spend_bps as f64 / 10_000.0,
                s.convert_bps as f64 / 10_000.0,
                s.meal_price_cents,
                commission_bps,
            )
        })
        .collect();

    let mut world = World::new(params.clone());
    for (i, s) in specs.iter().enumerate() {
        let allocated = (s.allocated_coverage * horizon as f64 * d_proj[i]) as i64;
        let inflow = (s.inflow_frac * d_proj[i]) as i64;
        world
            .add_zone(
                s.id,
                &s.name,
                s.meal_price_cents,
                s.plausible_population(),
                s.scale_cap,
                s.quota_covers,
                allocated,
                inflow,
            )
            .expect("zone setup");
        for p in 0..s.persons {
            world.enroll_person(s.id, p).expect("enroll person");
        }
        for m in 0..s.merchants {
            world.enroll_merchant(s.id, m).expect("enroll merchant");
        }
    }

    // One independent RNG stream per zone, all derived from the master seed.
    let mut rngs: Vec<Rng> = specs
        .iter()
        .map(|s| Rng::new(args.seed ^ (s.id.wrapping_mul(0x9E3779B97F4A7C15))))
        .collect();

    // Aggregated daily grid: [day][zone].
    let mut grid: Vec<Vec<DayStats>> =
        vec![vec![DayStats::default(); n_zones]; days as usize];
    // Which conversion cap bound, per zone (Annex F.4 diagnostics).
    let mut cap_counts: Vec<[u64; 5]> = vec![[0; 5]; n_zones];
    // Coverage ratio per (day, zone), recorded after the day's conversions.
    let mut coverage: Vec<Vec<f64>> = vec![vec![0.0; n_zones]; days as usize];

    for day in 1..=days {
        world.new_day();

        for (zi, s) in specs.iter().enumerate() {
            let zone = s.id;
            let hoarders = s.persons * s.hoarder_bps / 10_000;

            // 1. Persons spend the daily allowance with probability u,
            //    at a uniformly random merchant (single-hop rule, Annex F.1).
            for p in 0..s.persons {
                if p < hoarders {
                    continue;
                }
                if rngs[zi].bps_hit(s.spend_bps) {
                    let m = rngs[zi].below(s.merchants);
                    // A living person's balance is always at least one
                    // allowance after issuance (cap = 2 allowances), so the
                    // daily allowance can always be spent in full.
                    let _ = world.spend(zone, p, m, allowance);
                }
            }

            // 2. Merchants circuit part of their credit to other merchants
            //    (Annex H.2 bound: 20% of the weekly received credit).
            let merchants = s.merchants;
            for m in 0..merchants {
                if !rngs[zi].bps_hit(s.circuit_bps) {
                    continue;
                }
                let merchant = &world.zones[zi].merchants[m as usize];
                let confirmed = merchant.confirmed_total();
                if confirmed == 0 {
                    continue;
                }
                let window = world.params.circuit_window_days;
                let received_7d: u64 = merchant
                    .received_hist
                    .iter()
                    .filter(|(d, _)| world.day.saturating_sub(*d) <= window)
                    .map(|(_, a)| a)
                    .sum();
                let spent_7d: u64 = merchant
                    .circuit_spent_hist
                    .iter()
                    .filter(|(d, _)| world.day.saturating_sub(*d) <= window)
                    .map(|(_, a)| a)
                    .sum();
                let bound = received_7d * world.params.circuit_fraction_bps / 10_000;
                let headroom = bound.saturating_sub(spent_7d);
                if headroom == 0 {
                    continue;
                }
                let amount = confirmed.min(headroom);
                if amount == 0 {
                    continue;
                }
                let target = {
                    let t = rngs[zi].below(merchants - 1);
                    if t >= m {
                        t + 1
                    } else {
                        t
                    }
                };
                // Errors (bounds already reached) are part of the protocol:
                // a failed circuit is simply not executed.
                let _ = world.circuit_spend(zone, m, target, amount);
            }

            // 3. End-of-day conversion: each merchant converts the v
            //    fraction of its confirmed credit (Annexes F.3 to F.5).
            //    The order is shuffled daily for fairness: both the zone
            //    quota band and the ration budget are order-sensitive, so
            //    no merchant may systematically convert first.
            let mut order: Vec<u64> = (0..s.merchants).collect();
            for i in (1..order.len()).rev() {
                let j = rngs[zi].below((i + 1) as u64) as usize;
                order.swap(i, j);
            }
            for m in order {
                let confirmed = world.zones[zi].merchants[m as usize].confirmed_total();
                if confirmed == 0 {
                    continue;
                }
                let requested = confirmed * s.convert_bps / 10_000;
                if let Ok(receipt) = world.convert(zone, m, requested) {
                    let slot = match receipt.capped_by {
                        CapReason::NotCapped | CapReason::Request => 0,
                        CapReason::PhysicalCap => 1,
                        CapReason::ZoneQuotaCap => 2,
                        CapReason::PlausibilityCap => 3,
                        CapReason::TierCap => 4,
                    };
                    cap_counts[zi][slot] += 1;
                }
            }
        }

        for (zi, s) in specs.iter().enumerate() {
            coverage[(day - 1) as usize][zi] = world.coverage_ratio(s.id);
        }
    }

    // Fold the history into one row per (day, zone).
    for (d, zid, row) in &world.history {
        let zi = (*zid - 1) as usize;
        let cell = &mut grid[(*d - 1) as usize][zi];
        cell.spent_mexz += row.spent_mexz;
        cell.converted_cents += row.converted_cents;
        cell.commission_cents += row.commission_cents;
        cell.queued_cents += row.queued_cents;
        cell.melted_mexz += row.melted_mexz;
        cell.excess_mexz += row.excess_mexz;
        cell.reverted_mexz += row.reverted_mexz;
        cell.overshoot_melted_mexz += row.overshoot_melted_mexz;
    }

    // ---- outputs ----------------------------------------------------------

    let out_dir = Path::new(&args.out);
    fs::create_dir_all(out_dir).expect("create out dir");

    let mut csv = String::from("day,zone,spent_mexz,converted_cents,commission_cents,queued_cents,melted_mexz,excess_to_fund_mexz,reverted_mexz,overshoot_melted_mexz,coverage_ratio\n");
    for (di, row_zones) in grid.iter().enumerate() {
        for (zi, cell) in row_zones.iter().enumerate() {
            let cov = coverage[di][zi];
            let cov = if cov.is_finite() { format!("{:.3}", cov) } else { "inf".to_string() };
            if metas.is_empty() {
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{}\n",
                    di + 1,
                    specs[zi].name,
                    cell.spent_mexz,
                    cell.converted_cents,
                    cell.commission_cents,
                    cell.queued_cents,
                    cell.melted_mexz,
                    cell.excess_mexz,
                    cell.reverted_mexz,
                    cell.overshoot_melted_mexz,
                    cov
                ));
            } else {
                let sc = metas[zi].scale;
                csv.push_str(&format!(
                    "{},{},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{}\n",
                    di + 1,
                    specs[zi].name,
                    cell.spent_mexz as f64 * sc,
                    cell.converted_cents as f64 * sc,
                    cell.commission_cents as f64 * sc,
                    cell.queued_cents as f64 * sc,
                    cell.melted_mexz as f64 * sc,
                    cell.excess_mexz as f64 * sc,
                    cell.reverted_mexz as f64 * sc,
                    cell.overshoot_melted_mexz as f64 * sc,
                    cov
                ));
            }
        }
    }
    fs::write(out_dir.join("exz_sim.csv"), csv).expect("write csv");

    let svg = if metas.is_empty() {
        build_svg(&specs, &grid, &coverage, days)
    } else {
        build_svg_world(&metas, &grid, &coverage, days)
    };
    fs::write(out_dir.join("exz_sim.svg"), svg).expect("write svg");

    // ---- stdout summary ---------------------------------------------------

    println!("exz-sim: {} days, seed {}, {} zones", days, args.seed, n_zones);
    println!();

    if metas.is_empty() {
    for (zi, s) in specs.iter().enumerate() {
        let converted: i64 = world.zones
            [world.zones.iter().position(|z| z.id == s.id).unwrap()]
            .merchants
            .iter()
            .map(|m| m.converted_cents_total)
            .sum();
        let queued: i64 = world.zones
            [world.zones.iter().position(|z| z.id == s.id).unwrap()]
            .merchants
            .iter()
            .map(|m| m.queued_cents)
            .sum();
        let totals: DayStats = grid.iter().map(|r| r[zi]).fold(DayStats::default(), |a, b| DayStats {
            spent_mexz: a.spent_mexz + b.spent_mexz,
            converted_cents: a.converted_cents + b.converted_cents,
            commission_cents: a.commission_cents + b.commission_cents,
            queued_cents: a.queued_cents + b.queued_cents,
            melted_mexz: a.melted_mexz + b.melted_mexz,
            excess_mexz: a.excess_mexz + b.excess_mexz,
            reverted_mexz: a.reverted_mexz + b.reverted_mexz,
            overshoot_melted_mexz: a.overshoot_melted_mexz + b.overshoot_melted_mexz,
        });

        let eff_ppy = converted as f64 / 100.0 / s.persons as f64 * (365.0 / days as f64);
        let proj_ppy = d_proj[zi] * 365.0 / s.persons as f64 / 100.0;
        let final_cov = coverage[(days - 1) as usize][zi];

        println!("zone {} (id {}): {} persons, {} merchants, meal {}c", s.name, s.id, s.persons, s.merchants, s.meal_price_cents);
        println!("  class profile               {:>14} covers/day quota, {}-cover tables", s.quota_covers, s.scale_cap);
        println!("  spent (single-hop)          {:>14.2} EXZ", totals.spent_mexz as f64 / 1000.0);
        println!("  converted (paid)           {:>14.2} USD", converted as f64 / 100.0);
        println!("  commission charged          {:>14.2} USD", totals.commission_cents as f64 / 100.0);
        println!("  queued (rationed)           {:>14.2} USD", queued as f64 / 100.0);
        println!("  merchant credit reverted    {:>14.2} EXZ (7-day window)", totals.reverted_mexz as f64 / 1000.0);
        println!("  overshoot melted            {:>14.2} EXZ (accelerated, revised F.4)", totals.overshoot_melted_mexz as f64 / 1000.0);
        println!("  cap overflow to fund        {:>14.2} EXZ", totals.excess_mexz as f64 / 1000.0);
        println!("  melted (idle persons)       {:>14.2} EXZ", totals.melted_mexz as f64 / 1000.0);
        println!("  conversion caps hit         {:>14} physical / {} zone-quota / {} plausible / {} tier",
            cap_counts[zi][1], cap_counts[zi][2], cap_counts[zi][3], cap_counts[zi][4]);
        println!("  effective payout            {:>14.2} USD/person/year", eff_ppy);
        println!("  Annex G.1 projection        {:>14.2} USD/person/year", proj_ppy);
        println!("  final coverage ratio        {:>14.3}", if final_cov.is_finite() { final_cov } else { -1.0 });
        println!();
    }

    println!("global fund (EXZ commons, melting suspended): {:.2} EXZ", world.fund.exz as f64 / 1000.0);
    println!("global coverage ratio: {:.3}", world.global_coverage_ratio());

    // Sanity check on the pilot zone: the simulated effective payout should
    // stay within 15% of the Annex G.1 projection for the same parameters.
    let pilot = 0;
    let s = &specs[pilot];
    let converted: i64 = world.zones
        [world.zones.iter().position(|z| z.id == s.id).unwrap()]
        .merchants
        .iter()
        .map(|m| m.converted_cents_total)
        .sum();
    let eff_ppy = converted as f64 / 100.0 / s.persons as f64 * (365.0 / days as f64);
    let proj_ppy = d_proj[pilot] * 365.0 / s.persons as f64 / 100.0;
    let dev = (eff_ppy - proj_ppy).abs() / proj_ppy;
    if dev <= 0.15 {
        println!("pilot sanity check: PASS (effective {:.2} vs projected {:.2} USD/person/year, deviation {:.1}%)", eff_ppy, proj_ppy, dev * 100.0);
    } else {
        println!("pilot sanity check: FAIL (effective {:.2} vs projected {:.2} USD/person/year, deviation {:.1}%)", eff_ppy, proj_ppy, dev * 100.0);
    }
    } else {
        print_world_summary(&world, &specs, &metas, &grid, &coverage, &cap_counts, &d_proj, days, n_countries);
    }
    println!();
    println!("artifacts: {}/exz_sim.csv, {}/exz_sim.svg", args.out, args.out);
}

// ---------------------------------------------------------------------------
// World fabric mode: zones generated from real country data.
// ---------------------------------------------------------------------------

/// Proposed malnutrition classes (revised F.4). A country's prevalence of
/// undernourishment (PoU) selects the class; the class sets the solidarity
/// table size P_max and the utilization band for u (an assumption pending
/// field data, documented in prototype/README.md).
struct ClassProfile {
    name: &'static str,
    pou_min: f64,
    pou_max: f64,
    p_max: u64,
    u_lo: u64,
    u_hi: u64,
}

const CLASSES: [ClassProfile; 4] = [
    ClassProfile { name: "stable", pou_min: 0.0, pou_max: 5.0, p_max: 100, u_lo: 1_500, u_hi: 3_500 },
    ClassProfile { name: "vulnerable", pou_min: 5.0, pou_max: 15.0, p_max: 60, u_lo: 3_500, u_hi: 6_000 },
    ClassProfile { name: "alarming", pou_min: 15.0, pou_max: 25.0, p_max: 40, u_lo: 6_000, u_hi: 8_000 },
    ClassProfile { name: "critical", pou_min: 25.0, pou_max: 60.0, p_max: 30, u_lo: 8_000, u_hi: 9_900 },
];

fn class_index(pou: f64) -> usize {
    if pou < 5.0 {
        0
    } else if pou < 15.0 {
        1
    } else if pou < 25.0 {
        2
    } else {
        3
    }
}

/// r* such that tanh(r*)/r* = v: the largest per-merchant load ratio at which
/// the tables still count covers with efficiency v. Solved by bisection,
/// deterministic, no dependency.
fn density_r_star(v: f64) -> f64 {
    let f = |r: f64| r.tanh() / r - v;
    let mut lo = 0.05f64;
    let mut hi = 4.0f64;
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if f(mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

struct CountryRow {
    name: String,
    population: u64,
    pou: f64,
}

/// Minimal CSV reader for the fabric file:
/// country,code,population,pou_pct,pou_year
fn load_countries(path: &str) -> Vec<CountryRow> {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read world fabric {}: {}", path, e));
    let mut out = Vec::new();
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 4 {
            continue;
        }
        let pop: u64 = f[2].trim().parse().unwrap_or(0);
        let pou: f64 = f[3].trim().parse().unwrap_or(-1.0);
        if pop == 0 || pou < 0.0 {
            continue;
        }
        out.push(CountryRow { name: f[0].trim().to_string(), population: pop, pou });
    }
    out
}

/// Extra world data for each generated zone.
struct ZoneMeta {
    class: usize,
    pou: f64,
    /// Endowment coverage (allocated = endowed x H x D).
    endowed: f64,
    /// Real people represented by each simulated agent.
    scale: f64,
    represented: u64,
    /// Comfortably dense fabric (merchant health >= 0.95): the G.1
    /// invariant is expected to hold on these zones.
    healthy: bool,
    /// Endowed at full projected coverage (allocated = 1.0 x H x D).
    fully_endowed: bool,
}

/// One zone per population chunk of each country. Deterministic: same
/// fabric file and seed, same world.
fn world_specs(path: &str, seed: u64, pop_per_zone: u64, agents_cap: u64) -> (Vec<ZoneSpec>, Vec<ZoneMeta>, usize) {
    let countries = load_countries(path);
    let n_countries = countries.len();
    let mut rng = Rng::new(seed ^ 0x5F37_59DF_BEEF);
    let mut specs = Vec::new();
    let mut metas = Vec::new();
    let mut next_id = 1u64;
    for c in &countries {
        let nz = ((c.population + pop_per_zone - 1) / pop_per_zone).max(1);
        let chunk = (c.population + nz - 1) / nz;
        for k in 0..nz {
            let represented =
                if k + 1 < nz { chunk } else { c.population - chunk * (nz - 1) };
            let agents = agents_cap.min(represented).max(1);
            let ci = class_index(c.pou);
            let class = &CLASSES[ci];
            // Utilization ramps with the country's PoU inside its class band,
            // plus a 2% zone-level jitter.
            let frac = ((c.pou - class.pou_min) / (class.pou_max - class.pou_min))
                .clamp(0.0, 1.0);
            let u_base = class.u_lo as f64 + (class.u_hi - class.u_lo) as f64 * frac;
            let jitter = 0.98 + 0.04 * rng.below(1001) as f64 / 1000.0;
            let u_bps = ((u_base * jitter) as u64).clamp(500, 9_950);
            let v_bps = 7_000 + rng.below(2_001);
            // Merchant density per the structural rule:
            // M >= N x u / (r* x P_max). A health factor drawn in [0.6, 1.6]
            // lets some fabrics be deliberately under-tabled, where the
            // throttling of the whole zone becomes visible.
            let r_star = density_r_star(v_bps as f64 / 10_000.0);
            let m_req = ((agents as f64 * (u_bps as f64 / 10_000.0))
                / (r_star * class.p_max as f64))
                .ceil() as u64;
            let health = 0.6 + 0.01 * rng.below(101) as f64;
            // Replica granularity floor: a coarse replica whose density
            // ratio resolves to fewer than 8 merchants degenerates into a
            // single-table monopoly artifact that a real fabric of the same
            // ratio would not have (one table absorbing the whole zone).
            // Such zones are dense by construction.
            let (merchants, healthy) = if m_req < 8 {
                (8u64.min(agents.max(1)), true)
            } else {
                (((m_req as f64) * health).ceil().max(1.0) as u64, health >= 0.95)
            };
            // Rollout stage: endowment and inflow as a share of projected D.
            let roll = rng.below(100);
            let (cov, fully) = if roll < 55 {
                (1.0, true)
            } else if roll < 70 {
                (0.75, false)
            } else if roll < 90 {
                (0.5, false)
            } else {
                (0.3, false)
            };
            let inflow = cov * (0.85 + 0.15 * rng.below(1001) as f64 / 1000.0);
            let name = if nz == 1 { c.name.clone() } else { format!("{}#{}", c.name, k + 1) };
            specs.push(ZoneSpec {
                id: next_id,
                name,
                persons: agents,
                merchants,
                meal_price_cents: 60,
                spend_bps: u_bps,
                convert_bps: v_bps,
                hoarder_bps: 200,
                circuit_bps: 3_000,
                scale_cap: class.p_max,
                quota_covers: agents * u_bps / 10_000,
                allocated_coverage: cov,
                inflow_frac: inflow,
            });
            metas.push(ZoneMeta {
                class: ci,
                pou: c.pou,
                endowed: cov,
                scale: represented as f64 / agents as f64,
                represented,
                healthy,
                fully_endowed: fully,
            });
            next_id += 1;
        }
    }
    (specs, metas, n_countries)
}

/// World-mode summary: per-class aggregates scaled to the represented
/// population, planetary totals, and the G.1 invariant on fully-endowed,
/// comfortably dense zones.
#[allow(clippy::too_many_arguments)]
fn print_world_summary(
    world: &World,
    specs: &[ZoneSpec],
    metas: &[ZoneMeta],
    grid: &[Vec<DayStats>],
    coverage: &[Vec<f64>],
    cap_counts: &[[u64; 5]],
    d_proj: &[f64],
    days: u64,
    n_countries: usize,
) {
    let n = specs.len();
    let mut converted = vec![0i64; n];
    let mut queued = vec![0i64; n];
    for zi in 0..n {
        converted[zi] =
            world.zones[zi].merchants.iter().map(|m| m.converted_cents_total).sum();
        queued[zi] = world.zones[zi].merchants.iter().map(|m| m.queued_cents).sum();
    }
    let mut totals = vec![DayStats::default(); n];
    for row in grid {
        for (zi, cell) in row.iter().enumerate() {
            let t = &mut totals[zi];
            t.spent_mexz += cell.spent_mexz;
            t.converted_cents += cell.converted_cents;
            t.commission_cents += cell.commission_cents;
            t.queued_cents += cell.queued_cents;
            t.melted_mexz += cell.melted_mexz;
            t.excess_mexz += cell.excess_mexz;
            t.reverted_mexz += cell.reverted_mexz;
            t.overshoot_melted_mexz += cell.overshoot_melted_mexz;
        }
    }

    let agents_total: u64 = specs.iter().map(|s| s.persons).sum();
    let mut represented_total = 0u64;
    let mut converted_scaled = 0f64;
    let mut projected_daily_scaled = 0f64;
    for zi in 0..n {
        represented_total += metas[zi].represented;
        converted_scaled += converted[zi] as f64 * metas[zi].scale;
        projected_daily_scaled += d_proj[zi] * metas[zi].scale;
    }

    println!(
        "world fabric: {} countries -> {} zones, {} simulated agents representing {:.2}B people",
        n_countries, n, agents_total, represented_total as f64 / 1e9
    );
    println!("meal price: 0.60 USD PPP (Annex G assumption), {} days", days);
    let mix_full = metas.iter().filter(|m| m.endowed >= 1.0).count();
    let mix_75 = metas.iter().filter(|m| (m.endowed - 0.75).abs() < 1e-9).count();
    let mix_50 = metas.iter().filter(|m| (m.endowed - 0.5).abs() < 1e-9).count();
    let mix_30 = metas.iter().filter(|m| (m.endowed - 0.3).abs() < 1e-9).count();
    println!(
        "rollout endowment mix: {} zones at full coverage / {} at 75% / {} at 50% / {} at 30% of projected D",
        mix_full, mix_75, mix_50, mix_30
    );
    println!();

    let mut invariant_pass = 0usize;
    let mut invariant_total = 0usize;
    let mut worst: Vec<(f64, &str)> = Vec::new();

    println!("per malnutrition class (scaled to represented population):");
    for (ci, class) in CLASSES.iter().enumerate() {
        let idxs: Vec<usize> = (0..n).filter(|&zi| metas[zi].class == ci).collect();
        if idxs.is_empty() {
            continue;
        }
        let pop: u64 = idxs.iter().map(|&zi| metas[zi].represented).sum();
        let pou_w = idxs.iter().map(|&zi| metas[zi].pou * metas[zi].represented as f64).sum::<f64>()
            / pop as f64;
        let conv: f64 = idxs.iter().map(|&zi| converted[zi] as f64 * metas[zi].scale).sum();
        let proj: f64 = idxs.iter().map(|&zi| d_proj[zi] * metas[zi].scale).sum();
        let eff_ppy = conv / pop as f64 * (365.0 / days as f64) / 100.0;
        let proj_ppy = proj * 365.0 / pop as f64 / 100.0;
        // G.1 invariant: fully-endowed, comfortably dense zones must pay
        // within 15% of the Annex G.1 projection.
        let mut pass = 0usize;
        let mut considered = 0usize;
        for &zi in &idxs {
            if metas[zi].fully_endowed && metas[zi].healthy {
                considered += 1;
                let e = converted[zi] as f64 / specs[zi].persons as f64
                    * (365.0 / days as f64) / 100.0;
                let p = d_proj[zi] * 365.0 / specs[zi].persons as f64 / 100.0;
                let dev = if p > 0.0 { (e - p).abs() / p } else { 0.0 };
                if dev <= 0.15 {
                    pass += 1;
                }
                worst.push((dev, &specs[zi].name));
            }
        }
        invariant_pass += pass;
        invariant_total += considered;
        let q_caps: u64 = idxs.iter().map(|&zi| cap_counts[zi][2]).sum();
        let p_caps: u64 = idxs.iter().map(|&zi| cap_counts[zi][1]).sum();
        let pl_caps: u64 = idxs.iter().map(|&zi| cap_counts[zi][3]).sum();
        let omelt: f64 = idxs
            .iter()
            .map(|&zi| totals[zi].overshoot_melted_mexz as f64 * metas[zi].scale / 1000.0)
            .sum();
        let queued_scaled: f64 =
            idxs.iter().map(|&zi| queued[zi] as f64 * metas[zi].scale / 100.0).sum();
        let cov_now: f64 = {
            let mut sum = 0.0;
            let mut cnt = 0.0;
            for &zi in &idxs {
                let c = coverage[(days - 1) as usize][zi];
                if c.is_finite() {
                    sum += c;
                    cnt += 1.0;
                }
            }
            if cnt > 0.0 { sum / cnt } else { -1.0 }
        };
        println!("  {:<10} {} zones, {:>5.2}B people, mean PoU {:>4.1}%, P_max {}", class.name, idxs.len(), pop as f64 / 1e9, pou_w, class.p_max);
        println!("    effective payout {:>7.2} vs projected {:>7.2} USD/person/year", eff_ppy, proj_ppy);
        println!("    G.1 invariant {} of {} fully-endowed dense zones within 15%", pass, considered);
        println!("    final coverage {:.2} | rationed {:.1} M USD queued | overshoot melted {:.1} EXZ", cov_now, queued_scaled / 1e6, omelt);
        println!("    caps: {} physical (tanh tables), {} zone-quota, {} plausible", p_caps, q_caps, pl_caps);
    }
    println!();

    let eff_b_yr = converted_scaled / 100.0 / 1e9 * (365.0 / days as f64);
    let proj_b_yr = projected_daily_scaled / 100.0 * 365.0 / 1e9;
    println!("planetary, scaled to {:.2}B represented people:", represented_total as f64 / 1e9);
    println!("  simulated converted payout   {:>8.1} B USD/year", eff_b_yr);
    println!("  fabric-projected payout      {:>8.1} B USD/year (G.1 formula, same fabric)", proj_b_yr);
    println!("  deviation                     {:>8.1}%", (eff_b_yr - proj_b_yr).abs() / proj_b_yr * 100.0);
    println!("  payout per person             {:>8.2} USD/year", converted_scaled / represented_total as f64 * (365.0 / days as f64) / 100.0);
    let fund_scale = represented_total as f64 / agents_total as f64;
    println!("  fund commons (scaled)         {:>8.0} M EXZ", world.fund.exz as f64 / 1000.0 * fund_scale / 1e6);
    println!();

    if invariant_total > 0 {
        let verdict = if invariant_pass == invariant_total { "PASS" } else { "FAIL" };
        println!(
            "world G.1 invariant: {} ({} of {} fully-endowed dense zones within 15% of the Annex G.1 projection)",
            verdict, invariant_pass, invariant_total
        );
        worst.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        println!("worst deviations among invariant zones:");
        for (dev, name) in worst.iter().take(5) {
            println!("  {:.1}%  {}", dev * 100.0, name);
        }
    } else {
        println!("world G.1 invariant: no fully-endowed dense zones in the fabric");
    }
}

/// World-mode SVG: panel 1 plots the mean coverage ratio per malnutrition
/// class; panel 2 plots the planetary daily converted payout, scaled to the
/// represented population.
fn build_svg_world(metas: &[ZoneMeta], grid: &[Vec<DayStats>], coverage: &[Vec<f64>], days: u64) -> String {
    let n = metas.len();
    let width = 960.0;
    let panel_h = 360.0;
    let margin_l = 70.0;
    let margin_r = 20.0;
    let margin_t = 34.0;
    let plot_h = panel_h - margin_t - 40.0;
    let total_h = panel_h * 2.0 + 20.0;

    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{0}\" height=\"{1}\" viewBox=\"0 0 {0} {1}\" font-family=\"system-ui, sans-serif\" font-size=\"12\">\n",
        width as u32, total_h as u32
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#faf7f1\"/>\n");

    let x_of = |day: usize| -> f64 {
        margin_l + (day as f64 / (days as f64 - 1.0).max(1.0)) * (width - margin_l - margin_r)
    };

    // Panel 1: mean coverage per class.
    let y1 = 0.0;
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" font-size=\"15\" font-weight=\"600\">Mean coverage ratio per malnutrition class (R / (H x D-hat))</text>\n",
        margin_l, y1 + 20.0
    ));
    let cov_max = 3.0f64;
    let y_cov = |v: f64| -> f64 { y1 + panel_h - 40.0 - (v / cov_max).clamp(0.0, 1.0) * plot_h };
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{t:.1}\" x2=\"{m}\" y2=\"{b:.1}\" stroke=\"#3a230d\" stroke-width=\"1\"/>\n",
        m = margin_l, t = y1 + margin_t, b = y1 + panel_h - 40.0
    ));
    for (label, val) in [("0", 0.0f64), ("1.0", 1.0f64), ("3.0", cov_max)] {
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">{}</text>\n",
            margin_l - 8.0, y_cov(val) + 4.0, label
        ));
    }
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{y:.1}\" x2=\"{e:.1}\" y2=\"{y:.1}\" stroke=\"#3a230d\" stroke-dasharray=\"6 4\" stroke-width=\"1\" opacity=\"0.6\"/>\n",
        m = margin_l, y = y_cov(1.0), e = width - margin_r
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" text-anchor=\"middle\">day</text>\n",
        width / 2.0, y1 + panel_h - 12.0
    ));
    for (ci, _class) in CLASSES.iter().enumerate() {
        let mut pts = String::new();
        for di in 0..days as usize {
            let mut sum = 0.0;
            let mut cnt = 0.0;
            for zi in 0..n {
                if metas[zi].class != ci {
                    continue;
                }
                let v = coverage[di][zi];
                if v.is_finite() {
                    sum += v;
                    cnt += 1.0;
                }
            }
            let mean = if cnt > 0.0 { sum / cnt } else { cov_max };
            pts.push_str(&format!("{:.1},{:.1} ", x_of(di), y_cov(mean.min(cov_max))));
        }
        svg.push_str(&format!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            pts.trim_end(),
            CHART_COLORS[ci]
        ));
    }
    let mut lx = margin_l;
    for (ci, class) in CLASSES.iter().enumerate() {
        svg.push_str(&format!(
            "<rect x=\"{:.0}\" y=\"{:.1}\" width=\"12\" height=\"12\" fill=\"{}\"/>\n",
            lx, y1 + 26.0, CHART_COLORS[ci]
        ));
        svg.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{:.1}\" fill=\"#3a230d\">{}</text>\n",
            lx + 17.0, y1 + 37.0, class.name
        ));
        lx += 17.0 + 14.0 * class.name.len() as f64;
    }

    // Panel 2: planetary daily converted payout (scaled, B USD).
    let y2 = panel_h + 20.0;
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" font-size=\"15\" font-weight=\"600\">Planetary daily converted payout (B USD, scaled to represented population)</text>\n",
        margin_l, y2 + 20.0
    ));
    let mut pay_max = 1.0f64;
    for row in grid {
        let mut day_total = 0.0f64;
        for (zi, cell) in row.iter().enumerate() {
            day_total += cell.converted_cents as f64 * metas[zi].scale / 100.0 / 1e9;
        }
        if day_total > pay_max {
            pay_max = day_total;
        }
    }
    pay_max *= 1.1;
    let y_pay = |v: f64| -> f64 { y2 + panel_h - 40.0 - (v / pay_max).clamp(0.0, 1.0) * plot_h };
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{t:.1}\" x2=\"{m}\" y2=\"{b:.1}\" stroke=\"#3a230d\" stroke-width=\"1\"/>\n",
        m = margin_l, t = y2 + margin_t, b = y2 + panel_h - 40.0
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">0</text>\n",
        margin_l - 8.0, y_pay(0.0) + 4.0
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">{:.1}B</text>\n",
        margin_l - 8.0, y_pay(pay_max / 1.1) + 4.0, pay_max / 1.1
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" text-anchor=\"middle\">day</text>\n",
        width / 2.0, y2 + panel_h - 12.0
    ));
    let mut pts = String::new();
    for (di, row) in grid.iter().enumerate() {
        let day_total: f64 = row
            .iter()
            .enumerate()
            .map(|(zi, cell)| cell.converted_cents as f64 * metas[zi].scale / 100.0 / 1e9)
            .sum();
        pts.push_str(&format!("{:.1},{:.1} ", x_of(di), y_pay(day_total)));
    }
    svg.push_str(&format!(
        "<polyline points=\"{}\" fill=\"none\" stroke=\"#d95f18\" stroke-width=\"2\"/>\n",
        pts.trim_end()
    ));
    svg.push_str(&format!("<rect x=\"{:.0}\" y=\"{:.1}\" width=\"12\" height=\"12\" fill=\"#d95f18\"/>\n", margin_l, y2 + 26.0));
    svg.push_str(&format!("<text x=\"{:.0}\" y=\"{:.1}\" fill=\"#3a230d\">planetary total</text>\n", margin_l + 17.0, y2 + 37.0));

    svg.push_str("</svg>\n");
    svg
}

/// Hand-rolled SVG line charts, no dependencies. Two stacked panels:
/// coverage ratio per zone (reference line at 1.0), and daily converted
/// payout per zone.
fn build_svg(specs: &[ZoneSpec], grid: &[Vec<DayStats>], coverage: &[Vec<f64>], days: u64) -> String {
    let width = 960.0;
    let panel_h = 360.0;
    let margin_l = 70.0;
    let margin_r = 20.0;
    let margin_t = 34.0;
    let plot_h = panel_h - margin_t - 40.0;
    let total_h = panel_h * 2.0 + 20.0;

    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{0}\" height=\"{1}\" viewBox=\"0 0 {0} {1}\" font-family=\"system-ui, sans-serif\" font-size=\"12\">\n",
        width as u32, total_h as u32
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#faf7f1\"/>\n");

    let x_of = |day: usize| -> f64 {
        margin_l + (day as f64 / (days as f64 - 1.0).max(1.0)) * (width - margin_l - margin_r)
    };

    // Panel 1: coverage ratio.
    let y1 = 0.0;
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" font-size=\"15\" font-weight=\"600\">Coverage ratio per zone (R / (H x D-hat), Annex G.3)</text>\n",
        margin_l, y1 + 20.0
    ));
    let mut cov_max = 0.0f64;
    for row in coverage {
        for v in row {
            if v.is_finite() && *v > cov_max {
                cov_max = *v;
            }
        }
    }
    cov_max = (cov_max.max(1.0) * 1.1).min(4.0);
    let y_cov = |v: f64| -> f64 { y1 + panel_h - 40.0 - (v / cov_max).clamp(0.0, 1.0) * plot_h };
    // axes
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{t}\" x2=\"{m}\" y2=\"{b}\" stroke=\"#3a230d\" stroke-width=\"1\"/>\n",
        m = margin_l,
        t = y1 + margin_t,
        b = y1 + panel_h - 40.0
    ));
    for (label, val) in [("0", 0.0f64), ("1.0", 1.0f64), (&format!("{:.1}", cov_max), cov_max)] {
        let v = if val > cov_max { cov_max } else { val };
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">{}</text>\n",
            margin_l - 8.0,
            y_cov(v) + 4.0,
            label
        ));
    }
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" text-anchor=\"middle\">day</text>\n",
        width / 2.0,
        y1 + panel_h - 12.0
    ));
    // reference line at coverage = 1.0
    if cov_max >= 1.0 {
        svg.push_str(&format!(
            "<line x1=\"{m}\" y1=\"{y:.1}\" x2=\"{e:.1}\" y2=\"{y:.1}\" stroke=\"#3a230d\" stroke-dasharray=\"6 4\" stroke-width=\"1\" opacity=\"0.6\"/>\n",
            m = margin_l,
            y = y_cov(1.0),
            e = width - margin_r
        ));
    }
    for (zi, _s) in specs.iter().enumerate() {
        let mut pts = String::new();
        for (di, row) in coverage.iter().enumerate() {
            let v = row[zi];
            let v = if v.is_finite() { v.min(cov_max) } else { cov_max };
            pts.push_str(&format!("{:.1},{:.1} ", x_of(di), y_cov(v)));
        }
        svg.push_str(&format!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            pts.trim_end(),
            CHART_COLORS[zi % 4]
        ));
    }
    // legend
    let mut lx = margin_l;
    for (zi, s) in specs.iter().enumerate() {
        svg.push_str(&format!(
            "<rect x=\"{:.0}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\"/>\n",
            lx,
            y1 + 26.0,
            CHART_COLORS[zi % 4]
        ));
        svg.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{}\" fill=\"#3a230d\">{}</text>\n",
            lx + 17.0,
            y1 + 37.0,
            s.name
        ));
        lx += 17.0 + 14.0 * s.name.len() as f64;
    }

    // Panel 2: daily converted payout.
    let y2 = panel_h + 20.0;
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" font-size=\"15\" font-weight=\"600\">Daily converted payout per zone (USD cents)</text>\n",
        margin_l,
        y2 + 20.0
    ));
    let mut pay_max = 1.0f64;
    for row in grid {
        for cell in row {
            if cell.converted_cents > pay_max as i64 {
                pay_max = cell.converted_cents as f64;
            }
        }
    }
    pay_max *= 1.1;
    let y_pay = |v: f64| -> f64 { y2 + panel_h - 40.0 - (v / pay_max).clamp(0.0, 1.0) * plot_h };
    svg.push_str(&format!(
        "<line x1=\"{m}\" y1=\"{t}\" x2=\"{m}\" y2=\"{b}\" stroke=\"#3a230d\" stroke-width=\"1\"/>\n",
        m = margin_l,
        t = y2 + margin_t,
        b = y2 + panel_h - 40.0
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">0</text>\n",
        margin_l - 8.0,
        y_pay(0.0) + 4.0
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{:.1}\" fill=\"#3a230d\" text-anchor=\"end\">{:.0}c</text>\n",
        margin_l - 8.0,
        y_pay(pay_max / 1.1) + 4.0,
        pay_max / 1.1
    ));
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" fill=\"#3a230d\" text-anchor=\"middle\">day</text>\n",
        width / 2.0,
        y2 + panel_h - 12.0
    ));
    for (zi, _s) in specs.iter().enumerate() {
        let mut pts = String::new();
        for (di, row) in grid.iter().enumerate() {
            pts.push_str(&format!(
                "{:.1},{:.1} ",
                x_of(di),
                y_pay(row[zi].converted_cents as f64)
            ));
        }
        svg.push_str(&format!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/>\n",
            pts.trim_end(),
            CHART_COLORS[zi % 4]
        ));
    }
    let mut lx = margin_l;
    for (zi, s) in specs.iter().enumerate() {
        svg.push_str(&format!(
            "<rect x=\"{:.0}\" y=\"{}\" width=\"12\" height=\"12\" fill=\"{}\"/>\n",
            lx,
            y2 + 26.0,
            CHART_COLORS[zi % 4]
        ));
        svg.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{}\" fill=\"#3a230d\">{}</text>\n",
            lx + 17.0,
            y2 + 37.0,
            s.name
        ));
        lx += 17.0 + 14.0 * s.name.len() as f64;
    }

    svg.push_str("</svg>\n");
    svg
}
